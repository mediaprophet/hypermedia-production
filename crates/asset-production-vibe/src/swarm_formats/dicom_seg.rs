//! Native binary and fractional DICOM SEG for synthetic, axis-aligned phantoms.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use qualia_core_db::{
    container_10d::{
        integrity::check_whole_file_crc32c, sdf_brick_section::read_container_sdfb_with_iris,
    },
    medical::dicom_dataset::{ItemId, ParseLimits, Part10Dataset, WorkBudget},
};
use serde_json::json;
use sha2::{Digest, Sha256};

use super::{encode_json, receipt, result, sha256};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::FieldLattice, boolean, fields,
    qualia_containers_governance_poet_ops::encoded_resource, required, text,
};

const SEG_SOP: &str = "1.2.840.10008.5.1.4.1.1.66.4";
const EXPLICIT_LE: &str = "1.2.840.10008.1.2.1";
const PHANTOM_ID: &str = "QUALIA_SYNTHETIC_PHANTOM";
const CONTENT_LABEL: &str = "SYNTHETIC_SEG";
const MAX_DCM_BYTES: usize = 2 * 1024 * 1024;
const MAX_INPUT_10D: usize = 16 * 1024 * 1024;
const MAX_AXES: [u32; 3] = [256, 256, 128];
const MAX_VOXELS: usize = 4 * 1024 * 1024;
const MAX_ORIGIN_UM: i64 = 1_000_000_000;
const MAX_SPACING_UM: i64 = 20_000;
const FIXTURE_DA: &str = "20000101";
const FIXTURE_TM: &str = "000000";

#[derive(Clone, Copy, PartialEq, Eq)]
enum SegKind {
    Binary,
    FractionalOccupancy,
}

impl SegKind {
    fn dicom_name(self) -> &'static str {
        match self {
            Self::Binary => "BINARY",
            Self::FractionalOccupancy => "FRACTIONAL",
        }
    }

    fn bit_depth(self) -> u16 {
        match self {
            Self::Binary => 1,
            Self::FractionalOccupancy => 8,
        }
    }

    fn receipt_iri(self) -> &'static str {
        match self {
            Self::Binary => "urn:qualia:format:synthetic-binary-dicom-seg",
            Self::FractionalOccupancy => "urn:qualia:format:synthetic-fractional-dicom-seg",
        }
    }

    fn lineage(self) -> &'static str {
        match self {
            Self::Binary => "qualia.synthetic.binary-seg/1",
            Self::FractionalOccupancy => "qualia.synthetic.fractional-occupancy-seg/1",
        }
    }

    fn pixel_bytes(self, count: usize) -> usize {
        match self {
            Self::Binary => count.div_ceil(8),
            Self::FractionalOccupancy => count,
        }
        .next_multiple_of(2)
    }
}

struct SyntheticSeg {
    kind: SegKind,
    axes: [u32; 3],
    spacing_um: [i64; 3],
    origin_um: [i64; 3],
    pixels: Vec<u8>,
    foreground: usize,
}

