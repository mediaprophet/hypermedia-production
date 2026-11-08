//! Lottie vector animation schema, importer, exporter, and frame evaluator.

pub mod eval;
pub mod schema;

pub use eval::{EvaluatedTransform, eval_layer_transform, eval_multi_prop, eval_scalar_prop};
pub use schema::{
    LottieAnimation, LottieLayer, LottiePropMulti, LottiePropScalar, LottieShape, LottieTransform,
};

/// Parse a Lottie JSON document string.
pub fn parse_lottie(json_str: &str) -> Result<LottieAnimation, String> {
    serde_json::from_str(json_str).map_err(|e| format!("Failed to parse Lottie JSON: {e}"))
}

/// Serialize a LottieAnimation structure to JSON string.
pub fn write_lottie(anim: &LottieAnimation) -> Result<String, String> {
    serde_json::to_string_pretty(anim).map_err(|e| format!("Failed to serialize Lottie: {e}"))
}
