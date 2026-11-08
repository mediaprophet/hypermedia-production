//! 3D Gaussian Splatting and radiance field synthesis cost and credit estimation.
//!
//! Provides credit and USD calculation for generative 3D radiance fields and Gaussian splat models
//! across diverse multimodal inputs (text, panoramas, images, videos).

use serde::{Deserialize, Serialize};

/// 3D Radiance field and Gaussian splat synthesis models.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadianceModel {
    Marble0p1Mini,
    Marble0p1Plus,
    Marble1p0,
    Marble1p0Draft,
    Marble1p1,
    Marble1p1Plus,
}

impl RadianceModel {
    pub const ALL: [RadianceModel; 6] = [
        RadianceModel::Marble0p1Mini,
        RadianceModel::Marble0p1Plus,
        RadianceModel::Marble1p0,
        RadianceModel::Marble1p0Draft,
        RadianceModel::Marble1p1,
        RadianceModel::Marble1p1Plus,
    ];

    pub fn api_name(self) -> &'static str {
        match self {
            RadianceModel::Marble0p1Mini => "marble_0p1_mini",
            RadianceModel::Marble0p1Plus => "marble_0p1_plus",
            RadianceModel::Marble1p0 => "marble_1p0",
            RadianceModel::Marble1p0Draft => "marble_1p0_draft",
            RadianceModel::Marble1p1 => "marble_1p1",
            RadianceModel::Marble1p1Plus => "marble_1p1_plus",
        }
    }
}

/// The type of input used to generate a 3D radiance field / splat scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadianceInputType {
    Text,
    ImageNonPanorama,
    ImagePanorama,
    MultiImage,
    Video,
}

impl RadianceInputType {
    pub const ALL: [RadianceInputType; 5] = [
        RadianceInputType::Text,
        RadianceInputType::ImageNonPanorama,
        RadianceInputType::ImagePanorama,
        RadianceInputType::MultiImage,
        RadianceInputType::Video,
    ];
}

/// Cost breakdown for a 3D radiance field generation request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplatCostBreakdown {
    pub credits: u32,
    pub us_dollar_cents: u32,
}

/// Calculate the cost of a 3D Gaussian Splat / radiance field generation.
pub fn calculate_splat_cost(model: RadianceModel, input_type: RadianceInputType) -> SplatCostBreakdown {
    let credits = credits_for(model, input_type);
    let us_dollar_cents = credits_to_usd_cents(credits);
    SplatCostBreakdown {
        credits,
        us_dollar_cents,
    }
}

/// Returns the credit cost for a given model and input type.
pub fn credits_for(model: RadianceModel, input_type: RadianceInputType) -> u32 {
    match (model, input_type) {
        // Marble 0.1-mini & 1.0-draft
        (RadianceModel::Marble0p1Mini | RadianceModel::Marble1p0Draft, RadianceInputType::ImagePanorama) => 150,
        (RadianceModel::Marble0p1Mini | RadianceModel::Marble1p0Draft, RadianceInputType::Text) => 230,
        (RadianceModel::Marble0p1Mini | RadianceModel::Marble1p0Draft, RadianceInputType::ImageNonPanorama) => 230,
        (RadianceModel::Marble0p1Mini | RadianceModel::Marble1p0Draft, RadianceInputType::MultiImage) => 250,
        (RadianceModel::Marble0p1Mini | RadianceModel::Marble1p0Draft, RadianceInputType::Video) => 250,

        // Marble 0.1-plus, 1.0, and 1.1 standard
        (RadianceModel::Marble0p1Plus | RadianceModel::Marble1p0 | RadianceModel::Marble1p1, RadianceInputType::ImagePanorama) => 1500,
        (RadianceModel::Marble0p1Plus | RadianceModel::Marble1p0 | RadianceModel::Marble1p1, RadianceInputType::Text) => 1580,
        (RadianceModel::Marble0p1Plus | RadianceModel::Marble1p0 | RadianceModel::Marble1p1, RadianceInputType::ImageNonPanorama) => 1580,
        (RadianceModel::Marble0p1Plus | RadianceModel::Marble1p0 | RadianceModel::Marble1p1, RadianceInputType::MultiImage) => 1600,
        (RadianceModel::Marble0p1Plus | RadianceModel::Marble1p0 | RadianceModel::Marble1p1, RadianceInputType::Video) => 1600,

        // Marble 1.1-plus (range pricing ceiling)
        (RadianceModel::Marble1p1Plus, RadianceInputType::ImagePanorama) => 3000,
        (RadianceModel::Marble1p1Plus, RadianceInputType::Text) => 3080,
        (RadianceModel::Marble1p1Plus, RadianceInputType::ImageNonPanorama) => 3080,
        (RadianceModel::Marble1p1Plus, RadianceInputType::MultiImage) => 3100,
        (RadianceModel::Marble1p1Plus, RadianceInputType::Video) => 3100,
    }
}

/// Converts credits to US dollar cents.
/// Rate: $1.00 USD per 1,250 credits -> 100 cents per 1,250 credits.
pub fn credits_to_usd_cents(credits: u32) -> u32 {
    ((credits as u64 * 100) / 1250) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_and_draft_costs() {
        assert_eq!(calculate_splat_cost(RadianceModel::Marble0p1Mini, RadianceInputType::Text).credits, 230);
        assert_eq!(calculate_splat_cost(RadianceModel::Marble0p1Mini, RadianceInputType::ImagePanorama).credits, 150);
        assert_eq!(calculate_splat_cost(RadianceModel::Marble1p0Draft, RadianceInputType::Video).credits, 250);
    }

    #[test]
    fn plus_and_1p1_costs() {
        assert_eq!(calculate_splat_cost(RadianceModel::Marble0p1Plus, RadianceInputType::Text).credits, 1580);
        assert_eq!(calculate_splat_cost(RadianceModel::Marble1p1, RadianceInputType::ImagePanorama).credits, 1500);
        assert_eq!(calculate_splat_cost(RadianceModel::Marble1p1Plus, RadianceInputType::Video).credits, 3100);
    }

    #[test]
    fn dollar_conversions() {
        assert_eq!(credits_to_usd_cents(1250), 100); // $1.00
        assert_eq!(credits_to_usd_cents(150), 12);   // 12 cents
        assert_eq!(credits_to_usd_cents(230), 18);   // 18 cents
        assert_eq!(credits_to_usd_cents(1580), 126); // 126 cents
        assert_eq!(credits_to_usd_cents(3100), 248); // 248 cents
    }
}