impl AssetHost<'_> {
    pub(crate) fn export_dicom_seg_rtstruct(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "synthetic_phantom",
                "format",
                "mask",
                "source_10d",
                "spacing_um",
                "origin_um",
            ],
        )?;
        let format = args
            .get("format")
            .map(text)
            .transpose()?
            .unwrap_or("binary_seg");
        if !boolean(args, "synthetic_phantom", false)? {
            return Err("SEG export requires an authored synthetic phantom".into());
        }
        let kind = match format {
            "binary_seg" => SegKind::Binary,
            "fractional_seg" => SegKind::FractionalOccupancy,
            _ => return Err("Synthetic SEG export supports binary_seg or fractional_seg".into()),
        };
        let (seg, source_hash, quantization_error) =
            match (args.get("mask"), args.get("source_10d")) {
                (Some(mask), None) => self.synthetic_dense_seg(args, mask, kind)?,
                (None, Some(source)) => {
                    if kind != SegKind::Binary {
                        return Err(
                            "Fractional SEG requires explicitly authored occupancy samples in mask"
                                .into(),
                        );
                    }
                    if args.contains_key("spacing_um") || args.contains_key("origin_um") {
                        return Err("SDFB export uses its own grid origin and spacing".into());
                    }
                    let (seg, hash) = self.synthetic_sdfb_seg(source)?;
                    (seg, hash, None)
                }
                _ => return Err("Supply exactly one synthetic mask or source_10d".into()),
            };
        if seg.foreground == 0 {
            return Err("The synthetic SEG must have a nonempty segment".into());
        }
        let bytes = seg.encode();
        if bytes.len() > self.limits.resource_bytes.min(MAX_DCM_BYTES) {
            return Err("DICOM SEG exceeds the admitted output size".into());
        }
        let hash = sha256(&bytes);
        let mut metadata = seg.report(&hash);
        if let Some(max_error) = quantization_error {
            metadata["max_absolute_occupancy_quantization_error"] = json!(max_error);
        }
        let report = encode_json(self, &metadata)?;
        let receipt = receipt(self, kind.receipt_iri(), &source_hash, &hash)?;
        let seg = self.encode("dcm", bytes)?;
        Ok(result(&[
            ("seg", seg),
            ("report", report),
            ("receipt", receipt),
        ]))
    }

    fn synthetic_dense_seg(
        &mut self,
        args: &Args,
        mask: &Value,
        kind: SegKind,
    ) -> Result<(SyntheticSeg, String, Option<f64>), String> {
        let (axes, values) = dense_mask(self, mask)?;
        let count = values.len();
        let spacing = micrometers(required(args, "spacing_um")?)?;
        let origin = args
            .get("origin_um")
            .map(micrometers)
            .transpose()?
            .unwrap_or([0; 3]);
        check_geometry(axes, spacing, origin)?;
        let bytes = kind.pixel_bytes(count);
        if bytes >= self.limits.resource_bytes.min(MAX_DCM_BYTES) {
            return Err("Synthetic SEG pixel bytes exceed the admitted output size".into());
        }
        self.charge(count * 12, bytes * 2 + MAX_DCM_BYTES)?;
        let (_, values) = dense_mask(self, mask)?;
        let mut seg = SyntheticSeg::new(axes, spacing, origin, kind)?;
        let mut max_error = 0.0_f64;
        let mut source_digest = seg.geometry_digest();
        for (index, sample) in values.iter().copied().enumerate() {
            let encoded = match kind {
                SegKind::Binary if sample == 0.0 || sample == 1.0 => sample as u8,
                SegKind::Binary => {
                    return Err("Synthetic binary SEG mask requires exact 0 or 1".into());
                }
                SegKind::FractionalOccupancy if (0.0..=1.0).contains(&sample) => {
                    source_digest.update(sample.to_le_bytes());
                    let encoded = (f64::from(sample) * 255.0).round() as u8;
                    max_error =
                        max_error.max((f64::from(sample) - f64::from(encoded) / 255.0).abs());
                    encoded
                }
                SegKind::FractionalOccupancy => {
                    return Err("Fractional occupancy SEG requires finite values in [0, 1]".into());
                }
            };
            seg.set_code(index, encoded);
        }
        let hash = match kind {
            SegKind::Binary => seg.digest(),
            SegKind::FractionalOccupancy => format!("{:x}", source_digest.finalize()),
        };
        let error = (kind == SegKind::FractionalOccupancy).then_some(max_error);
        Ok((seg, hash, error))
    }

    fn synthetic_sdfb_seg(&mut self, source: &Value) -> Result<(SyntheticSeg, String), String> {
        let input = encoded_resource(self, source)?;
        if input.kind != "10d" || input.bytes.len() > self.limits.resource_bytes.min(MAX_INPUT_10D)
        {
            return Err("Synthetic SEG requires bounded 10D source bytes".into());
        }
        let input_bytes = input.bytes.len();
        self.charge(input_bytes.saturating_mul(6), input_bytes + MAX_DCM_BYTES)?;
        let (axes, spacing_um, origin_um) = {
            let source = &encoded_resource(self, source)?.bytes;
            check_whole_file_crc32c(source).map_err(|error| format!("10D integrity: {error:?}"))?;
            let (view, _) =
                read_container_sdfb_with_iris(source)?.ok_or("10D source lacks semantic SDFB")?;
            let grid = view.grid();
            if grid
                .base_bricks
                .iter()
                .zip(MAX_AXES)
                .any(|(&n, cap)| n > cap / 8)
            {
                return Err("SDFB grid exceeds the synthetic SEG voxel bounds".into());
            }
            let axes = grid.base_bricks.map(|n| n * 8);
            let spacing_um = grid
                .voxel_spacing_m
                .map(|n| (f64::from(n) * 1_000_000.0).round() as i64);
            let origin_um = grid
                .origin_m
                .map(|n| (f64::from(n) * 1_000_000.0).round() as i64);
            check_geometry(axes, spacing_um, origin_um)?;
            (axes, spacing_um, origin_um)
        };
        let count = count_voxels(axes)?;
        self.charge(count * 6, count / 2 + input_bytes + MAX_DCM_BYTES)?;
        let source = &encoded_resource(self, source)?.bytes;
        let (view, lexicon) =
            read_container_sdfb_with_iris(source)?.ok_or("10D source lacks semantic SDFB")?;
        if view.has_openvdb_active_masks()
            || lexicon.len() != 2
            || lexicon
                .values()
                .any(|iri| !iri.starts_with("urn:qualia:phantom:"))
        {
            return Err("SEG from 10D requires one synthetic wall/lumen pair identified by urn:qualia:phantom: IRIs".into());
        }
        let mut seg = SyntheticSeg::new(axes, spacing_um, origin_um, SegKind::Binary)?;
        let xy_stride = axes[0] as usize * axes[1] as usize;
        for brick in view.bricks() {
            let key = brick.key();
            if key.refinement_level != 0 {
                return Err("Synthetic SEG accepts only a uniform SDFB base lattice".into());
            }
            let [bx, by, bz] = key.coordinate.map(|n| n as usize * 8);
            for i in 0..512 {
                if brick.occupied(i) != Some(true) {
                    continue;
                }
                let x = bx + (i % 8);
                let y = by + (i / 8 % 8);
                let z = bz + (i / 64);
                seg.set_code(x + y * axes[0] as usize + z * xy_stride, 1);
            }
        }
        Ok((seg, sha256(source)))
    }

    pub(super) fn import_synthetic_dicom_seg(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["modality", "source", "synthetic_phantom"])?;
        if !boolean(args, "synthetic_phantom", false)? {
            return Err("The DICOM SEG reader requires the synthetic phantom profile".into());
        }
        let source = required(args, "source")?;
        let input = encoded_resource(self, source)?;
        let len = input.bytes.len();
        if !matches!(input.kind.as_str(), "dcm" | "dicom")
            || len > self.limits.resource_bytes.min(MAX_DCM_BYTES)
        {
            return Err("Supply bounded native DICOM SEG bytes from the synthetic exporter".into());
        }
        self.charge(len * 32, len.saturating_mul(4) + 256 * 1024)?;
        let input = &encoded_resource(self, source)?.bytes;
        let expect = match text(required(args, "modality")?)? {
            "synthetic_binary_seg" => SegKind::Binary,
            "synthetic_fractional_seg" => SegKind::FractionalOccupancy,
            _ => return Err("Unknown synthetic SEG readback modality".into()),
        };
        let seg = decode(input, expect)?;
        let source_hash = sha256(input);
        let count = count_voxels(seg.axes)?;
        self.charge(count * 7, count * 4 + MAX_DCM_BYTES)?;
        let output_hash = seg.digest();
        let samples = (0..count).map(|i| seg.sample(i)).collect();
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: seg.axes.to_vec(),
                samples,
            },
        )))?;
        let report = encode_json(self, &seg.report(&source_hash))?;
        let receipt = receipt(self, seg.kind.receipt_iri(), &source_hash, &output_hash)?;
        Ok(result(&[
            ("field", field),
            ("report", report),
            ("receipt", receipt),
        ]))
    }
}

