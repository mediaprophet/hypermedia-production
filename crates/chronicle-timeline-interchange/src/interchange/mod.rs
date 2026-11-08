//! Timeline interchange formats (EDL, FCP7 XML, FCPX XML, OTIO, AAF).

pub mod aaf;
pub mod edl;
pub mod fcp7;
pub mod fcpxml;
pub mod otio;

pub use aaf::AafDocument;
pub use edl::{parse_edl, write_edl};
pub use fcp7::{parse_fcp7_xml, write_fcp7_xml};
pub use fcpxml::{parse_fcpxml, write_fcpxml};
pub use otio::{parse_otio, write_otio};
