//! Lossless production PNG sequence with exact rational timing and leaf resources.
use crate::*;
use sha2::{Digest,Sha256};

/// The shared timeline owner normalizes rational rates; reject invalid values
/// before its compatibility fallback can substitute another rate.
pub(crate) fn frame_rate(a:&Args,default:u32,max:u32)->Result<timeline_editing::FrameRate,String> {
    let (num,den)=match a.get("fps") {
        None=>(default as i64,1),
        Some(Value::Record(rate))=>{
            fields(rate,&["numerator","denominator"])?;
            (i64::try_from(integer(required(rate,"numerator")?,"numerator")?).map_err(|_|"Frame rate overflow")?,
             i64::try_from(integer(required(rate,"denominator")?,"denominator")?).map_err(|_|"Frame rate overflow")?)
        },
        Some(value)=>(i64::try_from(integer(value,"fps")?).map_err(|_|"Frame rate overflow")?,1),
    };
    if num<=0||den<=0||num>240_000_000||den>1_000_000||num<den||num>den*max as i64 {
        return Err(format!("fps must be a positive rational rate in 1..={max}"));
    }
    Ok(timeline_editing::FrameRate::new(num,den))
}

impl AssetHost<'_> {
    pub(crate) fn capture_png_sequence(&mut self,a:&Args)->Result<Value,String> {
        let frames=list(required(a,"frames")?)?;
        if frames.is_empty()||frames.len()>64{return Err("PNG sequence requires 1..=64 frame handles".into());}
        let rate=frame_rate(a,24,240)?;
        let mut dimensions=None;
        let mut visits=0usize;
        let mut source_bits=None;
        let mut interpretation=None;
        for frame in frames {
            let (w,h,n,bits,transfer)=match self.workspace.get(frame)? {
                Artifact::Rgba(r)=>(r.width,r.height,r.pixels.len(),8u8,"unspecified"),
                Artifact::Rgba16(r)=>(r.width,r.height,r.pixels.len(),16u8,r.transfer.map(crate::radiance_precision::transfer_name).unwrap_or("unspecified")),
                _=>return Err("PNG sequence frames require retained RGBA8 or RGBA16 images".into()),
            };
            if w==0||h==0||w>2048||h>2048{return Err("PNG sequence dimensions exceed 2048 profile".into());}
            if dimensions.is_some_and(|dims|dims!=(w,h))||source_bits.is_some_and(|b|b!=bits){return Err("PNG sequence frames must have congruent dimensions and precision".into());}
            if interpretation.is_some_and(|value|value!=transfer){return Err("PNG sequence frames must have congruent retained transfer interpretation".into());}
            interpretation=Some(transfer);
            dimensions=Some((w,h));source_bits=Some(bits);
            visits=visits.checked_add(n).ok_or("Sequence pixel count overflow")?;
        }
        let bits=source_bits.unwrap();
        let raw_bytes=visits.checked_mul(bits as usize/2).ok_or("Sequence byte count overflow")?;
        if raw_bytes>self.limits.total_resource_bytes{return Err("PNG sequence exceeds aggregate pixel budget".into());}
        self.charge(visits*2,raw_bytes)?;
        let mut frame_records=Vec::new();
        let mut dependencies=Vec::new();
        let mut resources=Vec::new();
        for (index,frame) in frames.iter().enumerate() {
            let mut args=Args::new();args.insert("image".into(),frame.clone());
            let encoded=self.graphics("hypermedia.radiance.encode_rgba_png",&args)?;
            let resource=self.workspace.encoded(&encoded)?;
            let key=format!("frames/{index:06}.png");
            let record=serde_json::json!({"index":index,"path":key,"sha256":format!("{:x}",Sha256::digest(&resource.bytes)),"bytes":resource.bytes.len(),
                "start":{"numerator":index as i64*rate.den,"denominator":rate.num},
                "duration":{"numerator":rate.den,"denominator":rate.num},
                "start_tick":rate.tick_of(index as i64).0,"end_tick":rate.tick_of(index as i64+1).0,"transfer":interpretation.unwrap()});
            frame_records.push(record);
            dependencies.push(Value::Record(BTreeMap::from([("key".into(),Value::String(key.clone())),("resource".into(),encoded.clone())])));
            resources.push(Value::Record(BTreeMap::from([("key".into(),Value::String(key)),("resource".into(),encoded)])));
        }
        let mut audio_metadata=serde_json::Value::Null;
        if let Some(audio)=a.get("audio") {
            let (sample_rate,samples)=match self.workspace.get(audio)?{Artifact::Audio{rate,samples}=>(*rate,samples.len()),_=>return Err("Sequence companion audio requires an audio handle".into())};
            let expected=(frames.len() as u128*rate.den as u128*sample_rate as u128+rate.num as u128/2)/rate.num as u128;
            if samples as u128!=expected{return Err("Companion audio length must match the exact rational sequence duration, rounded to the nearest sample".into());}
            let encoded=self.sound("hypermedia.acoustics.encode_wav",&Args::from([("audio".into(),audio.clone())]))?;
            dependencies.push(Value::Record(BTreeMap::from([("key".into(),Value::String("audio/companion.wav".into())),("resource".into(),encoded)])));
            audio_metadata=serde_json::json!({"path":"audio/companion.wav","sample_rate":sample_rate,"samples":samples,"channels":1});
        }
        let (w,h)=dimensions.unwrap();
        let manifest=serde_json::json!({"schema":"synth.png-sequence/v1","profile":"png-sequence","delivery":"lossless image sequence; not a muxed movie",
            "width":w,"height":h,"bit_depth":bits,"transfer":interpretation.unwrap(),"alpha":"straight","frame_rate":{"numerator":rate.num,"denominator":rate.den},
            "clock":{"ticks_per_second":timeline_editing::TICKS_PER_SECOND,"boundary_rule":"ceil exact rational frame boundary to ticks; rational seconds remain authoritative"},
            "frames":frame_records,"audio":audio_metadata});
        let mut encoded=self.encode("json",serde_json::to_vec_pretty(&manifest).map_err(|e|e.to_string())?)?;
        if let Value::Record(handle)=&mut encoded{handle.insert("dependencies".into(),Value::List(dependencies));}
        Ok(Value::Record(BTreeMap::from([("profile".into(),Value::String("png-sequence".into())),("manifest_resource".into(),encoded),("frames".into(),Value::List(resources))])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_sequence_uses_exact_ntsc_timing_and_preserves_png16_resource_leaves() {
        let limits=ToolLimits::default();let cancel=AtomicBool::new(false);
        let mut host=AssetHost{workspace:AssetWorkspace::default(),inputs:&[],limits:&limits,cancel:&cancel,work:0};
        let image=host.store_rgba16(8,8,vec![[1234,2345,3456,65535];64],Some(color_transforms::Transfer::Linear)).unwrap();
        let rate=Value::Record(Args::from([("numerator".into(),Value::U64(24000)),("denominator".into(),Value::U64(1001))]));
        let args=Args::from([("frames".into(),Value::List(vec![image.clone(),image])),("fps".into(),rate),("profile".into(),Value::String("png-sequence".into()))]);
        let produced=host.acoustics_discourse_chronicle_ext_op("hypermedia.chronicle.capture_deterministic_movie",&args).unwrap();
        let output=record(&produced).unwrap();let manifest_handle=required(output,"manifest_resource").unwrap();
        let manifest:serde_json::Value=serde_json::from_slice(&host.workspace.encoded(manifest_handle).unwrap().bytes).unwrap();
        assert_eq!(manifest["frame_rate"]["numerator"],24000);assert_eq!(manifest["frame_rate"]["denominator"],1001);
        assert_eq!(manifest["bit_depth"],16);assert_eq!(manifest["frames"][1]["start"]["numerator"],1001);
        assert_eq!(manifest["transfer"],"linear");assert_eq!(manifest["frames"][0]["transfer"],"linear");
        let leaves=list(required(record(manifest_handle).unwrap(),"dependencies").unwrap()).unwrap();assert_eq!(leaves.len(),2);
        let first=record(&leaves[0]).unwrap();let png=host.workspace.encoded(required(first,"resource").unwrap()).unwrap();assert_eq!(png.bytes[24],16);
        let invalid=Args::from([("fps".into(),Value::Record(Args::from([("numerator".into(),Value::U64(24)),("denominator".into(),Value::U64(0))])))]);
        assert!(frame_rate(&invalid,24,240).is_err());
        let linear=host.store_rgba16(8,8,vec![[1234,2345,3456,65535];64],Some(color_transforms::Transfer::Linear)).unwrap();
        let srgb=host.store_rgba16(8,8,vec![[1234,2345,3456,65535];64],Some(color_transforms::Transfer::Srgb)).unwrap();
        let mixed=Args::from([("frames".into(),Value::List(vec![linear,srgb]))]);
        assert!(host.capture_png_sequence(&mixed).unwrap_err().contains("transfer"));
    }
}