fn dense_mask<'a>(host: &'a AssetHost<'_>, mask: &Value) -> Result<([u32; 3], &'a [f32]), String> {
    let TypedArtifact::FieldLattice(field) = host.workspace.typed(mask)? else {
        return Err("Expected a typed synthetic mask FieldLattice".into());
    };
    let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
        return Err("Expected a dense F32 synthetic binary mask".into());
    };
    let axes: [u32; 3] = axes
        .as_slice()
        .try_into()
        .map_err(|_| "Expected XYZ mask axes")?;
    if count_voxels(axes)? != samples.len() {
        return Err("Synthetic SEG sample length differs from its lattice axes".into());
    }
    Ok((axes, samples))
}

fn micrometers(value: &Value) -> Result<[i64; 3], String> {
    let [x, y, z] = crate::list(value)? else {
        return Err("Expected an XYZ micrometer triple".into());
    };
    let read = |value: &Value| -> Result<i64, String> {
        match value {
            Value::I64(value) => Ok(*value),
            Value::U64(value) => {
                i64::try_from(*value).map_err(|_| "Micrometer coordinate out of range".into())
            }
            _ => Err("Micrometer coordinates require integers".into()),
        }
    };
    Ok([read(x)?, read(y)?, read(z)?])
}

fn count_voxels(axes: [u32; 3]) -> Result<usize, String> {
    if axes
        .into_iter()
        .zip(MAX_AXES)
        .any(|(n, max)| n == 0 || n > max)
    {
        return Err("Synthetic SEG axes exceed the admitted extent".into());
    }
    let count = axes.into_iter().map(|n| n as usize).product();
    if count > MAX_VOXELS {
        return Err("Synthetic SEG exceeds the admitted voxel count".into());
    }
    Ok(count)
}

