//! Native PCM device lifecycle; source, decoding, scheduling and authority retain
//! their existing QualiaDB/host owners. No application-shell dependency.
mod device;
mod queue;
pub use device::DeviceOutput;
