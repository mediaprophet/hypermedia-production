//! Bounded VibeScript source tooling and scientific evaluation.

use crate::{fields, list, num, record, required, Args, AssetHost, Resource, Value};
use ciborium::value::Value as Cbor;
use qualia_core_db::render::{
    texture_ktx2::{Ktx2ColorSpace, Ktx2Document, Ktx2Supercompression},
    texture_ktx2_rgba8::decode_ktx2_rgba8_base_level,
    texture_streaming_plan::{
        reserve_texture_mip, rgba8_ktx2_mip_candidate, select_texture_mip, TextureMipBackendCost,
        TextureMipBudget,
    },
    texture_transcode::{transcode_ktx2_into, TranscodeTargetFormat},
};
use std::{collections::{BTreeMap, BTreeSet}, fmt::Write};
use vibe::{
    ast_merge::{merge_programs, MergeConflict},
    bytecode::{compile, decode_chunk, encode_chunk, Const, Op},
    decompiler::{decompile_expr, decompile_program, DecompileOptions},
    physics::quantum_3d::evaluate_hydrogenic_orbital_3d,
    Item, Program,
};

const SOURCE_LIMIT: usize = 16 * 1024;
const BYTECODE_LIMIT: usize = 1024 * 1024;
const ITEM_LIMIT: usize = 256;
const ORBITAL_SAMPLE_LIMIT: usize = 4096;
const SYNTAX_DEPTH_LIMIT: usize = 48;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "decompile_vibe_ast" => decompile_ast(host, params),
        "compile_vibe_bytecode" => compile_bytecode(host, params),
        "decompile_vibe_bytecode" => decompile_bytecode(host, params),
        "merge_vibe_ast" => merge_ast(host, params),
        "hydrogenic_orbital" => orbital(host, params),
        "transcode_ktx2" => transcode_ktx2(host, params),
        "plan_texture_mip" => plan_texture_mip(host, params),
        "normalize_qudt_unit" => host.transcode_scientific_cae_bio_and_qudt(params),
        _ => Err(format!("Unsupported VibeScript or rendering tool operation: {op}")),
    }
}

fn resource<'a>(host: &'a AssetHost<'_>, input: &Value, kind: &str) -> Result<&'a Resource, String> {
    let resource = match input {
        Value::String(key) => host.inputs.iter().find(|entry| &entry.key == key)
            .ok_or_else(|| format!("Unknown retained {kind} resource {key}"))?,
        _ => host.workspace.encoded(input)?,
    };
    if resource.kind != kind || resource.bytes.len() > host.limits.resource_bytes {
        return Err(format!("Expected a bounded {kind} resource"));
    }
    Ok(resource)
}

fn source<'a>(host: &'a AssetHost<'_>, input: &Value) -> Result<&'a str, String> {
    let bytes = &resource(host, input, "vibe")?.bytes;
    if bytes.len() > SOURCE_LIMIT {
        return Err("Retained VibeScript exceeds the structural tooling source budget".into());
    }
    std::str::from_utf8(bytes).map_err(|_| "VibeScript source must be UTF-8".into())
}

fn parse_source(source: &str) -> Result<Program, String> {
    if source.len() > SOURCE_LIMIT {
        return Err("VibeScript source exceeds the structural tooling budget".into());
    }
    bound_source_syntax(source)?;
    let program = vibe::parse_program(source).map_err(|error| format!("{error:?}"))?;
    let preamble_count = program.locales.len() + program.prefixes.len() + program.imports.len()
        + program.requires.len();
    if program.items.len() + preamble_count > ITEM_LIMIT {
        return Err("VibeScript has too many items and declarations for structural tooling".into());
    }
    Ok(program)
}