fn check_geometry(axes: [u32; 3], spacing: [i64; 3], origin: [i64; 3]) -> Result<(), String> {
    count_voxels(axes)?;
    if spacing
        .into_iter()
        .any(|n| !(1..=MAX_SPACING_UM).contains(&n))
        || spacing[2] < 50
        || origin
            .into_iter()
            .any(|n| !(-MAX_ORIGIN_UM..=MAX_ORIGIN_UM).contains(&n))
    {
        return Err(
            "Synthetic SEG spatial coordinates exceed the admitted micrometer range".into(),
        );
    }
    Ok(())
}

impl SyntheticSeg {
    fn new(
        axes: [u32; 3],
        spacing_um: [i64; 3],
        origin_um: [i64; 3],
        kind: SegKind,
    ) -> Result<Self, String> {
        check_geometry(axes, spacing_um, origin_um)?;
        let pixels = vec![0; kind.pixel_bytes(count_voxels(axes)?)];
        Ok(Self {
            kind,
            axes,
            spacing_um,
            origin_um,
            pixels,
            foreground: 0,
        })
    }

    fn set_code(&mut self, i: usize, value: u8) {
        if value == 0 {
            return;
        }
        match self.kind {
            SegKind::Binary => self.pixels[i / 8] |= 1 << (i % 8),
            SegKind::FractionalOccupancy => self.pixels[i] = value,
        }
        self.foreground += 1;
    }

    fn sample(&self, i: usize) -> f32 {
        match self.kind {
            SegKind::Binary => f32::from((self.pixels[i / 8] >> (i % 8)) & 1),
            SegKind::FractionalOccupancy => f32::from(self.pixels[i]) / 255.0,
        }
    }

