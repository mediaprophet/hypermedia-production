//! Bounded UNORM16 colour transport. RGBA8-only operators refuse these handles.
use crate::*;
use color_transforms::{GradeSpace, Transfer, WorkingSpace};

#[derive(Clone)]
pub struct Rgba16 {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u16; 4]>,
    pub transfer: Option<Transfer>,
}

impl AssetHost<'_> {
    pub(crate) fn store_rgba16(&mut self, width: u32, height: u32, pixels: Vec<[u16; 4]>, transfer: Option<Transfer>) -> Result<Value, String> {
        let n = (width as usize).checked_mul(height as usize).ok_or("RGBA16 dimensions overflow")?;
        if n != pixels.len() || n == 0 { return Err("RGBA16 pixel count mismatch".into()); }
        let mut handle = self.workspace.insert(Artifact::Rgba16(Rgba16 {width, height, pixels, transfer}), self.limits)?;
        if let Value::Record(record) = &mut handle {
            record.insert("bit_depth".into(), Value::U64(16));
            record.insert("transfer".into(), Value::String(transfer.map(transfer_name).unwrap_or("unspecified").into()));
        }
        Ok(handle)
    }

    pub(crate) fn rgba16(&self, value: &Value) -> Result<&Rgba16, String> {
        match self.workspace.get(value)? { Artifact::Rgba16(image) => Ok(image), _ => Err("Expected rgba16 handle".into()) }
    }

    pub(crate) fn encode_rgba16_png(&mut self, a: &Args) -> Result<Value, String> {
        let handle = required(a, "image")?;
        let image = self.rgba16(handle)?;
        let profile = crate::graphics::lattice_profile(a)?;
        if !profile.admits_16bit() { return Err("Named 8-bit profile refuses RGBA16; no implicit precision reduction".into()); }
        if image.width > profile.max_dim || image.height > profile.max_dim { return Err("RGBA16 PNG exceeds selected dimensions".into()); }
        let (w,h,n) = (image.width,image.height,image.pixels.len());
        if image.transfer.is_some_and(|t| !matches!(t, Transfer::Srgb | Transfer::Linear)) {
            return Err("PNG16 export currently preserves sRGB/linear transfer metadata only; convert explicitly before export".into());
        }
        self.charge(n * 12, n * 16 + 65536)?;
        let image = self.rgba16(handle)?;
        let mut bytes = Vec::new();
        let mut samples = Vec::with_capacity(n * 8);
        for pixel in &image.pixels { for sample in pixel { samples.extend_from_slice(&sample.to_be_bytes()); } }
        {
            let mut encoder = png::Encoder::new(&mut bytes,w,h);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Sixteen);
            match image.transfer {
                Some(Transfer::Srgb) => encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual),
                Some(Transfer::Linear) => encoder.set_source_gamma(png::ScaledFloat::new(1.0)),
                _ => {},
            }
            encoder.write_header().map_err(|e|e.to_string())?.write_image_data(&samples).map_err(|e|e.to_string())?;
        }
        self.encode("png",bytes)
    }

    pub(crate) fn grade_rgba16(&mut self, a: &Args) -> Result<Value, String> {
        let handle = required(a,"image")?;
        let image = self.rgba16(handle)?;
        let transfer = image.transfer.ok_or("Untagged RGBA16 grading requires explicit source_transfer via convert_transfer first")?;
        let (w,h,n) = (image.width,image.height,image.pixels.len());
        let working = match a.get("working_space").map(text).transpose()?.unwrap_or("rec709") {
            "rec709" => WorkingSpace::Rec709,"rec2100-pq" => WorkingSpace::Rec2100Pq,"rec2100-hlg" => WorkingSpace::Rec2100Hlg,
            _ => return Err("Unknown grade working_space".into()),
        };
        let white = number(a,"hdr_white_nits",1000.)? as f32;
        if !white.is_finite() || white <= 0. {return Err("hdr_white_nits must be positive".into());}
        let grade = GradeSpace::new(working,white);
        let lift = grade_vector(a,"lift",[0.;3],-1.,1.)?;
        let gamma = grade_vector(a,"gamma",[1.;3],0.1,4.)?;
        let gain = grade_vector(a,"gain",[1.;3],0.,4.)?;
        let saturation = bounded(a,"saturation",1.,0.,4.)?;
        let contrast = bounded(a,"contrast",1.,0.,4.)?;
        let pivot = bounded(a,"pivot",0.435,0.05,0.95)?;
        self.charge(n * 24,n * 8)?;
        let pixels = self.rgba16(handle)?.pixels.iter().map(|pixel| {
            let lin = [0,1,2].map(|c| color_transforms::to_linear(pixel[c] as f32 / 65535.,transfer));
            let mut signal = grade.encode(lin);
            for c in 0..3 {signal[c] = ((signal[c]*gain[c]+lift[c]).max(0.).powf(gamma[c])-pivot)*contrast+pivot;}
            let luma = color_transforms::luma709(signal[0],signal[1],signal[2]);
            for c in 0..3 {signal[c] = luma+(signal[c]-luma)*saturation;}
            let decoded = grade.decode(signal);
            [quantize(color_transforms::linear_to_srgb(decoded[0])),quantize(color_transforms::linear_to_srgb(decoded[1])),quantize(color_transforms::linear_to_srgb(decoded[2])),pixel[3]]
        }).collect();
        self.store_rgba16(w,h,pixels,Some(Transfer::Srgb))
    }

    pub(crate) fn transfer_rgba16(&mut self, a: &Args) -> Result<Value, String> {
        let handle = required(a,"image")?;
        let source = parse_transfer(text(required(a,"source_transfer")?)?)?;
        let target = parse_transfer(text(required(a,"target_transfer")?)?)?;
        let image = self.rgba16(handle)?;
        if image.transfer.is_some_and(|declared| transfer_name(declared) != transfer_name(source)) {
            return Err("source_transfer conflicts with retained image transfer metadata".into());
        }
        let (w,h,n) = (image.width,image.height,image.pixels.len());
        self.charge(n * 10,n * 8)?;
        let pixels = self.rgba16(handle)?.pixels.iter().map(|pixel| {
            if transfer_name(source) == transfer_name(target) {return *pixel;}
            let rgb = [0,1,2].map(|c| quantize(color_transforms::from_linear(color_transforms::to_linear(pixel[c] as f32/65535.,source),target)));
            [rgb[0],rgb[1],rgb[2],pixel[3]]
        }).collect();
        self.store_rgba16(w,h,pixels,Some(target))
    }
}

