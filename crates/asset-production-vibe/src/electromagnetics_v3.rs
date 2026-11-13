//! Bounded typed v3 interpretation and original PCM-f32LE input admission.
use crate::*;
use qualia_core_db::q42::emf_container::*;

pub(crate) struct OwnedEmfAudio { pub spec:EmfAudioTrackSpec,pub samples:Vec<f32> }
pub(crate) struct EmfV3Request { pub field:EmfFieldInterpretation,pub tracks:Vec<OwnedEmfAudio> }

const METADATA_BYTES:usize=64*1024;

/// Walk borrowed input first. Account strings/keys at worst-case JSON escaping,
/// Value storage and map-node overhead before cloning any JSON/model values.
fn metadata_cost(value:&Value,depth:usize,nodes:&mut usize,bytes:&mut usize)->Result<(),String>{
    *nodes+=1;
    if depth>16||*nodes>8192{return Err("EMF metadata exceeds nesting/node budget".into());}
    *bytes=bytes.checked_add(std::mem::size_of::<serde_json::Value>()).ok_or("Metadata size overflow")?;
    match value {
        Value::Null|Value::Bool(_)|Value::I64(_)|Value::U64(_)=>{},
        Value::F64(v) if v.is_finite()=>{},
        Value::String(v) if v.len()<=4096=>{*bytes=bytes.checked_add(v.len().checked_mul(6).ok_or("Metadata size overflow")?).ok_or("Metadata size overflow")?;},
        Value::List(items)=>for item in items {metadata_cost(item,depth+1,nodes,bytes)?;},
        Value::Record(items)=>for (key,item) in items {
            if key.len()>128{return Err("EMF metadata key exceeds budget".into());}
            *bytes=bytes.checked_add(key.len()*6+128).ok_or("Metadata size overflow")?;
            metadata_cost(item,depth+1,nodes,bytes)?;
        },
        _=>return Err("EMF metadata requires bounded finite JSON-like values".into()),
    }
    if *bytes>METADATA_BYTES{return Err("EMF aggregate metadata exceeds 64 KiB admission".into());}
    Ok(())
}
fn metadata_json(value:&Value)->serde_json::Value {
    match value {
        Value::Null=>serde_json::Value::Null,Value::Bool(v)=>serde_json::json!(v),
        Value::I64(v)=>serde_json::json!(v),Value::U64(v)=>serde_json::json!(v),Value::F64(v)=>serde_json::json!(v),
        Value::String(v)=>serde_json::Value::String(v.clone()),
        Value::List(items)=>serde_json::Value::Array(items.iter().map(metadata_json).collect()),
        Value::Record(items)=>serde_json::Value::Object(items.iter().map(|(k,v)|(k.clone(),metadata_json(v))).collect()),
        _=>unreachable!("metadata was admitted before cloning"),
    }
}
fn typed_metadata_json(value:&Value)->Result<serde_json::Value,String>{
    metadata_cost(value,0,&mut 0,&mut 0)?;Ok(metadata_json(value))
}

enum AudioSource<'a>{Audio(&'a Value),Retained(usize)}
struct PlannedAudio<'a>{spec:EmfAudioTrackSpec,source:AudioSource<'a>,bytes:usize}
fn align64(bytes:usize)->Result<usize,String>{bytes.checked_add(63).map(|n|n&!63).ok_or("EMF size overflow".into())}