    fn geometry_digest(&self) -> Sha256 {
        let mut hash = Sha256::new();
        hash.update(self.kind.lineage().as_bytes());
        for n in self.axes {
            hash.update(n.to_le_bytes());
        }
        for n in self.spacing_um.into_iter().chain(self.origin_um) {
            hash.update(n.to_le_bytes());
        }
        hash
    }

    fn digest(&self) -> String {
        let mut hash = self.geometry_digest();
        hash.update(&self.pixels);
        format!("{:x}", hash.finalize())
    }

    fn report(&self, hash: &str) -> serde_json::Value {
        let mut report = json!({
            "schema":"qualia-synthetic-dicom-seg/1", "sop_class_uid":SEG_SOP,
            "transfer_syntax_uid":EXPLICIT_LE, "segmentation_type":self.kind.dicom_name(), "synthetic_phantom":true,
            "axes_xyz":self.axes, "spacing_xyz_um":self.spacing_um, "first_pixel_center_xyz_um":self.origin_um,
            "coordinate_system":"LPS",
            "segment_number":1, "foreground_voxels":self.foreground, "dicom_sha256":hash,
            "lattice_sha256":self.digest(),
        });
        if self.kind == SegKind::FractionalOccupancy {
            report["fractional_type"] = json!("OCCUPANCY");
            report["maximum_fractional_value"] = json!(255);
        }
        report
    }