fn quantize(value:f32)->u16 {(value.clamp(0.,1.)*65535.).round() as u16}
pub(crate) fn transfer_name(value:Transfer)->&'static str {match value {Transfer::Srgb=>"srgb",Transfer::Bt709=>"bt709",Transfer::Linear=>"linear",Transfer::Pq=>"pq",Transfer::Hlg=>"hlg"}}
fn parse_transfer(value:&str)->Result<Transfer,String> {match value {"srgb"=>Ok(Transfer::Srgb),"bt709"=>Ok(Transfer::Bt709),"linear"=>Ok(Transfer::Linear),"pq"=>Ok(Transfer::Pq),"hlg"=>Ok(Transfer::Hlg),_=>Err("Unknown transfer".into())}}
fn bounded(a:&Args,key:&str,default:f64,min:f64,max:f64)->Result<f32,String> {let v=number(a,key,default)?;if !(min..=max).contains(&v){Err(format!("{key} outside {min}..={max}"))}else{Ok(v as f32)}}
fn grade_vector(a:&Args,key:&str,default:[f32;3],min:f64,max:f64)->Result<[f32;3],String> {
    let Some(value)=a.get(key) else{return Ok(default);};let v=vector::<3>(value)?;
    if v.iter().any(|value|!(min..=max).contains(value)){return Err(format!("{key} outside {min}..={max}"));}
    Ok(v.map(|value|value as f32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeSet,io::Cursor};
    #[test]
    fn declared_unsupported_png_profile_refuses_and_untagged_grade_requires_interpretation() {
        let limits=ToolLimits::default();let cancel=AtomicBool::new(false);
        let mut host=AssetHost{workspace:AssetWorkspace::default(),inputs:&[],limits:&limits,cancel:&cancel,work:0};
        let mut bytes=Vec::new();
        {
            let mut encoder=png::Encoder::new(&mut bytes,1,1);encoder.set_color(png::ColorType::Rgba);encoder.set_depth(png::BitDepth::Sixteen);
            encoder.set_source_gamma(png::ScaledFloat::new(0.7));
            encoder.write_header().unwrap().write_image_data(&[0x12,0x34,0x23,0x45,0x34,0x56,0xff,0xff]).unwrap();
        }
        let declared=png::Decoder::new(Cursor::new(&bytes)).read_info().unwrap();
        assert_eq!(declared.info().gama_chunk.expect("fixture contains a real gAMA chunk").into_value(),0.7);
        drop(declared);
        let encoded=host.admit_encoded("png",bytes).unwrap();
        let error=host.graphics("hypermedia.radiance.import_rgba_png",&Args::from([("input".into(),encoded)])).unwrap_err();
        assert!(error.contains("unsupported colour profile"));
        let untagged=host.store_rgba16(1,1,vec![[1234,2345,3456,65535]],None).unwrap();
        let error=host.radiance_color_op("hypermedia.radiance.grade_color",&Args::from([("image".into(),untagged.clone())])).unwrap_err();
        assert!(error.contains("source_transfer"));
        let declared=host.radiance_color_op("hypermedia.radiance.convert_transfer",&Args::from([("image".into(),untagged),("source_transfer".into(),Value::String("linear".into())),("target_transfer".into(),Value::String("linear".into()))])).unwrap();
        assert_eq!(host.rgba16(&declared).unwrap().transfer,Some(Transfer::Linear));
        assert_eq!(host.rgba16(&declared).unwrap().pixels[0],[1234,2345,3456,65535]);
    }
    #[test]
    fn imported_gradient_keeps_more_than_8bit_precision_through_grade_transfer_and_png_reload() {
        let limits=ToolLimits::default();let cancel=AtomicBool::new(false);
        let mut host=AssetHost{workspace:AssetWorkspace::default(),inputs:&[],limits:&limits,cancel:&cancel,work:0};
        let samples:Vec<[u16;4]>=(0..1024).map(|i|[i*63+1,i*63+2,i*63+3,54321]).collect();
        let original=host.store_rgba16(1024,1,samples.clone(),Some(Transfer::Srgb)).unwrap();
        let png=host.graphics("hypermedia.radiance.encode_rgba_png",&Args::from([("image".into(),original)])).unwrap();
        let imported=host.graphics("hypermedia.radiance.import_rgba_png",&Args::from([("input".into(),png)])).unwrap();
        assert_eq!(host.rgba16(&imported).unwrap().pixels,samples);
        assert!(host.rgba(&imported).is_err(),"RGBA8-only edits must refuse rather than quantize");
        let graded=host.radiance_color_op("hypermedia.radiance.grade_color",&Args::from([("image".into(),imported)])).unwrap();
        let linear=host.radiance_color_op("hypermedia.radiance.convert_transfer",&Args::from([("image".into(),graded),("source_transfer".into(),Value::String("srgb".into())),("target_transfer".into(),Value::String("linear".into()))])).unwrap();
        let encoded=host.graphics("hypermedia.radiance.encode_rgba_png",&Args::from([("image".into(),linear)])).unwrap();
        let bytes=&host.workspace.encoded(&encoded).unwrap().bytes;
        let mut reader=png::Decoder::new(Cursor::new(bytes)).read_info().unwrap();
        assert_eq!(reader.info().bit_depth,png::BitDepth::Sixteen);
        assert_eq!(reader.info().gama_chunk.expect("linear PNG must carry an explicit gAMA chunk").into_value(),1.0);
        let mut buffer=vec![0;reader.output_buffer_size().unwrap()];let info=reader.next_frame(&mut buffer).unwrap();
        let values:BTreeSet<_>=buffer[..info.buffer_size()].chunks_exact(8).map(|p|u16::from_be_bytes([p[0],p[1]])).collect();
        assert!(values.len()>900,"a real high-precision gradient must survive, got {} levels",values.len());
        assert!(buffer[..info.buffer_size()].chunks_exact(8).all(|p|u16::from_be_bytes([p[6],p[7]])==54321));
        drop(reader);
        let reloaded=host.graphics("hypermedia.radiance.import_rgba_png",&Args::from([("input".into(),encoded)])).unwrap();
        assert_eq!(host.rgba16(&reloaded).unwrap().transfer,Some(Transfer::Linear));
        let conflict=host.radiance_color_op("hypermedia.radiance.convert_transfer",&Args::from([("image".into(),reloaded),("source_transfer".into(),Value::String("srgb".into())),("target_transfer".into(),Value::String("linear".into()))])).unwrap_err();
        assert!(conflict.contains("retained image transfer metadata"));
    }
}