fn bound_source_syntax(source: &str) -> Result<(), String> {
    let mut bytes = source.bytes().peekable();
    let mut depth = 0usize;
    let mut chain = 0usize;
    let mut previous = None;
    while let Some(byte) = bytes.next() {
        match (byte, bytes.peek().copied()) {
            (b'/', Some(b'/')) => {
                for byte in bytes.by_ref() {
                    if byte == b'\n' {
                        break;
                    }
                }
            }
            (b'/', Some(b'*')) => {
                bytes.next();
                while let Some(byte) = bytes.next() {
                    if byte == b'*' && bytes.peek() == Some(&b'/') {
                        bytes.next();
                        break;
                    }
                }
            }
            (b'"', _) => {
                let mut interpolation = 0;
                while let Some(literal) = bytes.next() {
                    match literal {
                        b'"' => break,
                        b'\\' => { bytes.next(); }
                        b'{' | b'(' | b'[' | b'<' | b'!' | b'?' | b'.' | b'-' | b'+' | b'*' | b'/' =>
                            interpolation += 1,
                        _ => {}
                    }
                }
                if previous == Some(b'f') && interpolation > SYNTAX_DEPTH_LIMIT {
                    return Err("VibeScript interpolated expression exceeds the syntax budget".into());
                }
            }
            (b'(' | b'[' | b'{', _) => depth += 1,
            (b')' | b']' | b'}', _) => depth = depth.saturating_sub(1),
            (b';', _) => chain = 0,
            (b'!' | b'?' | b'.' | b'-' | b'+' | b'*' | b'/' | b'%' | b'&' | b'|' | b'<' | b'>', _) =>
                chain += 1,
            _ => {}
        }
        if depth > SYNTAX_DEPTH_LIMIT || chain > SYNTAX_DEPTH_LIMIT * 2 {
            return Err("VibeScript expression exceeds the structural tooling syntax budget".into());
        }
        previous = Some(byte);
    }
    Ok(())
}

fn semantic_tree(program: &Program) -> Result<Cbor, String> {
    fn erase_spans(value: &mut Cbor) {
        match value {
            Cbor::Map(fields) => {
                fields.retain(|(key, _)| !matches!(key, Cbor::Text(name) if name == "span"));
                for (_, value) in fields {
                    erase_spans(value);
                }
            }
            Cbor::Array(items) => items.iter_mut().for_each(erase_spans),
            Cbor::Tag(_, child) => erase_spans(child),
            _ => {}
        }
    }

    let bytes = vibe::encode(program);
    let mut tree = ciborium::from_reader(bytes.as_slice()).map_err(|error| error.to_string())?;
    erase_spans(&mut tree);
    Ok(tree)
}

fn formatted(program: &Program) -> Result<String, String> {
    let options = DecompileOptions::default();
    // Keep declaration order aligned with the parser, and preserve capability arguments.
    let mut header = program.clone();
    header.items.clear();
    let prefixes = std::mem::take(&mut header.prefixes);
    let requires = std::mem::take(&mut header.requires);
    let imports = std::mem::take(&mut header.imports);
    let mut output = decompile_program(&header, &options);
    for import in imports {
        output.push_str("import ");
        write_source_string(&mut output, &import.path);
        if let Some(alias) = import.alias {
            let _ = write!(output, " as {alias}");
        }
        output.push_str(";\n");
    }
    header.locales.clear();
    header.module = None;
    header.prefixes = prefixes;
    output.push_str(&decompile_program(&header, &options));
    for requirement in requires {
        output.push_str("requires [ capability(");
        write_source_string(&mut output, &requirement.id);
        for arg in requirement.args {
            let _ = write!(output, ", {}: ", arg.name);
            decompile_expr(&mut output, &arg.value);
        }
        output.push_str(") ];\n");
    }
    header.prefixes.clear();
    header.items.clone_from(&program.items);
    output.push_str(&decompile_program(&header, &options));
    let roundtrip = parse_source(&output)?;
    if semantic_tree(program)? != semantic_tree(&roundtrip)? {
        return Err("VibeScript decompiler could not preserve the source AST".into());
    }
    Ok(output)
}

fn write_source_string(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if ch.is_control() => { let _ = write!(output, "\\u{{{:x}}}", u32::from(ch)); }
            ch => output.push(ch),
        }
    }
    output.push('"');
}