    fn encode(&self) -> Vec<u8> {
        let identity = self.digest();
        let instance = uid(&identity, b"sop");
        let study = uid(&identity, b"study");
        let series = uid(&identity, b"series");
        let frame = uid(&identity, b"frame");
        let dimension = uid(&identity, b"dimension");
        let implementation = uid("qualia.synthetic.binary-seg/1", b"implementation");
        let meta = item(|d| {
            d.element(0x00020001, b"OB", &[0, 1]);
            d.text(0x00020002, b"UI", SEG_SOP);
            d.text(0x00020003, b"UI", &instance);
            d.text(0x00020010, b"UI", EXPLICIT_LE);
            d.text(0x00020012, b"UI", &implementation);
            d.text(0x00020013, b"SH", "QUALIA_SEG_1");
        });
        let mut file = Dataset(vec![0; 128]);
        file.0.extend_from_slice(b"DICM");
        file.ul(0x00020000, meta.0.len() as u32);
        file.0.extend(meta.0);

        file.text(0x00080008, b"CS", "DERIVED\\PRIMARY");
        file.text(0x00080016, b"UI", SEG_SOP);
        file.text(0x00080018, b"UI", &instance);
        file.text(0x00080020, b"DA", "");
        file.text(0x00080023, b"DA", FIXTURE_DA);
        file.text(0x00080030, b"TM", "");
        file.text(0x00080033, b"TM", FIXTURE_TM);
        file.text(0x00080050, b"SH", "");
        file.text(0x00080060, b"CS", "SEG");
        file.text(0x00080070, b"LO", "Qualia");
        file.text(0x00080090, b"PN", "");
        file.text(
            0x0008103E,
            b"LO",
            match self.kind {
                SegKind::Binary => "Synthetic binary segmentation fixture",
                SegKind::FractionalOccupancy => "Synthetic fractional occupancy fixture",
            },
        );
        file.text(0x00081090, b"LO", "Hypermedia Production");
        file.text(0x00100010, b"PN", "");
        file.text(0x00100020, b"LO", PHANTOM_ID);
        file.text(0x00100030, b"DA", "");
        file.text(0x00100040, b"CS", "");
        file.text(0x00100200, b"CS", "YES");
        file.text(0x00181000, b"LO", "QUALIA_SYNTHETIC");
        file.text(0x00181020, b"LO", "1");
        file.text(0x00189004, b"CS", "RESEARCH");
        file.text(0x0020000D, b"UI", &study);
        file.text(0x0020000E, b"UI", &series);
        file.text(0x00200010, b"SH", "");
        file.text(0x00200011, b"IS", "1");
        file.text(0x00200013, b"IS", "1");
        file.text(0x00200020, b"CS", "L\\P");
        file.text(0x00200052, b"UI", &frame);
        file.text(0x00201040, b"LO", "");
        file.sequence(
            0x00209221,
            [item(|d| d.text(0x00209164, b"UI", &dimension))],
        );
        file.sequence(
            0x00209222,
            [
                dimension_index(&dimension, 0x0062000B, 0x0062000A, "Segment"),
                dimension_index(&dimension, 0x00200032, 0x00209113, "Slice position"),
            ],
        );
        file.text(0x00209311, b"CS", "3D");
        file.us(0x00280002, 1);
        file.text(0x00280004, b"CS", "MONOCHROME2");
        file.text(0x00280008, b"IS", &self.axes[2].to_string());
        file.us(0x00280010, self.axes[1] as u16);
        file.us(0x00280011, self.axes[0] as u16);
        let bits = self.kind.bit_depth();
        file.us(0x00280100, bits);
        file.us(0x00280101, bits);
        file.us(0x00280102, bits - 1);
        file.us(0x00280103, 0);
        file.text(0x00280300, b"CS", "YES");
        file.text(0x00280301, b"CS", "NO");
        file.text(0x00282110, b"CS", "00");
        file.text(0x00620001, b"CS", self.kind.dicom_name());
        file.sequence(
            0x00620002,
            [item(|d| {
                d.sequence(0x00620003, [code("260787004", "Physical object")]);
                d.us(0x00620004, 1);
                d.text(0x00620005, b"LO", "Synthetic phantom");
                d.text(0x00620008, b"CS", "AUTOMATIC");
                d.text(
                    0x00620009,
                    b"LO",
                    match self.kind {
                        SegKind::Binary => "Qualia binary mask export",
                        SegKind::FractionalOccupancy => "Qualia fractional occupancy export",
                    },
                );
                d.sequence(0x0062000F, [code("706342009", "Phantom")]);
            })],
        );
        if self.kind == SegKind::FractionalOccupancy {
            file.us(0x0062000E, 255);
            file.text(0x00620010, b"CS", "OCCUPANCY");
        }
        file.text(0x00620013, b"CS", "NO");
        file.text(0x00700080, b"CS", CONTENT_LABEL);
        file.text(0x00700081, b"LO", "Authored synthetic fixture");
        file.text(0x00700084, b"PN", "");
        file.sequence(
            0x52009229,
            [item(|shared| {
                shared.sequence(
                    0x00209116,
                    [item(|d| d.text(0x00200037, b"DS", "1\\0\\0\\0\\1\\0"))],
                );
                shared.sequence(
                    0x00289110,
                    [item(|d| {
                        d.text(0x00180050, b"DS", &mm(self.spacing_um[2]));
                        d.text(0x00180088, b"DS", &mm(self.spacing_um[2]));
                        d.text(
                            0x00280030,
                            b"DS",
                            &format!("{}\\{}", mm(self.spacing_um[1]), mm(self.spacing_um[0])),
                        );
                    })],
                );
            })],
        );
        file.sequence(
            0x52009230,
            (0..self.axes[2]).map(|z| {
                item(|d| {
                    d.sequence(
                        0x00209111,
                        [item(|content| {
                            let mut ordinal = 1u32.to_le_bytes().to_vec();
                            ordinal.extend_from_slice(&(z + 1).to_le_bytes());
                            content.element(0x00209157, b"UL", &ordinal);
                        })],
                    );
                    d.sequence(
                        0x00209113,
                        [item(|position| {
                            let [x, y, first_z] = self.origin_um;
                            position.text(
                                0x00200032,
                                b"DS",
                                &format!(
                                    "{}\\{}\\{}",
                                    mm(x),
                                    mm(y),
                                    mm(first_z + i64::from(z) * self.spacing_um[2])
                                ),
                            );
                        })],
                    );
                    d.sequence(0x0062000A, [item(|segment| segment.us(0x0062000B, 1))]);
                })
            }),
        );
        file.element(0x7FE00010, b"OB", &self.pixels);
        file.0
    }
}

