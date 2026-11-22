//! Band-limited angular-spectrum propagation and phase-only hologram synthesis.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{f64::consts::TAU, sync::Arc};

use qualia_core_db::audio::qac::spatial_dsp::fft_radix2_interleaved_f32;

use crate::artifact::{FieldLattice, LatticeSamples, RadianceLattice};
use crate::primal_replay::counter_uniform;
use crate::{fields, integer, num, number, required, Args, AssetHost, TypedArtifact, Value};

struct Propagator {
    width: usize,
    transfer: Vec<[f32; 2]>,
    column: Vec<f32>,
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_cgh_wave_diffraction(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "field",
                "wavelength_nm",
                "slm_pitch_m",
                "propagation_dist_m",
                "synthesis_iterations",
                "seed",
                "white_intensity",
            ],
        )?;
        let handle = required(args, "field")?;
        let TypedArtifact::FieldLattice(field) = self.workspace.typed(handle)? else {
            return Err("Wave diffraction requires a retained DenseF32 field lattice".into());
        };
        let field = Arc::clone(field);
        let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
            return Err("Wave diffraction requires DenseF32 complex samples".into());
        };
        let [height, width, 2] = axes.as_slice() else {
            return Err("Wave diffraction field shape must be [height,width,2] with adjacent real/imaginary lanes".into());
        };
        let (w, h) = (*width as usize, *height as usize);
        if !w.is_power_of_two()
            || !h.is_power_of_two()
            || w > 512
            || h > 512
            || w < 2
            || h < 2
            || samples.len() != 2 * w * h
            || samples.iter().any(|v| !(-1.0e6..=1.0e6).contains(v))
        {
            return Err("Diffraction needs complete finite complex samples and power-of-two dimensions 2..=512".into());
        }
        let wavelength_nm = num(required(args, "wavelength_nm")?)?;
        let wavelength = wavelength_nm * 1e-9;
        let pitch = num(required(args, "slm_pitch_m")?)?;
        let distance = num(required(args, "propagation_dist_m")?)?;
        let white = number(args, "white_intensity", 1.0)?;
        let iterations = args
            .get("synthesis_iterations")
            .map_or(Ok(0), |value| integer(value, "synthesis_iterations"))?;
        if !(100.0..=20_000.0).contains(&wavelength_nm)
            || !(5.0e-7..=1.0e-4).contains(&pitch)
            || !(1.0e-5..=10.0).contains(&distance)
            || !(1.0e-6..=1.0e12).contains(&white)
            || iterations > 32
        {
            return Err("Wave diffraction requires the admitted wavelength, SLM pitch, distance and intensity bounds".into());
        }
        let seed = match args.get("seed") {
            None => 0,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            _ => return Err("CGH seed must be an unsigned 64-bit value".into()),
        };
        let pixels = w * h;
        let transform_work = pixels * (w.ilog2() + h.ilog2()) as usize * 20;
        self.charge(transform_work * (2 + iterations * 4), pixels * 64 + h * 8)?;
        let mut propagator = Propagator::new(w, h, wavelength, pitch, distance);
        let mut source = samples.clone();
        let mut output = vec![0.0; 2 * pixels];
        let mut phase_only = None;
        if iterations > 0 {
            let amplitude: Vec<_> = samples.chunks_exact(2).map(|c| c[0].hypot(c[1])).collect();
            let rms = (amplitude.iter().map(|&v| f64::from(v).powi(2)).sum::<f64>()
                / pixels as f64)
                .sqrt();
            if rms == 0.0 {
                return Err("Phase-only synthesis requires a nonzero target amplitude".into());
            }
            for (index, point) in source.chunks_exact_mut(2).enumerate() {
                let (s, c) = (TAU * counter_uniform(seed, index as u64, 0)).sin_cos();
                point.copy_from_slice(&[c as f32, s as f32]);
            }
            for _ in 0..iterations {
                self.charge(0, 0)?;
                output.copy_from_slice(&source);
                propagator.apply(&mut output, false)?;
                for (point, &target) in output.chunks_exact_mut(2).zip(&amplitude) {
                    set_amplitude(point, (f64::from(target) / rms) as f32);
                }
                propagator.apply(&mut output, true)?;
                for point in output.chunks_exact_mut(2) {
                    set_amplitude(point, 1.0);
                }
                source.copy_from_slice(&output);
            }
            phase_only = Some(
                source
                    .chunks_exact(2)
                    .map(|p| p[1].atan2(p[0]))
                    .collect::<Vec<_>>(),
            );
        }
        output.copy_from_slice(&source);
        propagator.apply(&mut output, false)?;
        let mut kept = 0;
        let rgba: Vec<_> = output
            .chunks_exact(2)
            .flat_map(|c| {
                let intensity = f64::from(c[0]).powi(2) + f64::from(c[1]).powi(2);
                if intensity > white {
                    kept += 1;
                }
                let mono = ((intensity / white).clamp(0.0, 1.0) * 65535.0).round() as u16;
                [mono, mono, mono, u16::MAX]
            })
            .collect();
        let receipt = super::receipt(
            self,
            "band-limited-angular-spectrum",
            &output,
            &[
                ("width", w as f64),
                ("height", h as f64),
                ("wavelength_nm", wavelength_nm),
                ("slm_pitch_m", pitch),
                ("propagation_dist_m", distance),
                ("white_intensity", white),
                ("synthesis_iterations", iterations as f64),
            ],
        )?;
        let complex = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![h as u32, w as u32, 2],
                samples: output,
            },
        )))?;
        let image =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: w as u32,
                height: h as u32,
                channels: 4,
                primaries_iri: "urn:qualia:color:primaries:rec709".into(),
                transfer_iri: "urn:qualia:color:transfer:linear".into(),
                samples: LatticeSamples::U16(rgba),
            })))?;
        let mut record = Args::from([
            ("field".into(), complex),
            ("image".into(), image),
            ("receipt".into(), receipt),
            ("clipped_pixels".into(), Value::U64(kept)),
        ]);
        if let Some(samples) = phase_only {
            let phase = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
                FieldLattice::DenseF32 {
                    axes: vec![h as u32, w as u32],
                    samples,
                },
            )))?;
            record.insert("phase_only_rad".into(), phase);
        }
        Ok(Value::Record(record))
    }
}