fn decompile_ast(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source"])?;
    let input = required(params, "source")?;
    let size = source(host, input)?.len();
    host.charge(size, size.saturating_mul(32))?;
    let program = parse_source(source(host, input)?)?;
    let formatted = formatted(&program)?;
    host.encode("vibe", formatted.into_bytes())
}

fn compile_bytecode(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source"])?;
    let input = required(params, "source")?;
    let size = source(host, input)?.len();
    host.charge(size, size.saturating_mul(16))?;
    let program = parse_source(source(host, input)?)?;
    let chunk = compile(&program).map_err(|error| error.to_string())?;
    let bytes = encode_chunk(&chunk);
    if bytes.len() > BYTECODE_LIMIT {
        return Err("Encoded VibeScript bytecode exceeds the structural tooling budget".into());
    }
    host.encode("vibe-bc", bytes)
}

fn decompile_bytecode(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["bytecode"])?;
    let input = required(params, "bytecode")?;
    let size = resource(host, input, "vibe-bc")?.bytes.len();
    if size > BYTECODE_LIMIT {
        return Err("VibeScript bytecode exceeds the structural tooling budget".into());
    }
    host.charge(size, size.saturating_mul(16))?;
    let bytes = &resource(host, input, "vibe-bc")?.bytes;
    let chunk = decode_chunk(bytes).map_err(|error| format!("{error:?}"))?;
    if encode_chunk(&chunk).as_slice() != bytes.as_slice() {
        return Err("VibeScript bytecode has a mismatched binary extent".into());
    }
    let mut output = format!("VBC1 locals={}\n", chunk.top_locals);
    for (index, constant) in chunk.constants.iter().enumerate() {
        let (kind, value) = match constant {
            Const::String(value) => ("string", value),
            Const::Iri(value) => ("iri", value),
        };
        let _ = writeln!(output, "const {index} {kind} {value:?}");
    }
    for (index, function) in chunk.functions.iter().enumerate() {
        let _ = writeln!(output, "fn {index} {:?} offset={:04x} parameters={} locals={} budget={}",
            function.name, function.code_offset, function.param_count, function.local_count,
            function.budget_steps);
    }
    let mut pc = 0;
    let mut starts = vec![false; chunk.code.len()];
    while let Some(&opcode) = chunk.code.get(pc) {
        let op = Op::from_byte(opcode).ok_or_else(|| format!("Unknown VibeScript opcode at {pc:#x}"))?;
        let end = pc + 1 + op.operand_size();
        let operands = chunk.code.get(pc + 1..end)
            .ok_or_else(|| format!("Truncated VibeScript operands at {pc:#x}"))?;
        starts[pc] = true;
        let _ = write!(output, "{pc:04x}  {op:?}");
        for operand in operands {
            let _ = write!(output, " {operand:02x}");
        }
        output.push('\n');
        pc = end;
    }
    if chunk.functions.iter().any(|function| !starts.get(function.code_offset as usize).copied().unwrap_or(false)) {
        return Err("VibeScript function offset does not identify an instruction".into());
    }
    host.encode("txt", output.into_bytes())
}

fn conflict_value(conflict: MergeConflict) -> Value {
    Value::Record(BTreeMap::from([
        ("path".into(), Value::String(conflict.path)),
        ("description".into(), Value::String(conflict.description)),
        ("ours".into(), Value::String(conflict.our_change)),
        ("theirs".into(), Value::String(conflict.their_change)),
    ]))
}

fn preamble(program: &Program) -> Program {
    let mut header = program.clone();
    header.items.clear();
    header
}

