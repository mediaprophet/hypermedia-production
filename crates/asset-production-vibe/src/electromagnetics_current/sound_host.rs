use super::{CurrentDocument, ReadLimits};
use crate::*;
impl AssetHost<'_> {
    pub(crate) fn sonify_current_emf(
        &mut self,
        value: &Value,
        rate: u32,
        frames: usize,
        frequency: f64,
        amplitude: f64,
    ) -> Result<(Vec<f32>, usize, usize), String> {
        let plan = self.current_read_plan(value)?;
        let audio = frames
            .checked_mul(4)
            .and_then(|n| n.checked_add(8192 * 24))
            .ok_or("EMF sonic peak overflow")?;
        self.emf_peak_admit(
            plan.memory
                .checked_add(audio)
                .ok_or("EMF sonic peak overflow")?,
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
        self.charge(8192 * 4, 0)?;
        let sonic = super::sonification::sonify(
            &document,
            rate,
            frames,
            frequency,
            amplitude,
            self.limits.total_resource_bytes,
        )?;
        Ok((sonic.samples, sonic.source_samples, sonic.missing_samples))
    }
}
