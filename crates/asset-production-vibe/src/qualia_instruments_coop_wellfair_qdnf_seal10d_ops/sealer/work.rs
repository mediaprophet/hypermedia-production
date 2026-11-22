//! Present-only extraction from retained source-of-truth files.

use super::{fields, required, resource, Args, AssetHost, Value};
use std::collections::BTreeMap;
use work_graph::{extract, model::Honesty};

const INPUT_LIMIT: usize = 2 * 1024 * 1024;

pub(super) fn present(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["all_bound", "catalog"])?;
    let bound = required(params, "all_bound")?;
    let catalog = params.get("catalog");
    let bound_size = resource(host, bound, "rs")?.bytes.len();
    let catalog_size = catalog.map(|file| resource(host, file, "rs")).transpose()?
        .map_or(0, |file| file.bytes.len());
    let size = bound_size.saturating_add(catalog_size);
    if size == 0 || size > INPUT_LIMIT {
        return Err("Work graph source files are empty or exceed the extraction budget".into());
    }
    host.charge(size, size)?;
    let source = |input| -> Result<&str, String> {
        std::str::from_utf8(&resource(host, input, "rs")?.bytes)
            .map_err(|_| "Work graph source files must be UTF-8".into())
    };
    let bound = extract::extract_all_bound(source(bound)?).map_err(|error| error.to_string())?;
    let found = catalog.map(|file| extract::extract_catalog_ids(source(file)?)
        .map_err(|error| error.to_string())).transpose()?;
    let families = extract::families_from_ids(&bound).into_iter().map(|family| {
        Value::Record(BTreeMap::from([
            ("name".into(), Value::String(family.name)),
            ("count".into(), Value::U64(u64::from(family.member_count))),
        ]))
    }).collect();
    let strings = |strings: Vec<String>| Value::List(strings.into_iter().map(Value::String).collect());
    let mut values = BTreeMap::from([
        ("status".into(), Value::String(Honesty::presence_index().this_emit.into())),
        ("invoke_ids".into(), strings(bound)),
        ("families".into(), Value::List(families)),
    ]);
    if let Some(catalog) = found {
        values.insert("catalog_ids".into(), strings(catalog));
    }
    Ok(Value::Record(values))
}
