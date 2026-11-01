//! NAL unit header and slice header parsing (7.3.1, 7.3.3).

use crate::error::{Result, ensure};
use crate::params::{Pps, Sps};
use bitstream_codec::BitReader;

/// NAL unit types used by the decoder.
pub mod nal_type {
    pub const SLICE: u8 = 1;
    pub const SLICE_DPA: u8 = 2;
    pub const SLICE_DPB: u8 = 3;
    pub const SLICE_DPC: u8 = 4;
    pub const IDR: u8 = 5;
    pub const SEI: u8 = 6;
    pub const SPS: u8 = 7;
    pub const PPS: u8 = 8;
    pub const AUD: u8 = 9;
    pub const END_SEQ: u8 = 10;
    pub const END_STREAM: u8 = 11;
    pub const FILLER: u8 = 12;
    pub const SPS_EXT: u8 = 13;
    pub const PREFIX: u8 = 14;
    pub const SUBSET_SPS: u8 = 15;
    pub const AUX_SLICE: u8 = 19;
    pub const SLICE_EXT: u8 = 20;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NalHeader {
    pub nal_ref_idc: u8,
    pub nal_unit_type: u8,
}

impl NalHeader {
    pub fn parse(b: u8) -> Result<Self> {
        ensure!(b & 0x80 == 0, "forbidden_zero_bit set");
        Ok(Self { nal_ref_idc: (b >> 5) & 3, nal_unit_type: b & 0x1f })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliceType {
    P,
    B,
    I,
    Sp,
    Si,
}

impl SliceType {
    fn from_raw(v: u32) -> Result<Self> {
        Ok(match v % 5 {
            0 => SliceType::P,
            1 => SliceType::B,
            2 => SliceType::I,
            3 => SliceType::Sp,
            _ => SliceType::Si,
        })
    }
    pub fn is_intra(self) -> bool {
        matches!(self, SliceType::I | SliceType::Si)
    }
    pub fn is_b(self) -> bool {
        self == SliceType::B
    }
    pub fn is_p(self) -> bool {
        matches!(self, SliceType::P | SliceType::Sp)
    }
}

/// One entry of ref_pic_list_modification().
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefPicListMod {
    pub idc: u32,
    /// abs_diff_pic_num_minus1 or long_term_pic_num.
    pub value: u32,
}

/// memory_management_control_operation entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mmco {
    pub op: u32,
    pub difference_of_pic_nums_minus1: u32,
    pub long_term_pic_num: u32,
    pub long_term_frame_idx: u32,
    pub max_long_term_frame_idx_plus1: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WeightEntry {
    pub luma_weight: i32,
    pub luma_offset: i32,
    pub luma_flag: bool,
    pub chroma_weight: [i32; 2],
    pub chroma_offset: [i32; 2],
    pub chroma_flag: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredWeightTable {
    pub luma_log2_denom: u32,
    pub chroma_log2_denom: u32,
    pub l0: Vec<WeightEntry>,
    pub l1: Vec<WeightEntry>,
}

#[derive(Clone, Debug)]
pub struct SliceHeader {
    pub nal_ref_idc: u8,
    pub nal_unit_type: u8,
    pub idr: bool,
    pub first_mb_in_slice: u32,
    pub slice_type_raw: u32,
    pub slice_type: SliceType,
    pub pps_id: u32,
    pub colour_plane_id: u32,
    pub frame_num: u32,
    pub field_pic: bool,
    pub bottom_field: bool,
    pub idr_pic_id: u32,
    pub pic_order_cnt_lsb: u32,
    pub delta_pic_order_cnt_bottom: i32,
    pub delta_pic_order_cnt: [i32; 2],
    pub redundant_pic_cnt: u32,
    pub direct_spatial_mv_pred: bool,
    pub num_ref_idx_active: [u32; 2],
    pub ref_pic_list_mod: [Vec<RefPicListMod>; 2],
    pub pred_weight_table: Option<PredWeightTable>,
    pub no_output_of_prior_pics: bool,
    pub long_term_reference: bool,
    pub adaptive_ref_pic_marking: bool,
    pub mmcos: Vec<Mmco>,
    pub cabac_init_idc: u32,
    pub slice_qp_delta: i32,
    pub sp_for_switch: bool,
    pub slice_qs_delta: i32,
    pub disable_deblocking_filter_idc: u32,
    pub slice_alpha_c0_offset_div2: i32,
    pub slice_beta_offset_div2: i32,
    pub slice_group_change_cycle: u32,
    /// Bit offset of slice_data() within the RBSP.
    pub header_bits: usize,
}

impl SliceHeader {
    pub fn qp(&self, pps: &Pps) -> i32 {
        pps.pic_init_qp + self.slice_qp_delta
    }
    pub fn has_mmco5(&self) -> bool {
        self.mmcos.iter().any(|m| m.op == 5)
    }

    /// Parse the slice header. `pps_lookup` resolves pic_parameter_set_id to (PPS, SPS).
    pub fn parse<'a>(rbsp: &[u8], nal: NalHeader, pps_lookup: impl FnOnce(u32) -> Result<(&'a Pps, &'a Sps)>) -> Result<(SliceHeader, &'a Pps, &'a Sps)> {
        let mut r = BitReader::new(rbsp);
        let idr = nal.nal_unit_type == nal_type::IDR;
        let first_mb_in_slice = r.read_ue()?;
        let slice_type_raw = r.read_ue()?;
        ensure!(slice_type_raw <= 9, "slice_type out of range");
        let slice_type = SliceType::from_raw(slice_type_raw)?;
        let pps_id = r.read_ue()?;
        ensure!(pps_id < 256, "pic_parameter_set_id out of range");
        let (pps, sps) = pps_lookup(pps_id)?;
        let mut colour_plane_id = 0;
        if sps.separate_colour_plane {
            colour_plane_id = r.read_bits(2)?;
        }
        let frame_num = r.read_bits(sps.log2_max_frame_num)?;
        let mut field_pic = false;
        let mut bottom_field = false;
        if !sps.frame_mbs_only {
            field_pic = r.read_flag()?;
            if field_pic {
                bottom_field = r.read_flag()?;
            }
        }
        let idr_pic_id = if idr { r.read_ue()? } else { 0 };
        let mut pic_order_cnt_lsb = 0;
        let mut delta_pic_order_cnt_bottom = 0;
        let mut delta_pic_order_cnt = [0; 2];
        if sps.pic_order_cnt_type == 0 {
            pic_order_cnt_lsb = r.read_bits(sps.log2_max_poc_lsb)?;
            if pps.bottom_field_pic_order_in_frame_present && !field_pic {
                delta_pic_order_cnt_bottom = r.read_se()?;
            }
        }
        if sps.pic_order_cnt_type == 1 && !sps.delta_pic_order_always_zero {
            delta_pic_order_cnt[0] = r.read_se()?;
            if pps.bottom_field_pic_order_in_frame_present && !field_pic {
                delta_pic_order_cnt[1] = r.read_se()?;
            }
        }
        let redundant_pic_cnt = if pps.redundant_pic_cnt_present { r.read_ue()? } else { 0 };
        let direct_spatial_mv_pred = if slice_type.is_b() { r.read_flag()? } else { false };
        let mut num_ref_idx_active = [pps.num_ref_idx_l0_default_active, pps.num_ref_idx_l1_default_active];
        if !slice_type.is_intra() {
            if r.read_flag()? {
                num_ref_idx_active[0] = r.read_ue()?.checked_add(1)
                    .ok_or_else(|| crate::Error::Invalid("num_ref_idx_l0_active overflows".into()))?;
                if slice_type.is_b() {
                    num_ref_idx_active[1] = r.read_ue()?.checked_add(1)
                        .ok_or_else(|| crate::Error::Invalid("num_ref_idx_l1_active overflows".into()))?;
                }
            }
            let max = if field_pic { 32 } else { 16 };
            ensure!(num_ref_idx_active[0] <= max && num_ref_idx_active[1] <= max, "num_ref_idx_active out of range");
        }
        if !slice_type.is_b() {
            num_ref_idx_active[1] = 0;
        }
        if slice_type.is_intra() {
            num_ref_idx_active[0] = 0;
        }
        ensure!(nal.nal_unit_type != 20 && nal.nal_unit_type != 21, "MVC slices are not supported");
        let mut ref_pic_list_mod = [Vec::new(), Vec::new()];
        for (list, mods) in ref_pic_list_mod.iter_mut().enumerate() {
            let present = if list == 0 { !slice_type.is_intra() } else { slice_type.is_b() };
            if present && r.read_flag()? {
                loop {
                    let idc = r.read_ue()?;
                    ensure!(idc <= 5, "modification_of_pic_nums_idc out of range");
                    if idc == 3 {
                        break;
                    }
                    let value = r.read_ue()?;
                    if idc <= 1 { ensure!(value < sps.max_frame_num(), "reference list difference out of range"); }
                    mods.push(RefPicListMod { idc, value });
                    ensure!(mods.len() <= 66, "too many ref_pic_list_modification entries");
                }
            }
        }
        let mut pred_weight_table = None;
        if (pps.weighted_pred && slice_type.is_p()) || (pps.weighted_bipred_idc == 1 && slice_type.is_b()) {
            let luma_log2_denom = r.read_ue()?;
            ensure!(luma_log2_denom <= 7, "luma_log2_weight_denom out of range");
            let mut chroma_log2_denom = 0;
            if sps.chroma_array_type() != 0 {
                chroma_log2_denom = r.read_ue()?;
                ensure!(chroma_log2_denom <= 7, "chroma_log2_weight_denom out of range");
            }
            let mut lists = [Vec::new(), Vec::new()];
            let nlists = if slice_type.is_b() { 2 } else { 1 };
            for (l, entries) in lists.iter_mut().enumerate().take(nlists) {
                for _ in 0..num_ref_idx_active[l] {
                    let mut e = WeightEntry { luma_weight: 1 << luma_log2_denom, chroma_weight: [1 << chroma_log2_denom; 2], ..Default::default() };
                    if r.read_flag()? {
                        e.luma_flag = true;
                        e.luma_weight = r.read_se()?;
                        e.luma_offset = r.read_se()?;
                        ensure!((-128..=127).contains(&e.luma_weight) && (-128..=127).contains(&e.luma_offset), "luma weight out of range");
                    }
                    if sps.chroma_array_type() != 0 && r.read_flag()? {
                        e.chroma_flag = true;
                        for j in 0..2 {
                            e.chroma_weight[j] = r.read_se()?;
                            e.chroma_offset[j] = r.read_se()?;
                            ensure!((-128..=127).contains(&e.chroma_weight[j]) && (-128..=127).contains(&e.chroma_offset[j]), "chroma weight out of range");
                        }
                    }
                    entries.push(e);
                }
            }
            let [l0, l1] = lists;
            pred_weight_table = Some(PredWeightTable { luma_log2_denom, chroma_log2_denom, l0, l1 });
        }
        let mut no_output_of_prior_pics = false;
        let mut long_term_reference = false;
        let mut adaptive_ref_pic_marking = false;
        let mut mmcos = Vec::new();
        if nal.nal_ref_idc != 0 {
            if idr {
                no_output_of_prior_pics = r.read_flag()?;
                long_term_reference = r.read_flag()?;
            } else {
                adaptive_ref_pic_marking = r.read_flag()?;
                if adaptive_ref_pic_marking {
                    loop {
                        let op = r.read_ue()?;
                        ensure!(op <= 6, "memory_management_control_operation out of range");
                        if op == 0 {
                            break;
                        }
                        let mut m =
                            Mmco { op, difference_of_pic_nums_minus1: 0, long_term_pic_num: 0, long_term_frame_idx: 0, max_long_term_frame_idx_plus1: 0 };
                        if op == 1 || op == 3 {
                            m.difference_of_pic_nums_minus1 = r.read_ue()?;
                            ensure!(m.difference_of_pic_nums_minus1 < sps.max_frame_num(), "MMCO picture difference out of range");
                        }
                        if op == 2 {
                            m.long_term_pic_num = r.read_ue()?;
                        }
                        if op == 3 || op == 6 {
                            m.long_term_frame_idx = r.read_ue()?;
                        }
                        if op == 4 {
                            m.max_long_term_frame_idx_plus1 = r.read_ue()?;
                        }
                        mmcos.push(m);
                        ensure!(mmcos.len() <= 66, "too many MMCOs");
                    }
                }
            }
        }
        let mut cabac_init_idc = 0;
        if pps.entropy_coding_mode && !slice_type.is_intra() {
            cabac_init_idc = r.read_ue()?;
            ensure!(cabac_init_idc <= 2, "cabac_init_idc out of range");
        }
        let slice_qp_delta = r.read_se()?;
        let qp = pps.pic_init_qp + slice_qp_delta;
        ensure!((0..=51).contains(&qp), "slice QP {qp} out of range");
        let mut sp_for_switch = false;
        let mut slice_qs_delta = 0;
        if matches!(slice_type, SliceType::Sp | SliceType::Si) {
            if slice_type == SliceType::Sp {
                sp_for_switch = r.read_flag()?;
            }
            slice_qs_delta = r.read_se()?;
        }
        let mut disable_deblocking_filter_idc = 0;
        let mut slice_alpha_c0_offset_div2 = 0;
        let mut slice_beta_offset_div2 = 0;
        if pps.deblocking_filter_control_present {
            disable_deblocking_filter_idc = r.read_ue()?;
            ensure!(disable_deblocking_filter_idc <= 2, "disable_deblocking_filter_idc out of range");
            if disable_deblocking_filter_idc != 1 {
                slice_alpha_c0_offset_div2 = r.read_se()?;
                slice_beta_offset_div2 = r.read_se()?;
                ensure!((-6..=6).contains(&slice_alpha_c0_offset_div2) && (-6..=6).contains(&slice_beta_offset_div2), "deblocking offsets out of range");
            }
        }
        let mut slice_group_change_cycle = 0;
        if pps.num_slice_groups > 1 && (3..=5).contains(&pps.slice_group_map_type) {
            // Ceil(Log2(PicSizeInMapUnits / SliceGroupChangeRate + 1)); rate is not retained, so read
            // conservatively using the picture size (FMO is unsupported for decoding anyway).
            let pic_size = sps.pic_width_in_mbs * sps.pic_height_in_map_units;
            let bits = 32 - pic_size.leading_zeros();
            slice_group_change_cycle = r.read_bits(bits)?;
        }
        let header_bits = r.position();
        Ok((
            SliceHeader {
                nal_ref_idc: nal.nal_ref_idc,
                nal_unit_type: nal.nal_unit_type,
                idr,
                first_mb_in_slice,
                slice_type_raw,
                slice_type,
                pps_id,
                colour_plane_id,
                frame_num,
                field_pic,
                bottom_field,
                idr_pic_id,
                pic_order_cnt_lsb,
                delta_pic_order_cnt_bottom,
                delta_pic_order_cnt,
                redundant_pic_cnt,
                direct_spatial_mv_pred,
                num_ref_idx_active,
                ref_pic_list_mod,
                pred_weight_table,
                no_output_of_prior_pics,
                long_term_reference,
                adaptive_ref_pic_marking,
                mmcos,
                cabac_init_idc,
                slice_qp_delta,
                sp_for_switch,
                slice_qs_delta,
                disable_deblocking_filter_idc,
                slice_alpha_c0_offset_div2,
                slice_beta_offset_div2,
                slice_group_change_cycle,
                header_bits,
            },
            pps,
            sps,
        ))
    }
}

#[cfg(test)]
mod ref_index_overflow_tests {
    use super::*;
    use bitstream_codec::BitWriter;

    fn parameter_sets() -> (Sps, Pps) {
        let mut s = BitWriter::new();
        s.write_bits(66,8); s.write_bits(0,8); s.write_bits(30,8);
        s.write_ue(0); s.write_ue(0); s.write_ue(2); s.write_ue(1);
        s.write_bit(false); s.write_ue(0); s.write_ue(0);
        s.write_bit(true); s.write_bit(true); s.write_bit(false); s.write_bit(false);
        s.rbsp_trailing();
        let sps = Sps::parse(&s.finish()).unwrap();
        let mut p = BitWriter::new();
        p.write_ue(0); p.write_ue(0); p.write_bit(false); p.write_bit(false);
        p.write_ue(0); p.write_ue(0); p.write_ue(0); p.write_bit(false); p.write_bits(0,2);
        p.write_se(0); p.write_se(0); p.write_se(0); p.write_bit(true); p.write_bit(false); p.write_bit(false);
        p.rbsp_trailing();
        let mut lookup = vec![None; 32]; lookup[0] = Some(sps.clone());
        let pps = Pps::parse(&p.finish(), &lookup).unwrap();
        (sps, pps)
    }

    #[test]
    fn p_slice_rejects_oversized_ref_mod_and_mmco_differences() {
        let (sps, pps) = parameter_sets();
        for mmco in [false, true] {
            let mut w = BitWriter::new();
            w.write_ue(0); w.write_ue(0); w.write_ue(0); w.write_bits(0, 4);
            w.write_bit(false); // default reference counts
            w.write_bit(!mmco); // list modification flag
            if mmco { w.write_bit(true); w.write_ue(1); } else { w.write_ue(0); }
            w.write_ue(u32::MAX); w.rbsp_trailing();
            let bytes = w.finish();
            let parsed = SliceHeader::parse(&bytes, NalHeader {nal_ref_idc:1,nal_unit_type:nal_type::SLICE}, |_| Ok((&pps,&sps)));
            assert!(parsed.is_err());
        }
    }
    #[test]
    fn poc_overflow_is_rejected_and_normal_values_survive() {
        let (mut sps, pps) = parameter_sets();
        let mut w = BitWriter::new();
        w.write_ue(0); w.write_ue(7); w.write_ue(0); w.write_bits(1,4);
        w.write_bit(false); w.write_se(0); w.write_ue(0); w.write_se(0); w.write_se(0); w.rbsp_trailing();
        let bytes=w.finish();
        let (mut sh, _, _) = SliceHeader::parse(&bytes, NalHeader {nal_ref_idc:1,nal_unit_type:nal_type::SLICE}, |_| Ok((&pps,&sps))).unwrap();
        let state=PocState::default();
        let valid=state.compute(&sh,&sps).unwrap();
        assert_eq!((valid.top,valid.bottom),(2,2));
        sps.pic_order_cnt_type=1; sps.offset_for_ref_frame=vec![i32::MAX];
        sh.frame_num=2;
        assert!(state.compute(&sh,&sps).is_err());
        sps.pic_order_cnt_type=0; sps.log2_max_poc_lsb=4;
        sh.pic_order_cnt_lsb=1; sh.delta_pic_order_cnt_bottom=i32::MAX;
        assert!(state.compute(&sh,&sps).is_err());
        let mut state=PocState { prev_frame_num_offset:i32::MAX, prev_frame_num:1, ..Default::default() };
        assert!(state.update_gap_frame(0,&sps).is_err());
    }
    #[test]
    fn p_slice_ref_count_overflow_is_error_not_panic_or_wrapped_zero() {
        let (sps, pps) = parameter_sets();
        let mut w = BitWriter::new();
        w.write_ue(0); w.write_ue(0); w.write_ue(0); w.write_bits(0,4);
        w.write_bit(true); w.write_ue(u32::MAX); w.rbsp_trailing();
        let bytes = w.finish();
        let result = std::panic::catch_unwind(|| SliceHeader::parse(&bytes, NalHeader {nal_ref_idc:1,nal_unit_type:nal_type::SLICE}, |_| Ok((&pps,&sps))));
        assert!(matches!(result, Ok(Err(_))));
    }
}

/// POC decoding state carried between pictures (8.2.1).
#[derive(Clone, Debug, Default)]
pub struct PocState {
    /// PicOrderCntMsb / pic_order_cnt_lsb of the previous reference picture (type 0).
    prev_ref_msb: i32,
    prev_ref_lsb: i32,
    /// FrameNumOffset / frame_num of the previous picture (types 1 and 2).
    prev_frame_num_offset: i32,
    prev_frame_num: u32,
}

/// Is `sh` the first slice of a new primary coded picture relative to `prev` (7.4.1.2.4)?
pub fn is_new_picture(prev: &SliceHeader, sh: &SliceHeader, sps: &Sps) -> bool {
    if sh.frame_num != prev.frame_num
        || sh.pps_id != prev.pps_id
        || sh.field_pic != prev.field_pic
        || sh.bottom_field != prev.bottom_field
        || (sh.nal_ref_idc == 0) != (prev.nal_ref_idc == 0)
        || sh.idr != prev.idr
        || (sh.idr && prev.idr && sh.idr_pic_id != prev.idr_pic_id)
    {
        return true;
    }
    if sps.pic_order_cnt_type == 0 && (sh.pic_order_cnt_lsb != prev.pic_order_cnt_lsb || sh.delta_pic_order_cnt_bottom != prev.delta_pic_order_cnt_bottom) {
        return true;
    }
    if sps.pic_order_cnt_type == 1 && sh.delta_pic_order_cnt != prev.delta_pic_order_cnt {
        return true;
    }
    false
}

/// Output of POC computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poc {
    pub top: i32,
    pub bottom: i32,
    msb: i32,
    frame_num_offset: i32,
}

impl Poc {
    pub fn frame(&self) -> i32 {
        self.top.min(self.bottom)
    }
}

impl PocState {
    /// Compute TopFieldOrderCnt/BottomFieldOrderCnt of a frame (8.2.1.1-8.2.1.3).
    pub fn compute(&self, sh: &SliceHeader, sps: &Sps) -> Result<Poc> {
        fn checked(v: Option<i64>) -> Result<i64> { v.ok_or_else(|| crate::Error::Invalid("POC arithmetic overflow".into())) }
        fn narrow(v: i64) -> Result<i32> { i32::try_from(v).map_err(|_| crate::Error::Invalid("POC exceeds supported range".into())) }
        let poc = match sps.pic_order_cnt_type {
            0 => {
                let (prev_msb, prev_lsb) = if sh.idr { (0i64, 0i64) } else { (i64::from(self.prev_ref_msb), i64::from(self.prev_ref_lsb)) };
                let max_lsb = 1i64.checked_shl(sps.log2_max_poc_lsb).ok_or_else(|| crate::Error::Invalid("Invalid POC width".into()))?;
                let lsb = i64::from(sh.pic_order_cnt_lsb);
                let msb = if lsb < prev_lsb && prev_lsb - lsb >= max_lsb / 2 { checked(prev_msb.checked_add(max_lsb))? }
                    else if lsb > prev_lsb && lsb - prev_lsb > max_lsb / 2 { checked(prev_msb.checked_sub(max_lsb))? } else { prev_msb };
                let top = checked(msb.checked_add(lsb))?;
                let bottom = checked(top.checked_add(i64::from(sh.delta_pic_order_cnt_bottom)))?;
                Poc { top:narrow(top)?, bottom:narrow(bottom)?, msb:narrow(msb)?, frame_num_offset:0 }
            }
            1 => {
                let off = self.frame_num_offset(sh.idr, sh.frame_num, sps)?;
                let n = i64::try_from(sps.offset_for_ref_frame.len()).map_err(|_| crate::Error::Invalid("POC cycle too large".into()))?;
                let mut abs = if n != 0 { checked(i64::from(off).checked_add(i64::from(sh.frame_num)))? } else { 0 };
                if sh.nal_ref_idc == 0 && abs > 0 { abs -= 1; }
                let mut expected = 0i64;
                if abs > 0 {
                    let cycle = (abs - 1) / n;
                    let index = ((abs - 1) % n) as usize;
                    let mut delta = 0i64;
                    for &o in &sps.offset_for_ref_frame { delta = checked(delta.checked_add(i64::from(o)))?; }
                    expected = checked(cycle.checked_mul(delta))?;
                    for &o in &sps.offset_for_ref_frame[..=index] { expected = checked(expected.checked_add(i64::from(o)))?; }
                }
                if sh.nal_ref_idc == 0 { expected = checked(expected.checked_add(i64::from(sps.offset_for_non_ref_pic)))?; }
                let top = checked(expected.checked_add(i64::from(sh.delta_pic_order_cnt[0])))?;
                let bottom = checked(checked(top.checked_add(i64::from(sps.offset_for_top_to_bottom_field)))?.checked_add(i64::from(sh.delta_pic_order_cnt[1])))?;
                Poc { top:narrow(top)?, bottom:narrow(bottom)?, msb:0, frame_num_offset:off }
            }
            _ => {
                let off = self.frame_num_offset(sh.idr, sh.frame_num, sps)?;
                let temp = if sh.idr { 0 } else {
                    let n = checked(i64::from(off).checked_add(i64::from(sh.frame_num)))?;
                    let doubled = checked(n.checked_mul(2))?;
                    if sh.nal_ref_idc == 0 { checked(doubled.checked_sub(1))? } else { doubled }
                };
                let temp=narrow(temp)?;
                Poc { top:temp, bottom:temp, msb:0, frame_num_offset:off }
            }
        };
        if sh.has_mmco5() { poc.top.checked_sub(poc.frame()).ok_or_else(|| crate::Error::Invalid("MMCO5 POC overflow".into()))?; }
        Ok(poc)
    }