fn uid(seed: &str, role: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(seed.as_bytes());
    hash.update(role);
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&hash.finalize()[..16]);
    uuid[6] = (uuid[6] & 0x0F) | 0x80;
    uuid[8] = (uuid[8] & 0x3F) | 0x80;
    format!("2.25.{}", u128::from_be_bytes(uuid))
}

fn mm(um: i64) -> String {
    let magnitude = um.unsigned_abs();
    let sign = if um < 0 { "-" } else { "" };
    format!("{sign}{}.{:03}", magnitude / 1000, magnitude % 1000)
}

fn tag_bytes(tag: u32) -> [u8; 4] {
    let [group_hi, group_lo, element_hi, element_lo] = tag.to_be_bytes();
    [group_lo, group_hi, element_lo, element_hi]
}

struct Dataset(Vec<u8>);

impl Dataset {
    fn element(&mut self, tag: u32, vr: &[u8; 2], data: &[u8]) {
        self.0.extend_from_slice(&tag_bytes(tag));
        self.0.extend_from_slice(vr);
        let len = data.len().next_multiple_of(2);
        if matches!(vr, b"OB" | b"SQ") {
            self.0.extend_from_slice(&[0, 0]);
            self.0.extend_from_slice(&(len as u32).to_le_bytes());
        } else {
            self.0.extend_from_slice(&(len as u16).to_le_bytes());
        }
        self.0.extend_from_slice(data);
        if data.len() != len {
            self.0
                .push(if matches!(vr, b"OB" | b"UI") { 0 } else { b' ' });
        }
    }

    fn text(&mut self, tag: u32, vr: &[u8; 2], value: &str) {
        self.element(tag, vr, value.as_bytes());
    }

    fn us(&mut self, tag: u32, value: u16) {
        self.element(tag, b"US", &value.to_le_bytes());
    }

    fn ul(&mut self, tag: u32, value: u32) {
        self.element(tag, b"UL", &value.to_le_bytes());
    }

    fn sequence(&mut self, tag: u32, items: impl IntoIterator<Item = Dataset>) {
        let mut data = Vec::new();
        for item in items {
            data.extend_from_slice(&tag_bytes(0xFFFEE000));
            data.extend_from_slice(&(item.0.len() as u32).to_le_bytes());
            data.extend(item.0);
        }
        self.element(tag, b"SQ", &data);
    }
}

fn item(fill: impl FnOnce(&mut Dataset)) -> Dataset {
    let mut data = Dataset(Vec::new());
    fill(&mut data);
    data
}

fn code(value: &str, meaning: &str) -> Dataset {
    item(|d| {
        d.text(0x00080100, b"SH", value);
        d.text(0x00080102, b"SH", "SCT");
        d.text(0x00080104, b"LO", meaning);
    })
}

fn dimension_index(uid: &str, field: u32, sequence: u32, title: &str) -> Dataset {
    item(|d| {
        d.text(0x00209164, b"UI", uid);
        d.element(0x00209165, b"AT", &tag_bytes(field));
        d.element(0x00209167, b"AT", &tag_bytes(sequence));
        d.text(0x00209421, b"LO", title);
    })
}