fn require_unique_merge_items(program: &Program) -> Result<(), String> {
    let mut keys = BTreeSet::new();
    let mut one = preamble(program);
    for item in &program.items {
        let key = match item {
            Item::Const(item) => format!("const:{}", item.name),
            Item::Function(item) => format!("fn:{}", item.name),
            Item::Hook(item) => format!("on:{}", item.path.join(".")),
            Item::Enum(item) => format!("enum:{}", item.name),
            Item::Law(item) => format!("law:{}", item.name),
            Item::Material(item) => format!("material:{}", item.name),
            Item::Field(item) => format!("field:{}", item.name),
            Item::Cell(item) => format!("cell:{}", item.name),
            Item::Present(item) => format!("present:{}", item.name),
            Item::Bind(_) | Item::Statement(_) => {
                one.items.push(item.clone());
                let key = format!("anonymous:{:?}", semantic_tree(&one)?);
                one.items.clear();
                key
            }
        };
        if !keys.insert(key) {
            return Err("VibeScript AST merge requires unique top-level item identities".into());
        }
    }
    Ok(())
}

fn merge_ast(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["base", "ours", "theirs"])?;
    let base = required(params, "base")?;
    let ours = required(params, "ours")?;
    let theirs = required(params, "theirs")?;
    let size = source(host, base)?.len() + source(host, ours)?.len() + source(host, theirs)?.len();
    host.charge(size.saturating_mul(ITEM_LIMIT), size.saturating_mul(48))?;
    let base = parse_source(source(host, base)?)?;
    let ours = parse_source(source(host, ours)?)?;
    let theirs = parse_source(source(host, theirs)?)?;
    for program in [&base, &ours, &theirs] {
        require_unique_merge_items(program)?;
    }
    let base_header = preamble(&base);
    let our_header = preamble(&ours);
    let their_header = preamble(&theirs);
    let [base_tree, our_tree, their_tree] = [
        semantic_tree(&base_header)?, semantic_tree(&our_header)?, semantic_tree(&their_header)?,
    ];
    let mut merged = merge_programs(&base, &ours, &theirs);
    let mut header = if base_tree == our_tree {
        their_header
    } else {
        if base_tree != their_tree && our_tree != their_tree {
            let options = DecompileOptions::default();
            merged.conflicts.push(MergeConflict {
                path: "preamble".into(),
                description: "Both branches changed VibeScript declarations".into(),
                our_change: decompile_program(&our_header, &options),
                their_change: decompile_program(&their_header, &options),
            });
        }
        our_header
    };
    let source = if merged.conflicts.is_empty() {
        header.items = merged.program.items;
        host.encode("vibe", formatted(&header)?.into_bytes())?
    } else {
        Value::Null
    };
    let conflicts = merged.conflicts.into_iter().map(conflict_value).collect();
    Ok(Value::Record(BTreeMap::from([
        ("source".into(), source),
        ("conflicts".into(), Value::List(conflicts)),
    ])))
}

fn integer(value: &Value) -> Result<i64, String> {
    match value {
        Value::I64(value) => Ok(*value),
        Value::U64(value) => i64::try_from(*value).map_err(|_| "Integer exceeds i64".into()),
        _ => Err("Expected an exact integer".into()),
    }
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected a nonnegative integer".into()),
        _ => Err("Expected a nonnegative integer".into()),
    }
}

fn u32_value(value: &Value) -> Result<u32, String> {
    u32::try_from(unsigned(value)?).map_err(|_| "Integer exceeds u32".into())
}

