//! Wet paint Mixer Brush pickup and dryout simulation.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

/// Rate at which picked-up paint moves towards canvas color per dab at 100% Wet.
pub const PICKUP_RATE: f32 = 0.25;

/// Drain rate of reservoir paint per dab when Load < 1.0.
pub const DRYOUT_RATE: f32 = 0.02;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MixerSettings {
    /// 0..1: How wet the brush is (0 = completely dry, 1 = maximum smear).
    pub wet: f32,
    /// 0..1: Paint loaded into reservoir (1 = never dries out).
    pub load: f32,
    /// 0..1: Mix ratio between reservoir color and canvas pickup color.
    pub mix: f32,
    /// 0..1: Flow rate of deposited paint.
    pub flow: f32,
    /// Whether to sample color across all layers or only active layer.
    pub sample_all_layers: bool,
}

impl Default for MixerSettings {
    fn default() -> Self {
        Self {
            wet: 0.5,
            load: 0.5,
            mix: 0.5,
            flow: 1.0,
            sample_all_layers: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MixerState {
    /// Paint in brush reservoir (straight RGBA). None = clean/unloaded.
    pub reservoir: Option<[f32; 4]>,
    /// Remaining amount in reservoir (0.0..=1.0).
    pub amount: f32,
    /// Paint picked up from canvas (straight RGBA). None = clean.
    pub pickup: Option<[f32; 4]>,
}

impl MixerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reservoir(color: [f32; 4]) -> Self {
        Self {
            reservoir: Some(color),
            amount: 1.0,
            pickup: None,
        }
    }

    pub fn load(&mut self, color: [f32; 4]) {
        self.reservoir = Some(color);
        self.amount = 1.0;
    }

    pub fn clean(&mut self) {
        self.pickup = None;
    }
}

/// Simulates a single dab of the wet mixer brush over a sampled canvas pixel.
///
/// Updates `state` (pickup color and reservoir amount) and returns the deposited straight RGBA color.
pub fn mix_dab(
    canvas_color: [f32; 4],
    settings: &MixerSettings,
    state: &mut MixerState,
) -> [f32; 4] {
    let wet = settings.wet.clamp(0.0, 1.0);
    let mix = settings.mix.clamp(0.0, 1.0);
    let flow = settings.flow.clamp(0.0, 1.0);

    // 1. Update pickup color moving towards canvas color by `wet * PICKUP_RATE`
    let pickup_k = (wet * PICKUP_RATE).clamp(0.0, 1.0);
    let new_pickup = match state.pickup {
        Some(p) => [
            p[0] + (canvas_color[0] - p[0]) * pickup_k,
            p[1] + (canvas_color[1] - p[1]) * pickup_k,
            p[2] + (canvas_color[2] - p[2]) * pickup_k,
            p[3] + (canvas_color[3] - p[3]) * pickup_k,
        ],
        None => canvas_color,
    };
    state.pickup = Some(new_pickup);

    // 2. Compute deposited color
    let (res_col, res_amt) = match state.reservoir {
        Some(c) => (c, state.amount),
        None => ([0.0, 0.0, 0.0, 0.0], 0.0),
    };

    // Weight of reservoir vs pickup
    let res_weight = res_amt * (1.0 - mix * wet);
    let pickup_weight = mix * wet;
    let total_weight = (res_weight + pickup_weight).max(1e-6);

    let norm_res = res_weight / total_weight;
    let norm_pickup = pickup_weight / total_weight;

    let deposited_r = res_col[0] * norm_res + new_pickup[0] * norm_pickup;
    let deposited_g = res_col[1] * norm_res + new_pickup[1] * norm_pickup;
    let deposited_b = res_col[2] * norm_res + new_pickup[2] * norm_pickup;
    let deposited_a = (res_col[3] * norm_res + new_pickup[3] * norm_pickup) * flow;

    // 3. Drain reservoir amount if load < 1.0
    if settings.load < 1.0 {
        let drain = (1.0 - settings.load) * DRYOUT_RATE;
        state.amount = (state.amount - drain).max(0.0);
    }

    [
        deposited_r.clamp(0.0, 1.0),
        deposited_g.clamp(0.0, 1.0),
        deposited_b.clamp(0.0, 1.0),
        deposited_a.clamp(0.0, 1.0),
    ]
}