/// Serializing to a counter allocates no JSON buffer. Maximal three-digit digest
/// bytes bound the actual metadata encoding without reserving a full 64 KiB file.
fn metadata_output_bound(field:&EmfFieldInterpretation,plans:&[PlannedAudio<'_>])->Result<(usize,usize),String>{
    struct Counter(usize);
    impl std::io::Write for Counter {
        fn write(&mut self,bytes:&[u8])->std::io::Result<usize>{
            self.0=self.0.checked_add(bytes.len()).filter(|n|*n<=METADATA_BYTES)
                .ok_or_else(||std::io::Error::new(std::io::ErrorKind::InvalidInput,"EMF metadata exceeds 64 KiB"))?;
            Ok(bytes.len())
        }
        fn flush(&mut self)->std::io::Result<()>{Ok(())}
    }
    let mut offset=0usize;let mut descriptors=Vec::with_capacity(plans.len());
    for plan in plans {
        descriptors.push(EmfAudioTrackDescriptor{codec:EmfAudioCodec::PcmF32Le,spec:plan.spec.clone(),
            frames:(plan.bytes/4/usize::from(plan.spec.channels)) as u64,pcm_offset:offset as u64,
            pcm_bytes:plan.bytes as u64,blake3:[255;32]});
        offset=align64(offset.checked_add(plan.bytes).ok_or("EMF size overflow")?)?;
    }
    let metadata=EmfExtensionMetadata{schema_version:EMF_EXTENSION_SCHEMA_VERSION,field:field.clone(),audio_tracks:descriptors};
    let mut counter=Counter(0);serde_json::to_writer(&mut counter,&metadata).map_err(|e|e.to_string())?;
    Ok((counter.0,offset))
}

pub(crate) fn emf_metadata_value(reader:&EmfContainerReader<'_>)->Result<Value,String>{
    fn convert(value:serde_json::Value)->Result<Value,String>{
        Ok(match value {
            serde_json::Value::Null=>Value::Null,serde_json::Value::Bool(v)=>Value::Bool(v),
            serde_json::Value::String(v)=>Value::String(v),
            serde_json::Value::Number(v)=>if let Some(n)=v.as_i64(){Value::I64(n)}else if let Some(n)=v.as_u64(){Value::U64(n)}else{Value::F64(v.as_f64().ok_or("Invalid metadata number")?)},
            serde_json::Value::Array(items)=>Value::List(items.into_iter().map(convert).collect::<Result<_,_>>()?),
            serde_json::Value::Object(items)=>Value::Record(items.into_iter().map(|(k,v)|Ok((k,convert(v)?))).collect::<Result<_,String>>()?),
        })
    }
    match reader.extension(){Some(extension)=>convert(serde_json::to_value(extension.metadata()).map_err(|e|e.to_string())?),None=>Ok(Value::Null)}
}

impl AssetHost<'_> {
    /// This local admission includes fixed retained inputs and workspace payloads,
    /// unlike the generic charge() scratch check. It is a logical payload bound.
    pub(crate) fn emf_peak_admit(&self,scratch:usize)->Result<(),String>{
        let fixed=self.inputs.iter().try_fold(self.workspace.bytes,|n,input|n.checked_add(input.bytes.len()))
            .ok_or("EMF fixed payload size overflow")?;
        let peak=fixed.checked_add(scratch).ok_or("EMF peak size overflow")?;
        if self.cancel.load(std::sync::atomic::Ordering::Acquire){return Err("Production cancelled".into());}
        if peak>self.limits.total_resource_bytes||self.work>self.limits.asset_work_units {
            return Err("EMF combined payload peak exceeds budget".into());
        }
        Ok(())
    }

    pub(crate) fn emf_v3_request(&mut self,a:&Args,num_bricks:usize)->Result<Option<EmfV3Request>,String>{
        let Some(interpretation)=a.get("interpretation") else {
            if a.contains_key("audio_tracks"){return Err("EMF audio_tracks require explicit v3 interpretation".into());}
            return Ok(None);
        };
        if self.workspace.entries.len()>=self.limits.resources{return Err("EMF output requires an available resource slot".into());}
        // Admit the aggregate metadata while it is still borrowed from arguments.
        let mut nodes=0;let mut metadata_bytes=0;metadata_cost(interpretation,0,&mut nodes,&mut metadata_bytes)?;
        if let Some(tracks)=a.get("audio_tracks"){metadata_cost(tracks,0,&mut nodes,&mut metadata_bytes)?;}
        let metadata_scratch=metadata_bytes.checked_mul(4).ok_or("Metadata size overflow")?;
        self.emf_peak_admit(metadata_scratch)?;
        let field:EmfFieldInterpretation=serde_json::from_value(typed_metadata_json(interpretation)?).map_err(|e|format!("Invalid EMF interpretation: {e}"))?;
        field.validate().map_err(|e|e.to_string())?;
        if field.spectral_semantics!=EmfSpectralSemantics::AmplitudeCentroidLegacySigma {
            return Err("compile_emf requires amplitude_centroid_legacy_sigma: its emitters collapse to a wavelength centroid, not a full spectrum".into());
        }
        let values:&[Value]=match a.get("audio_tracks"){Some(value)=>list(value)?,None=>&[]};
        if values.len()>16{return Err("EMF supports at most 16 PCM tracks per extension".into());}
        let mut plans=Vec::with_capacity(values.len());let mut total=0usize;let mut ids=std::collections::BTreeSet::new();
        // First pass validates source descriptors and all sizes without allocating PCM copies.
        for (index,value) in values.iter().enumerate(){
            let item=record(value)?;fields(item,&["spec","audio","input"])?;
            let spec:EmfAudioTrackSpec=serde_json::from_value(typed_metadata_json(required(item,"spec")?)?).map_err(|e|format!("Invalid PCM track spec: {e}"))?;
            spec.validate(index).map_err(|e|e.to_string())?;
            if !ids.insert(spec.id.clone()){return Err("Duplicate EMF audio track ID".into());}
            let (source,bytes)=match (item.get("audio"),item.get("input")) {
                (Some(audio),None)=>{
                    let Artifact::Audio{rate,samples}=self.workspace.get(audio)? else{return Err("EMF track audio requires an existing audio handle".into());};
                    if spec.channels!=1||spec.sample_rate_num!=u64::from(*rate).checked_mul(spec.sample_rate_den).ok_or("Audio rate overflow")? {
                        return Err("EMF track descriptor contradicts the source mono audio handle rate/channels".into());
                    }
                    (AudioSource::Audio(audio),samples.len().checked_mul(4).ok_or("Audio size overflow")?)
                },
                (None,Some(input))=>{
                    let key=text(input)?;
                    let index=self.inputs.iter().position(|r|r.key==key).ok_or("Missing retained PCM input")?;
                    let resource=&self.inputs[index];
                    if !matches!(resource.kind.as_str(),"pcm_f32le"|"pcm-f32le")||resource.bytes.len()%4!=0 {
                        return Err("EMF raw track input requires original pcm_f32le bytes".into());
                    }
                    (AudioSource::Retained(index),resource.bytes.len())
                },
                _=>return Err("EMF track requires exactly one audio handle or retained PCM input".into()),
            };
            if bytes==0||(bytes/4)%usize::from(spec.channels)!=0{return Err("EMF PCM frames/channels must agree".into());}
            total=total.checked_add(bytes).ok_or("Audio size overflow")?;
            if total>self.limits.resource_bytes{return Err("EMF audio bytes exceed selected resource budget".into());}
            plans.push(PlannedAudio{spec,source,bytes});
        }
        let (json_bytes,pcm_padded)=metadata_output_bound(&field,&plans)?;
        let cell_bytes=num_bricks.checked_mul(512*64).ok_or("EMF field size overflow")?;
        let field_bytes=num_bricks.checked_mul(512*64+128).and_then(|n|n.checked_add(64)).ok_or("EMF field size overflow")?;
        let output_bytes=field_bytes.checked_add(align64(json_bytes)?).and_then(|n|n.checked_add(pcm_padded))
            .and_then(|n|n.checked_add(128)).ok_or("EMF output size overflow")?;
        if output_bytes>self.limits.resource_bytes{return Err("EMF serialized output exceeds selected resource budget".into());}
        // Original staged cells, writer cell copies, all owned PCM, final encoded
        // output and bounded metadata/descriptor/readback scratch coexist here.
        let scratch=cell_bytes.checked_mul(2).and_then(|n|n.checked_add(total)).and_then(|n|n.checked_add(output_bytes))
            .and_then(|n|n.checked_add(metadata_scratch)).and_then(|n|n.checked_add(json_bytes.checked_mul(2)?))
            .and_then(|n|n.checked_add(num_bricks.checked_mul(128)?)).ok_or("EMF peak size overflow")?;
        self.emf_peak_admit(scratch)?;
        let mut tracks=Vec::with_capacity(plans.len());
        for plan in plans {
            self.charge(plan.bytes/2,0)?; // finite/unit scan plus copy/decode
            let normalized=matches!(plan.spec.units,EmfSignalUnits::NormalizedAmplitude);
            let valid=|s:f32|s.is_finite()&&(!normalized||s.abs()<=1.0);
            let samples=match plan.source {
                AudioSource::Audio(audio)=>{
                    let Artifact::Audio{samples,..}=self.workspace.get(audio)? else{return Err("Audio handle changed".into());};
                    if samples.iter().copied().any(|sample|!valid(sample)){return Err("Invalid original EMF PCM sample/units".into());}
                    samples.clone()
                },
                AudioSource::Retained(index)=>self.inputs[index].bytes.chunks_exact(4).map(|b|{
                    let sample=f32::from_le_bytes([b[0],b[1],b[2],b[3]]);
                    if valid(sample){Ok(sample)}else{Err("Invalid original EMF PCM sample/units".to_string())}
                }).collect::<Result<Vec<_>,_>>()?,
            };
            tracks.push(OwnedEmfAudio{spec:plan.spec,samples});
        }
        Ok(Some(EmfV3Request{field,tracks}))
    }

    /// Borrowed envelope sizing only; the core reader remains the integrity owner.
    /// Its bounded typed parse and verification record can coexist with the clone.
    pub(crate) fn emf_read_required(&self,value:&Value)->Result<usize,String>{
        let bytes=self.emf_source_bytes(value)?;
        if bytes.len()>self.limits.resource_bytes{return Err("EMF source exceeds selected resource bytes before copy".into());}
        if bytes.len()<64{return Err("EMF source is shorter than its header".into());}
        let chunks=u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
        if chunks>4096{return Err("EMF source exceeds chunk admission before copy".into());}
        let version=u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let parse=match version {
            2=>4096,
            3=>{
                if bytes.len()<192{return Err("EMF v3 source is shorter than its extension envelope".into());}
                let footer=bytes.len()-64;
                let json=usize::try_from(u64::from_le_bytes(bytes[footer+16..footer+24].try_into().unwrap())).map_err(|_|"EMF metadata length overflow")?;
                if json>METADATA_BYTES{return Err("EMF metadata exceeds 64 KiB admission before copy".into());}
                // JSON tree + typed parse + converted Vibe report, known track
                // descriptors, and canonical-directory uniqueness set. This is
                // deliberately conservative logical scratch, not an OS guarantee.
                json.checked_mul(32).and_then(|n|n.checked_add(16*1024))
                    .and_then(|n|n.checked_add(chunks.checked_mul(128)?)).ok_or("EMF parse size overflow")?
            },
            _=>return Err("EMF production reader requires current version 2 or 3".into()),
        };
        bytes.len().checked_add(parse).ok_or("EMF read size overflow".into())
    }

    pub(crate) fn parse_emf_current<'b>(&self,bytes:&'b[u8])->Result<EmfContainerReader<'b>,String>{
        EmfContainerReader::parse_with_extension_limits(bytes,EmfReadLimits{max_file_bytes:self.limits.resource_bytes,max_chunks:4096},
            EmfExtensionLimits{max_file_bytes:self.limits.resource_bytes,max_audio_bytes:self.limits.resource_bytes,max_tracks:16,max_metadata_bytes:64*1024})
            .map_err(|e|format!("Invalid current EMF container: {e}"))
    }
}

/// Explicit slice/raymarch operations are a derived human-visible preview only.
/// A declared RGB view needs its actual gamut/transfer/luminance conversion owner.
pub(crate) fn require_legacy_visible_projection(reader:&EmfContainerReader<'_>)->Result<(),String>{
    if reader.extension().is_some_and(|e|!e.metadata().field.supports_legacy_visible_projection()) {
        return Err("EMF v3 declares an RGB interpretation unsupported by this fixed legacy visible projection; metadata and PCM remain available through verify_emf".into());
    }
    Ok(())
}

/// Time slice zero is valid; the generic positive count parser cannot be used.
pub(crate) fn selected_time_slice(a:&Args)->Result<u32,String>{
    let selected=a.get("time_slice").map(|value|integer(value,"time_slice")).unwrap_or(Ok(0))?;
    u32::try_from(selected).map_err(|_|"time_slice exceeds u32".into())
}

#[cfg(test)]
mod tests;