fn decode(bytes: &[u8], expect: SegKind) -> Result<SyntheticSeg, String> {
    let limits = ParseLimits {
        max_bytes: MAX_DCM_BYTES,
        max_elements: 2048,
        max_items: 1024,
        max_depth: 8,
        max_index_bytes: 256 * 1024,
        max_work: 16_384,
    };
    let file = Part10Dataset::parse(bytes, limits).map_err(|error| error.to_string())?;
    let mut work = WorkBudget::new(64 * 1024);
    if file.transfer_syntax_uid() != EXPLICIT_LE
        || file.media_sop_class_uid() != Some(SEG_SOP)
        || file
            .text(None, 0x00620001, *b"CS", &mut work)
            .map_err(|e| e.to_string())?
            != Some(expect.dicom_name())
        || file
            .text(None, 0x00100020, *b"LO", &mut work)
            .map_err(|e| e.to_string())?
            != Some(PHANTOM_ID)
        || file
            .text(None, 0x00700080, *b"CS", &mut work)
            .map_err(|e| e.to_string())?
            != Some(CONTENT_LABEL)
    {
        return Err("Expected a native SEG produced by the synthetic fixture writer".into());
    }
    let axes = [
        u32::from(
            file.unsigned_short(0x00280011, &mut work)
                .map_err(|e| e.to_string())?
                .ok_or("Missing SEG columns")?,
        ),
        u32::from(
            file.unsigned_short(0x00280010, &mut work)
                .map_err(|e| e.to_string())?
                .ok_or("Missing SEG rows")?,
        ),
        file.frame_count(&mut work).map_err(|e| e.to_string())?,
    ];
    let shared = first_item(&file, None, 0x52009229, &mut work)?;
    let metrics = first_item(&file, Some(shared), 0x00289110, &mut work)?;
    let ds = |item, tag, work: &mut WorkBudget| {
        file.text(Some(item), tag, *b"DS", work)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Missing SEG spatial attribute".to_string())
    };
    let [row, column] = parse_ds(ds(metrics, 0x00280030, &mut work)?)?;
    let [depth] = parse_ds(ds(metrics, 0x00180088, &mut work)?)?;
    let first = first_item(&file, None, 0x52009230, &mut work)?;
    let position = first_item(&file, Some(first), 0x00209113, &mut work)?;
    let origin = parse_ds(ds(position, 0x00200032, &mut work)?)?;
    let mut seg = SyntheticSeg::new(axes, [column, row, depth], origin, expect)?;
    let pixel_bytes = file
        .expected_value(None, 0x7FE00010, *b"OB", &mut work)
        .map_err(|e| e.to_string())?
        .ok_or("Missing native packed SEG pixels")?;
    if pixel_bytes.len() != seg.pixels.len() {
        return Err("Packed SEG byte extent does not match declared dimensions".into());
    }
    let count = count_voxels(axes)?;
    match expect {
        SegKind::Binary => {
            for i in 0..count {
                seg.set_code(i, (pixel_bytes[i / 8] >> (i % 8)) & 1);
            }
        }
        SegKind::FractionalOccupancy => {
            for (i, &code) in pixel_bytes[..count].iter().enumerate() {
                seg.set_code(i, code);
            }
        }
    }
    if seg.foreground == 0 || seg.encode().as_slice() != bytes {
        return Err(
            "SEG is outside the synthetic writer's canonical bytes, tags, or frame geometry".into(),
        );
    }
    Ok(seg)
}

fn first_item(
    file: &Part10Dataset<'_>,
    scope: Option<ItemId>,
    tag: u32,
    work: &mut WorkBudget,
) -> Result<ItemId, String> {
    let seq = file
        .find(scope, tag, work)
        .map_err(|e| e.to_string())?
        .ok_or("Missing SEG sequence")?;
    file.sequence_items(seq, work)
        .map_err(|e| e.to_string())?
        .next()
        .ok_or_else(|| "Empty SEG sequence".into())
}

fn parse_ds<const N: usize>(text: &str) -> Result<[i64; N], String> {
    let mut coordinates = text.split('\\');
    let mut result = [0; N];
    for number in &mut result {
        let raw = coordinates
            .next()
            .ok_or("Missing SEG Decimal String component")?;
        let value = raw
            .parse::<f64>()
            .map_err(|_| "Invalid SEG Decimal String")?
            * 1000.0;
        if raw.len() > 16 || !value.is_finite() || value.abs() > MAX_ORIGIN_UM as f64 {
            return Err("SEG Decimal String component exceeds the spatial profile".into());
        }
        *number = value.round() as i64;
    }
    if coordinates.next().is_some() {
        return Err("Excess SEG Decimal String components".into());
    }
    Ok(result)
}