fn orbital(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["n", "l", "m", "z", "points_bohr"])?;
    let n = integer(required(params, "n")?)?;
    let l = integer(required(params, "l")?)?;
    let m = integer(required(params, "m")?)?;
    let z = num(required(params, "z")?)?;
    if !(1..=32).contains(&n) || !(0..n).contains(&l) || !(-l..=l).contains(&m)
        || !(0.0..=118.0).contains(&z) || z == 0.0
    {
        return Err("Hydrogenic orbital quantum numbers or effective nuclear charge are out of range".into());
    }
    let points = list(required(params, "points_bohr")?)?;
    if points.is_empty() || points.len() > ORBITAL_SAMPLE_LIMIT {
        return Err("Hydrogenic orbital sample count exceeds the tooling budget".into());
    }
    host.charge(points.len() * n as usize, points.len() * 128)?;
    let mut samples = Vec::with_capacity(points.len());
    for point in points {
        let [x, y, z_position] = list(point)? else {
            return Err("Orbital sampling expects a list of three-coordinate points in Bohr".into());
        };
        let point = [num(x)?, num(y)?, num(z_position)?];
        if point.iter().any(|coordinate| coordinate.abs() > 1e6) {
            return Err("Hydrogenic orbital coordinate exceeds the numerical domain".into());
        }
        let (psi, density, phase) = evaluate_hydrogenic_orbital_3d(n as u32, l as u32, m as i32, z, point);
        if !psi.is_finite() || !density.is_finite() {
            return Err("Hydrogenic orbital evaluation produced a non-finite result".into());
        }
        samples.push(Value::Record(BTreeMap::from([
            ("amplitude".into(), Value::F64(psi)),
            ("probability_density".into(), Value::F64(density)),
            ("phase_sign".into(), Value::I64(i64::from(phase))),
        ])));
    }
    Ok(Value::List(samples))
}

fn valid_compressed_block(vk_format: u32, block: &[u8]) -> bool {
    match vk_format {
        145 | 146 => block[0] != 0,
        151 | 152 => {
            // The ETC decoder supports individual and differential RGB encoding.
            // Differential overflow is the selector for the other ETC2 RGB modes.
            let differential = block[11] & 2 != 0;
            block[1] >> 4 != 0 && (!differential || block[8..11].iter().all(|&component| {
                let base = i16::from(component >> 3);
                let delta = i16::from(((component & 7) as i8) << 5 >> 5);
                (0..=31).contains(&(base + delta))
            }))
        }
        157 | 158 => block[..8] == [0xfc, 0xfd, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        _ => false,
    }
}

fn transcode_ktx2(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["ktx2"])?;
    let input = required(params, "ktx2")?;
    let size = resource(host, input, "ktx2")?.bytes.len();
    host.charge(size, 0)?;
    let bytes = &resource(host, input, "ktx2")?.bytes;
    let doc = Ktx2Document::parse(bytes).map_err(|error| format!("Invalid KTX2: {error:?}"))?;
    let (width, height) = (doc.pixel_width, doc.pixel_height);
    if width == 0 || height == 0 || width > 16_384 || height > 16_384 || doc.pixel_depth != 0
        || doc.layer_count() != 1 || doc.face_count != 1 || doc.type_size != 1
        || doc.supercompression() != Ktx2Supercompression::None
    {
        return Err("KTX2 transcode requires a bounded, single-face, unsupercompressed 2D image".into());
    }
    let bytes_out = (width as usize).checked_mul(height as usize).and_then(|n| n.checked_mul(4))
        .filter(|&n| n <= host.limits.resource_bytes).ok_or("KTX2 RGBA8 output exceeds resource budget")?;
    let raw_rgba = matches!(doc.vk_format, 37 | 43);
    if !raw_rgba {
        let level = doc.level(0).ok_or("KTX2 container is missing its base level")?;
        let blocks = width.div_ceil(4) as usize * height.div_ceil(4) as usize;
        if level.bytes.len() != blocks * 16
            || !level.bytes.chunks_exact(16).all(|block| valid_compressed_block(doc.vk_format, block))
        {
            return Err("Unsupported or invalid BC7, ETC2/EAC or ASTC block encoding".into());
        }
    }
    let color_space = match doc.color_space() {
        Ktx2ColorSpace::Linear => "linear",
        Ktx2ColorSpace::Srgb => "srgb",
        Ktx2ColorSpace::Unknown => return Err("KTX2 source transfer function is unavailable".into()),
    };
    host.charge(bytes_out, bytes_out)?;
    let bytes = &resource(host, input, "ktx2")?.bytes;
    let mut output = vec![0; bytes_out];
    if raw_rgba {
        let doc = Ktx2Document::parse(bytes).map_err(|error| format!("Invalid KTX2: {error:?}"))?;
        decode_ktx2_rgba8_base_level(&doc, &mut output)
            .map_err(|error| format!("RGBA8 KTX2 decode failed: {error:?}"))?;
    } else {
        transcode_ktx2_into(bytes, TranscodeTargetFormat::Rgba8, &mut output, &mut [])
            .map_err(|error| format!("Compressed KTX2 decode failed: {error:?}"))?;
    }
    Ok(Value::Record(BTreeMap::from([
        ("rgba8".into(), host.encode("rgba8", output)?),
        ("width".into(), Value::U64(u64::from(width))),
        ("height".into(), Value::U64(u64::from(height))),
        ("color_space".into(), Value::String(color_space.into())),
    ])))
}

