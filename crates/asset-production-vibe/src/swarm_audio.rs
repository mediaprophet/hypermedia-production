//! Bounded audio production adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod inverse;
mod master;
mod score;
mod sf2;
mod tract;

use crate::{AssetHost, Resource, Value};

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    if let Value::String(key) = value {
        return host
            .inputs
            .iter()
            .find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained audio resource {key}"));
    }
    host.workspace.encoded(value)
}
