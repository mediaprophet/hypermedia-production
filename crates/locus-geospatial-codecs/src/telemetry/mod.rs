//! Real-time spatial telemetry codecs and attribute query evaluator.

pub mod aircraft;
pub mod query;
pub mod vessels;

pub use aircraft::{AircraftMessage, CprFrame, decode_adsb_packet, decode_cpr_global};
pub use query::{AttributeValue, ComparisonOp, FilterExpr};
pub use vessels::{AisMessage, decode_aivdm_sentence};