fn plan_texture_mip(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["ktx2", "backend_costs", "budget", "projected_width", "projected_height"])?;
    let input = required(params, "ktx2")?;
    let dimensions = [
        u32_value(required(params, "projected_width")?)?,
        u32_value(required(params, "projected_height")?)?,
    ];
    if dimensions.contains(&0) {
        return Err("Mip residency planning requires a positive projection footprint".into());
    }
    let budget = record(required(params, "budget")?)?;
    fields(budget, &["decoded_cpu_bytes", "gpu_resident_bytes", "upload_staging_bytes", "per_frame_upload_bytes"])?;
    let mut budget = TextureMipBudget {
        decoded_cpu_bytes: unsigned(required(budget, "decoded_cpu_bytes")?)?,
        gpu_resident_bytes: unsigned(required(budget, "gpu_resident_bytes")?)?,
        upload_staging_bytes: unsigned(required(budget, "upload_staging_bytes")?)?,
        per_frame_upload_bytes: unsigned(required(budget, "per_frame_upload_bytes")?)?,
    };
    let size = resource(host, input, "ktx2")?.bytes.len();
    host.charge(size, 0)?;
    let bytes = &resource(host, input, "ktx2")?.bytes;
    let doc = Ktx2Document::parse(bytes).map_err(|error| format!("Invalid KTX2: {error:?}"))?;
    let costs = list(required(params, "backend_costs")?)?;
    if costs.len() != doc.level_count() || costs.len() > 32 {
        return Err("Provide one backend allocation cost for each bounded KTX2 mip level".into());
    }
    let mut candidates = Vec::with_capacity(costs.len());
    for (level, cost) in costs.iter().enumerate() {
        let cost = record(cost)?;
        fields(cost, &["gpu_resident_bytes", "upload_staging_bytes"])?;
        let cost = TextureMipBackendCost {
            gpu_resident_bytes: unsigned(required(cost, "gpu_resident_bytes")?)?,
            upload_staging_bytes: unsigned(required(cost, "upload_staging_bytes")?)?,
        };
        let candidate = rgba8_ktx2_mip_candidate(&doc, level, cost)
            .map_err(|error| format!("Invalid KTX2 mip candidate: {error:?}"))?;
        candidates.push(candidate);
    }
    let Some(selected) = select_texture_mip(&candidates, dimensions[0], dimensions[1], budget) else {
        return Ok(Value::Null);
    };
    let candidate = selected.candidate;
    if !reserve_texture_mip(candidate, &mut budget) {
        return Err("Selected mip exceeds the residency budget".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("level".into(), Value::U64(u64::from(candidate.level))),
        ("width".into(), Value::U64(u64::from(candidate.width))),
        ("height".into(), Value::U64(u64::from(candidate.height))),
        ("is_srgb".into(), Value::Bool(candidate.is_srgb)),
        ("meets_footprint".into(), Value::Bool(selected.meets_footprint)),
        ("remaining_budget".into(), Value::Record(BTreeMap::from([
            ("decoded_cpu_bytes".into(), Value::U64(budget.decoded_cpu_bytes)),
            ("gpu_resident_bytes".into(), Value::U64(budget.gpu_resident_bytes)),
            ("upload_staging_bytes".into(), Value::U64(budget.upload_staging_bytes)),
            ("per_frame_upload_bytes".into(), Value::U64(budget.per_frame_upload_bytes)),
        ]))),
    ])))
}
