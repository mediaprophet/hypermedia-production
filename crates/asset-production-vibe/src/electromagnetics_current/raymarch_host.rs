use super::{render_raymarch, CurrentDocument, GridAtTime, ProjectionLimits, ReadLimits};
use crate::*;
impl AssetHost<'_> {
    pub(crate) fn raymarch_current_emf(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "emf",
                "time_slice",
                "width",
                "height",
                "deflection_mass",
                "step_limit",
                "camera_position",
            ],
        )?;
        if number(args, "deflection_mass", 0.0)? != 0.0 {
            return Err("Current field-magnitude DDA derivative supports deflection_mass0; no gravitational/optical propagation model is implied".into());
        }
        let value = required(args, "emf")?;
        let selected = super::read_host::time_slice(args)?;
        let width = count(args, "width", 64, 512)?;
        let height = count(args, "height", 64, 512)?;
        let steps = count(args, "step_limit", 48, 4096)?;
        let camera = args.get("camera_position").map(vector::<3>).transpose()?;
        let pixels = width.checked_mul(height).ok_or("Raymarch pixel overflow")?;
        let pixel_bytes = pixels.checked_mul(4).ok_or("Raymarch byte overflow")?;
        if pixel_bytes > self.limits.resource_bytes {
            return Err("Raymarch exceeds selected resource bytes".into());
        }
        let plan = self.current_read_plan(value)?;
        self.emf_peak_admit(
            plan.memory
                .checked_add(pixel_bytes)
                .ok_or("Raymarch peak overflow")?,
        )?;
        let bytes = self.emf_source_bytes(value)?.to_vec();
        let document = CurrentDocument::open(
            &bytes,
            ReadLimits {
                file_bytes: self.limits.resource_bytes,
                cold_work: plan.shared_work,
                cache_bytes: plan.cache,
            },
        )?;
        self.charge_current_read(&document)?;
        let grid = GridAtTime::new(&document, selected)?;
        let work = (pixels as u64)
            .checked_mul(steps as u64)
            .and_then(|n| n.checked_mul(grid.lookup_work()))
            .ok_or("Raymarch work overflow")?;
        self.charge(
            usize::try_from(work).map_err(|_| "Raymarch work overflow")?,
            0,
        )?;
        let rendered = render_raymarch(
            &grid,
            width,
            height,
            steps,
            camera,
            1.0,
            ProjectionLimits { pixels, work },
        )?;
        let source_clock = document
            .view
            .blocks()
            .find(|b| b.id() == grid.clock_id)
            .map(super::read_host::clock_value)
            .transpose()?
            .ok_or("Admitted source clock missing")?;
        let source_frame = document
            .view
            .blocks()
            .find(|b| b.id() == grid.frame_id)
            .map(super::read_host::frame_value)
            .transpose()?
            .ok_or("Admitted source frame missing")?;
        let digest = super::read_host::index_digest(document.view.index_digest());
        let frame = grid.frame_id;
        let clock = grid.clock_id;
        let tick = grid.selected_tick;
        drop(grid);
        drop(document);
        let mut image = self.store_rgba(width as u32, height as u32, rendered.pixels)?;
        if let Value::Record(meta) = &mut image {
            meta.insert("source_clock".into(), source_clock);
            meta.insert("source_frame".into(), source_frame);
            meta.insert("source_index_digest_blake3".into(), Value::String(digest));
            super::read_host::projection_metadata(
                meta,
                "electric_magnitude_grid_alpha_dda_v1",
                frame,
                clock,
                tick,
                selected,
                1.0,
            );
            meta.insert("projection_scope".into(),Value::String("grid-space alpha compositor of magnitude palette; not absorption, energy, radiative transfer or gravitational propagation".into()));
            meta.insert(
                "camera_position_units".into(),
                Value::String("local metres in source frame".into()),
            );
        }
        let mut result = BTreeMap::new();
        result.insert("image".into(), image);
        result.insert("dda_steps_total".into(), Value::U64(rendered.steps));
        result.insert("brick_hits".into(), Value::U64(rendered.present_samples));
        result.insert("width".into(), Value::U64(width as u64));
        result.insert("height".into(), Value::U64(height as u64));
        Ok(Value::Record(result))
    }
}
