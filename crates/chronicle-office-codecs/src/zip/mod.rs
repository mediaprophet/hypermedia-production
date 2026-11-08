//! Self-contained ZIP package reader and writer.

pub mod minizip;

pub use minizip::{crc32, ZipError, ZipPackage};