impl Propagator {
    fn new(width: usize, height: usize, lambda: f64, pitch: f64, distance: f64) -> Self {
        let extent_x = width as f64 * pitch;
        let extent_y = height as f64 * pitch;
        let mut transfer = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let fx = centered_bin(x, width) / extent_x;
                let fy = centered_bin(y, height) / extent_y;
                let rho = fx.mul_add(fx, fy * fy);
                let longitudinal = lambda.recip().powi(2) - rho;
                let kz = longitudinal.max(0.0).sqrt();
                // Sampling the transfer function requires its phase difference between
                // adjoining discrete frequencies to stay within the Nyquist interval.
                if longitudinal <= 0.0
                    || 2.0 * distance * fx.abs() > extent_x * kz
                    || 2.0 * distance * fy.abs() > extent_y * kz
                {
                    transfer.push([0.0, 0.0]);
                } else {
                    // Report the slowly varying envelope after removing the plane-wave carrier.
                    let phase = -TAU * distance * rho / (kz + lambda.recip());
                    let (s, c) = phase.sin_cos();
                    transfer.push([c as f32, s as f32]);
                }
            }
        }
        Self {
            width,
            transfer,
            column: vec![0.0; 2 * height],
        }
    }

    fn apply(&mut self, samples: &mut [f32], reverse: bool) -> Result<(), String> {
        self.transform(samples, false)?;
        for (c, &[re, im]) in samples.chunks_exact_mut(2).zip(&self.transfer) {
            let im = if reverse { -im } else { im };
            let [x, y] = [c[0], c[1]];
            c[0] = x * re - y * im;
            c[1] = x * im + y * re;
        }
        self.transform(samples, true)?;
        if samples.iter().any(|v| !v.is_finite()) {
            return Err("Angular-spectrum propagation produced a nonfinite sample".into());
        }
        Ok(())
    }

    fn transform(&mut self, samples: &mut [f32], inverse: bool) -> Result<(), String> {
        let row_width = 2 * self.width;
        let fft = |array: &mut [f32]| {
            fft_radix2_interleaved_f32(array, inverse)
                .map_err(|e| format!("Native diffraction FFT rejected its axis: {e:?}"))
        };
        for row in samples.chunks_exact_mut(row_width) {
            fft(row)?;
        }
        for x in 0..self.width {
            for (row, out) in samples
                .chunks_exact(row_width)
                .zip(self.column.chunks_exact_mut(2))
            {
                out.copy_from_slice(&row[2 * x..2 * x + 2]);
            }
            fft(&mut self.column)?;
            for (row, src) in samples
                .chunks_exact_mut(row_width)
                .zip(self.column.chunks_exact(2))
            {
                row[2 * x..2 * x + 2].copy_from_slice(src);
            }
        }
        Ok(())
    }
}

fn centered_bin(index: usize, extent: usize) -> f64 {
    if index < extent / 2 {
        index as f64
    } else {
        index as f64 - extent as f64
    }
}

fn set_amplitude(point: &mut [f32], amplitude: f32) {
    let magnitude = point[0].hypot(point[1]);
    if magnitude > 0.0 {
        // Divide before multiplying to avoid overflow for subnormal magnitudes.
        point[0] = point[0] / magnitude * amplitude;
        point[1] = point[1] / magnitude * amplitude;
    } else {
        point[0] = amplitude;
        point[1] = 0.0;
    }
}