    fn frame_num_offset(&self, idr: bool, frame_num: u32, sps: &Sps) -> Result<i32> {
        if idr { Ok(0) } else if self.prev_frame_num > frame_num {
            self.prev_frame_num_offset.checked_add(sps.max_frame_num() as i32).ok_or_else(|| crate::Error::Invalid("frame number offset overflow".into()))
        } else { Ok(self.prev_frame_num_offset) }
    }
    /// Update after a picture has been decoded. `poc` is the value returned by [`PocState::compute`]
    /// (before any MMCO5 adjustment).
    pub fn update(&mut self, sh: &SliceHeader, poc: &Poc) {
        let mmco5 = sh.has_mmco5();
        if sh.nal_ref_idc != 0 {
            if mmco5 {
                self.prev_ref_msb = 0;
                self.prev_ref_lsb = poc.top - poc.frame();
            } else {
                self.prev_ref_msb = poc.msb;
                self.prev_ref_lsb = sh.pic_order_cnt_lsb as i32;
            }
        }
        if mmco5 {
            self.prev_frame_num_offset = 0;
            self.prev_frame_num = 0;
        } else {
            self.prev_frame_num_offset = poc.frame_num_offset;
            self.prev_frame_num = sh.frame_num;
        }
    }

    /// Account for a "non-existing" frame inferred by the frame_num gap process (8.2.5.2).
    pub fn update_gap_frame(&mut self, frame_num: u32, sps: &Sps) -> Result<()> {
        let off = self.frame_num_offset(false, frame_num, sps)?;
        self.prev_frame_num_offset = off;
        self.prev_frame_num = frame_num;
        Ok(())
    }
}
