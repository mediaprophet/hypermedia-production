//! Timeline compilation: organizes animations into sequential click steps.

use super::anim::{AnimClass, AnimStart, Animation, EffectTiming, Sequence};
use crate::model::master::Slide;
use crate::model::types::ShapeId;

#[derive(Clone, Debug, Default)]
pub struct StepInfo {
    pub step: usize,
    pub duration: f64,
    pub effects: Vec<EffectTiming>,
}

#[derive(Clone, Debug, Default)]
pub struct Timeline {
    pub steps: Vec<StepInfo>,
    pub total_steps: usize,
}

impl Timeline {
    pub fn compile(slide: &Slide) -> Self {
        let mut steps: Vec<StepInfo> = Vec::new();
        let mut cur_step = 0;
        let mut cur_step_effects = Vec::new();
        let mut cur_step_duration: f64 = 0.0;
        let mut last_end_time: f64 = 0.0;

        for (idx, anim) in slide.animations.iter().enumerate() {
            let (step, start_time) = match anim.start {
                AnimStart::OnClick => {
                    if !cur_step_effects.is_empty() {
                        steps.push(StepInfo {
                            step: cur_step,
                            duration: cur_step_duration,
                            effects: std::mem::take(&mut cur_step_effects),
                        });
                        cur_step += 1;
                    }
                    cur_step_duration = 0.0;
                    last_end_time = 0.0;
                    (cur_step, anim.delay)
                }
                AnimStart::WithPrev => {
                    // Starts at same time as previous in current step
                    (cur_step, anim.delay)
                }
                AnimStart::AfterPrev => {
                    // Starts after previous completes
                    (cur_step, last_end_time + anim.delay)
                }
            };

            let span = anim.duration * anim.repeat_count.max(1.0);
            let end_time = start_time + span;
            if end_time > cur_step_duration {
                cur_step_duration = end_time;
            }
            last_end_time = end_time;

            let timing = EffectTiming {
                anim_index: idx,
                shape: anim.shape,
                paragraph: None,
                sequence: Sequence::Main,
                step,
                start: start_time,
                duration: anim.duration,
                span,
                class: anim.class,
            };

            cur_step_effects.push(timing);
        }

        if !cur_step_effects.is_empty() {
            steps.push(StepInfo {
                step: cur_step,
                duration: cur_step_duration,
                effects: cur_step_effects,
            });
            cur_step += 1;
        }

        Timeline {
            total_steps: cur_step,
            steps,
        }
    }

    pub fn duration_of_step(&self, step: usize) -> f64 {
        self.steps.get(step).map(|s| s.duration).unwrap_or(0.0)
    }

    /// Check if shape is visible at a given step and progress t.
    pub fn is_shape_visible(&self, shape_id: ShapeId, step: usize, t: f64) -> bool {
        // By default visible unless it has an Entrance animation in a future step,
        // or an Exit animation that already completed.
        for s in &self.steps {
            for eff in &s.effects {
                if eff.shape != shape_id {
                    continue;
                }
                if eff.class == AnimClass::Entrance {
                    if s.step > step {
                        return false;
                    }
                    if s.step == step && t < eff.start {
                        return false;
                    }
                } else if eff.class == AnimClass::Exit {
                    if s.step < step {
                        return false;
                    }
                    if s.step == step && t >= (eff.start + eff.span) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
