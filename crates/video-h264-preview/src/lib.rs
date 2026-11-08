//! Bounded, silent, P/I/B-frame MP4/H.264 preview using the existing QualiaDB MP4
//! demux dependency and H.264/color operations. This is an adapter,
//! not a project/session, HMC format, media authority, or timeline clock.
//! Informed by open standards and Informatics Fiduciary guarantees under UDHR Articles 19 and 27.
use std::{borrow::Cow, io::Cursor, sync::atomic::{AtomicBool, Ordering}};

pub mod convert;
pub mod movie;
pub mod synth;

pub use movie::{fit_size, Movie, MovieError, MovieInfo, SharedSource};
pub use synth::{frame_color, h264_mp4, h264_mp4_with, prores_mov, test_pattern_mp4};

pub const TICKS_PER_SECOND:u64=254_016_000_000;
const MAX_INPUT:usize=2*1024*1024;
const MAX_DIM:u32=256;
const MAX_SAMPLES:u32=512;
const MAX_NALS:usize=256;
const MAX_SLICES:usize=8;
const MAX_OUTPUT:usize=8*1024*1024;
const MAX_GAP_FRAMES:u32=16;
#[derive(Default)] struct GapBudget {prev_ref:u32, generated:u32}
impl GapBudget {
    /// Mirrors Decoder::start_picture and submit_pending, including a gap on
    /// a non-reference picture and the reference reset performed by MMCO5.
    fn admit(&mut self,frame:u32,max:u32,idr:bool,reference:bool,mmco5:bool)->Result<(),String>{
        if max==0||frame>=max{return Err("Invalid H.264 frame number".into());}
        if idr {self.prev_ref=0;} else {
            let next=(self.prev_ref+1)%max;
            if frame!=self.prev_ref && frame!=next {
                let gap=(frame+max-next)%max;
                self.generated=self.generated.checked_add(gap).filter(|v|*v<=MAX_GAP_FRAMES).ok_or("H.264 gap frame work budget exceeded")?;
                self.prev_ref=(frame+max-1)%max;
            }
        }
        if reference {self.prev_ref=if mmco5{0}else{frame};}
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub duration_ticks: u64,
    pub rgba: Vec<u8>,
    pub index: usize,
    pub time: f64,
}

impl Frame {
    pub fn new(width: u32, height: u32, duration_ticks: u64, rgba: Vec<u8>) -> Self {
        Self { width, height, duration_ticks, rgba, index: 0, time: 0.0 }
    }
    pub fn fit(&self, max_w: u32, max_h: u32) -> Frame {
        let (w, h) = movie::fit_size(self.width, self.height, max_w, max_h);
        if (w, h) == (self.width, self.height) {
            return self.clone();
        }
        Frame {
            width: w,
            height: h,
            duration_ticks: self.duration_ticks,
            rgba: convert::scale_rgba(&self.rgba, self.width as usize, self.height as usize, w as usize, h as usize),
            index: self.index,
            time: self.time,
        }
    }
}

/// Decode a single traversal of the selected media range. Caller must supply a
/// source-authorized, digest-verified resource and recheck its revision before
/// publication. Audio is ignored; no playback clock or device is owned here.
pub fn decode_mp4(bytes:&[u8],start_tick:u64,remaining_ticks:u64,cancelled:&AtomicBool)->Result<Vec<Frame>,String>{
    let mut frames=Vec::new();let mut retained=0usize;
    decode_mp4_stream(bytes,start_tick,remaining_ticks,cancelled,|frame|{
        retained=retained.checked_add(frame.rgba.len()).ok_or("Video frame budget overflow")?;
        if retained>MAX_OUTPUT{return Err("Video frame budget exceeded".into());}
        frames.push(frame);Ok(())
    })?;
    Ok(frames)
}
/// Decode to a caller-owned, backpressured sink without retaining the whole
/// clip. The caller must recheck source authority when publishing each frame;
/// an error after earlier frames have been delivered terminates the stream.
pub fn decode_mp4_stream(bytes:&[u8],start_tick:u64,remaining_ticks:u64,cancelled:&AtomicBool,mut emit:impl FnMut(Frame)->Result<(),String>)->Result<(),String>{
    if bytes.is_empty()||bytes.len()>MAX_INPUT {return Err("MP4 input exceeds 2 MiB preview profile".into());}
    if remaining_ticks==0||remaining_ticks>TICKS_PER_SECOND*15 {return Err("MP4 clip remainder exceeds 15 seconds".into());}
    let projected=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||project_video_container(bytes)))
        .map_err(|_|"Combined MP4 demux rejected malformed input")??;
    let bytes=projected.as_ref();
    let mdat=check_top_level(bytes)?;
    // The structural preflight refuses unqualified video colour, aspect and
    // aperture boxes; their names inside opaque sample bytes are harmless.
    let end_tick=start_tick.checked_add(remaining_ticks).ok_or("Media range overflow")?;
    // The upstream parser is fallible but contains unchecked arithmetic in a
    // few malformed bitstream paths; confine a panic to a rejected preview.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(||decode_inner(bytes,mdat,start_tick,end_tick,cancelled,&mut emit)))
        .map_err(|_|"MP4/H.264 codec rejected malformed input")?
}
/// Select one video track without changing source bytes or moving media. For a
/// two-track file, the shared bounded demux first checks both original tracks;
/// an ephemeral projection replaces only the audio `trak` box type with `free`
/// for the video-only parser. Audio remains the separate codec owner's task.
fn project_video_container(bytes:&[u8])->Result<Cow<'_,[u8]>,String>{
    fn children(bytes:&[u8],start:usize,end:usize)->Result<Vec<(usize,usize)>,String>{
        let mut p=start;let mut out=Vec::new();
        while p<end {
            if out.len()>=256||end-p<8{return Err("Invalid MP4 box sequence".into());}
            let n=u32::from_be_bytes(bytes[p..p+4].try_into().map_err(|_|"MP4 box length")?) as usize;
            let next=p.checked_add(n).filter(|x|n>=8&&*x<=end).ok_or("MP4 box exceeds parent")?;out.push((p,next));p=next;
        }Ok(out)
    }
    fn one_child(bytes:&[u8],start:usize,end:usize,name:&[u8])->Result<(usize,usize),String>{
        let matching:Vec<_>=children(bytes,start,end)?.into_iter().filter(|&(p,_)|&bytes[p+4..p+8]==name).collect();
        if matching.len()!=1{return Err("Missing or ambiguous MP4 child box".into());}Ok(matching[0])
    }
    let roots=children(bytes,0,bytes.len())?;let moovs:Vec<_>=roots.iter().filter(|&&(p,_)|&bytes[p+4..p+8]==b"moov").copied().collect();
    let mdats:Vec<_>=roots.iter().filter(|&&(p,_)|&bytes[p+4..p+8]==b"mdat").copied().collect();
    if moovs.len()!=1{return Err("MP4 requires one movie box".into());}
    if mdats.len()!=1{return Err("MP4 requires one media-data box".into());}
    let (m,me)=moovs[0];let boxes=children(bytes,m+8,me)?;
    let tracks:Vec<_>=boxes.iter().filter(|&&(p,_)|&bytes[p+4..p+8]==b"trak").copied().collect();
    if tracks.len()==1{return Ok(Cow::Borrowed(bytes));}
    if tracks.len()!=2{return Err("MP4 requires one video or one video and one audio track".into());}
    let mut audio=None;let mut videos=0;
    for &(p,end) in &tracks {
        let mdia:Vec<_>=children(bytes,p+8,end)?.into_iter().filter(|&(x,_)|&bytes[x+4..x+8]==b"mdia").collect();
        if mdia.len()!=1{return Err("MP4 track requires one media box".into());}
        let (q,qe)=mdia[0];let handlers:Vec<_>=children(bytes,q+8,qe)?.into_iter().filter(|&(x,_)|&bytes[x+4..x+8]==b"hdlr").collect();
        if handlers.len()!=1{return Err("MP4 track requires one handler".into());}
        let (h,he)=handlers[0];if he-h<20{return Err("Truncated MP4 handler".into());}
        match &bytes[h+16..h+20]{b"vide"=>videos+=1,b"soun"=>{if audio.replace(p).is_some(){return Err("Multiple audio tracks".into());}},_=>return Err("Unsupported MP4 track".into())}
    }
    if videos!=1{return Err("MP4 requires exactly one video track".into());}let audio=audio.ok_or("Missing MP4 audio track")?;
    let (_,audio_end)=tracks.iter().find(|&&(p,_)|p==audio).copied().ok_or("Missing audio track extent")?;
    let (mdia,mdia_end)=one_child(bytes,audio+8,audio_end,b"mdia")?;
    let (minf,minf_end)=one_child(bytes,mdia+8,mdia_end,b"minf")?;
    let (stbl,stbl_end)=one_child(bytes,minf+8,minf_end,b"stbl")?;
    let _=one_child(bytes,stbl+8,stbl_end,b"stsd")?;
    let (dinf,dinf_end)=one_child(bytes,minf+8,minf_end,b"dinf")?;
    let (dref,dref_end)=one_child(bytes,dinf+8,dinf_end,b"dref")?;
    if dref_end-dref<16||bytes[dref+8..dref+12]!=[0,0,0,0]||bytes[dref+12..dref+16]!=[0,0,0,1]{return Err("Invalid audio data reference table".into());}
    let refs=children(bytes,dref+16,dref_end)?;
    if refs.len()!=1||refs[0].1-refs[0].0!=12||&bytes[refs[0].0+4..refs[0].0+8]!=b"url "||bytes[refs[0].0+8..refs[0].0+12]!=[0,0,0,1]{return Err("Audio data reference must be self-contained".into());}
    let limits=media_isobmff::DemuxLimits{max_input_bytes:MAX_INPUT as u64,max_header_bytes:MAX_INPUT as u64,max_tracks:2,max_fragments:0,max_table_entries:2048,max_samples_per_track:2048,reject_audio_composition_offsets:true,require_self_contained_data:true};
    let file=media_isobmff::open_with_limits(bytes,limits).map_err(|_|"Combined MP4 demux admission failed")?;
    if file.fragmented||file.tracks.len()!=2{return Err("Fragmented or ambiguous MP4 tracks".into());}
    let video:Vec<_>=file.tracks.iter().filter(|t|t.kind==media_isobmff::TrackKind::Video).collect();
    let audio_track:Vec<_>=file.tracks.iter().filter(|t|t.kind==media_isobmff::TrackKind::Audio).collect();
    if video.len()!=1||audio_track.len()!=1||video[0].id==audio_track[0].id||!matches!(video[0].codec(),Some(media_isobmff::CodecConfig::Avc(_))) {return Err("Combined MP4 requires distinct AVC video and AAC audio tracks".into());}
    let a=audio_track[0];
    if a.entries.len()!=1||a.samples.is_empty()||a.samples.len()>2048{return Err("Unsupported MP4 audio track profile".into());}
    let config=match a.codec(){Some(media_isobmff::CodecConfig::Aac(c))=>c,_=>return Err("MP4 audio is not AAC".into())};
    if config.asc.len()!=2||config.asc[1]&7!=0||config.object_type!=2||!(1..=2).contains(&config.channel_config)||config.sample_rate==0||a.timescale!=config.sample_rate{return Err("MP4 audio requires mono/stereo AAC-LC".into());}
    if a.entries[0].data_reference_index!=1{return Err("Audio sample entry must use the local data reference".into());}
    // Only accept the single-edit profile admitted by Poet's AAC owner.
    // This maps audio presentation onto decoded samples; it does not shift
    // video timestamps, which are already on the movie presentation timeline.
    let total=(a.samples.len() as u64).checked_mul(1024).ok_or("AAC sample count overflow")?;
    admit_audio_edit(&a.edits,file.timescale,a.timescale,total)?;
    let (mdat_start,mdat_end)=mdats[0];let mut spans=Vec::new();let mut audio_time=0i64;
    for track in &file.tracks {for sample in &track.samples {
        if sample.description_index!=1||sample.size==0{return Err("Invalid MP4 sample description or size".into());}
        let end=sample.offset.checked_add(u64::from(sample.size)).ok_or("MP4 sample range overflow")?;
        if sample.offset<(mdat_start+8) as u64||end>mdat_end as u64{return Err("MP4 sample outside media data".into());}
        spans.push((sample.offset,end));
        if track.kind==media_isobmff::TrackKind::Audio {if sample.dts!=audio_time||sample.pts!=audio_time||sample.duration!=1024{return Err("Unsupported AAC packet timing".into());}audio_time=audio_time.checked_add(1024).ok_or("AAC timing overflow")?;}
    }}
    spans.sort_unstable();if spans.windows(2).any(|x|x[0].1>x[1].0){return Err("Overlapping MP4 samples".into());}
    let mut copy=Vec::new();copy.try_reserve_exact(bytes.len()).map_err(|_|"MP4 projection allocation refused")?;copy.extend_from_slice(bytes);copy[audio+4..audio+8].copy_from_slice(b"free");
    Ok(Cow::Owned(copy))
}
fn admit_audio_edit(edits:&[media_isobmff::Edit],movie_rate:u32,rate:u32,total:u64)->Result<(),String>{
    if edits.is_empty(){return Ok(());}
    if edits.len()!=1||movie_rate==0||rate==0{return Err("Unsupported MP4 audio edit list".into());}
    let edit=edits[0];
    if edit.media_time<0||edit.media_rate!=0x10000||edit.segment_duration==0{return Err("Unsupported MP4 audio edit".into());}
    let scaled=u128::from(edit.segment_duration)*u128::from(rate);
    if scaled%u128::from(movie_rate)!=0{return Err("MP4 audio edit duration does not align to a sample".into());}
    let length=u64::try_from(scaled/u128::from(movie_rate)).map_err(|_|"MP4 audio edit duration overflow")?;
    if length==0||(edit.media_time as u64).checked_add(length).is_none_or(|end|end>total){return Err("MP4 audio edit exceeds decoded samples".into());}
    Ok(())
}
/// Bound every table allocation before passing the input to the MP4 parser.
fn admit_sample_table(name:&[u8],data:&[u8])->Result<(),String>{
    let (header, count_at, stride) = match name {
        b"stco"|b"stss" => (16,12,4), b"co64"|b"stts" => (16,12,8),
        b"stsc" => (16,12,12), b"stsz" => (20,16,4), b"ctts" => (16,12,8), _ => return Ok(())
    };
    if data.len()<header{return Err("Truncated MP4 sample table".into());}
    let count=u32::from_be_bytes(data[count_at..count_at+4].try_into().map_err(|_|"Invalid MP4 table count")?);
    if count>MAX_SAMPLES{return Err("MP4 sample table count exceeds preview profile".into());}
    if name==b"ctts" && (data[8]>1||data[9..12]!=[0,0,0]) {return Err("Unsupported composition table version or flags".into());}
    let fixed=name==b"stsz" && u32::from_be_bytes(data[12..16].try_into().map_err(|_|"Invalid fixed sample size")?)!=0;
    let expected=header+if fixed {0}else{(count as usize)*stride};
    if data.len()!=expected{return Err("MP4 sample table length does not match its count".into());}
    Ok(())
}
/// Admission for the owning demuxer: reject fragmented/event/extended boxes
/// and enforce all top-level extents before its fallible parser allocates.
fn check_top_level(bytes:&[u8])->Result<(u64,u64),String>{
    #[derive(Default)] struct Scan {boxes:usize,ftyp:usize,moov:usize,mdat:usize,mdat_range:(u64,u64),tracks:usize,ctts:usize,mdia:usize,minf:usize,stbl:usize,stsd:usize,avc1:usize,dinf:usize,dref:usize,url:usize}
    fn walk(bytes:&[u8],mut pos:usize,end:usize,parent:&[u8],depth:usize,s:&mut Scan)->Result<(),String>{
        if depth>8{return Err("MP4 box nesting exceeds preview profile".into());}
        while pos<end {
            if end-pos<8{return Err("Truncated MP4 box header".into());}
            let size=u32::from_be_bytes(bytes[pos..pos+4].try_into().map_err(|_|"Invalid MP4 box")?) as usize;
            let next=pos.checked_add(size).filter(|next|size>=8&&*next<=end).ok_or("MP4 child box exceeds parent")?;
            let name=&bytes[pos+4..pos+8];s.boxes+=1;if s.boxes>256{return Err("MP4 box count exceeds preview profile".into());}
            let allowed=match parent {
                b"root"=>matches!(name,b"ftyp"|b"moov"|b"mdat"|b"free"|b"skip"|b"wide"),
                b"moov"=>matches!(name,b"mvhd"|b"trak"|b"free"),
                b"trak"=>matches!(name,b"tkhd"|b"mdia"),
                b"mdia"=>matches!(name,b"mdhd"|b"hdlr"|b"minf"),
                b"minf"=>matches!(name,b"vmhd"|b"dinf"|b"stbl"),
                b"dinf"=>name==b"dref",
                b"dref"=>name==b"url ",
                b"stbl"=>matches!(name,b"stsd"|b"stts"|b"stsc"|b"stsz"|b"stco"|b"co64"|b"stss"|b"ctts"),
                b"stsd"=>name==b"avc1",
                b"avc1"=>name==b"avcC",
                _=>false,
            };
            if !allowed{return Err("Unsupported nested MP4 box".into());}
            admit_sample_table(name,&bytes[pos..next])?;
            match name {
                b"ftyp"=>s.ftyp+=1,b"moov"=>s.moov+=1,b"mdat"=>{s.mdat+=1;s.mdat_range=((pos+8) as u64,next as u64);},b"trak"=>s.tracks+=1,b"mdia"=>s.mdia+=1,b"minf"=>s.minf+=1,b"stbl"=>s.stbl+=1,b"stsd"=>s.stsd+=1,b"avc1"=>s.avc1+=1,b"dinf"=>s.dinf+=1,b"dref"=>s.dref+=1,b"url "=>s.url+=1,b"ctts"=>{s.ctts+=1;if s.ctts>1{return Err("Duplicate composition tables".into());}},_=>{}
            }
            let offset=match name {
                b"moov"|b"trak"|b"mdia"|b"minf"|b"dinf"|b"stbl"=>Some(8),
                b"dref"|b"stsd"=>{
                    if size<16||u32::from_be_bytes(bytes[pos+12..pos+16].try_into().map_err(|_|"Invalid MP4 entry count")?)!=1{return Err("MP4 requires one sample/data entry".into());}Some(16)
                },
                b"avc1"=>Some(86),
                _=>None,
            };
            if name==b"url " && (size!=12||bytes[pos+8..pos+12]!=[0,0,0,1]) {return Err("External MP4 data references are unsupported".into());}
            if name==b"avc1" && (size<16||bytes[pos+14..pos+16]!=[0,1]){return Err("Video sample entry must use the local data reference".into());}
            if name==b"avcC" {
                if size<15 || bytes[pos+8]!=1 || bytes[pos+13]&31!=1 {return Err("Invalid or unsupported AVC configuration".into());}
                let mut p=pos+14;
                for set in 0..2 {
                    if set==1 {if p>=next||bytes[p]!=1{return Err("AVC requires exactly one PPS".into());}p+=1;}
                    if p.checked_add(2).is_none_or(|x|x>next){return Err("Truncated AVC parameter set".into());}
                    let n=u16::from_be_bytes([bytes[p],bytes[p+1]]) as usize;p+=2;
                    p=p.checked_add(n).filter(|x|n>0&&n<=4096&&*x<=next).ok_or("Invalid AVC parameter-set extent")?;
                }
                if p!=next{return Err("Unsupported extra AVC configuration data".into());}
            }
            if let Some(offset)=offset {let child=pos.checked_add(offset).filter(|p|*p<=next).ok_or("Truncated MP4 container")?;walk(bytes,child,next,name,depth+1,s)?;}
            pos=next;
        }
        Ok(())
    }
    let mut scan=Scan::default();walk(bytes,0,bytes.len(),b"root",0,&mut scan)?;
    if scan.ftyp!=1||scan.moov!=1||scan.mdat!=1||scan.tracks!=1||scan.mdia!=1||scan.minf!=1||scan.stbl!=1||scan.stsd!=1||scan.avc1!=1||scan.dinf!=1||scan.dref!=1||scan.url!=1{return Err("Preview requires one video track with one self-contained data reference".into());}Ok(scan.mdat_range)
}

fn decode_inner(bytes:&[u8],mdat:(u64,u64),start_tick:u64,end_tick:u64,cancelled:&AtomicBool,emit:&mut dyn FnMut(Frame)->Result<(),String>)->Result<(),String>{
    live(cancelled)?;
    let mut reader=mp4::Mp4Reader::read_header(Cursor::new(bytes),bytes.len() as u64).map_err(|_|"Invalid MP4 container")?;
    let tracks:Vec<u32>=reader.tracks().iter().filter_map(|(&id,t)| matches!(t.track_type(),Ok(mp4::TrackType::Video)).then_some(id)).collect();
    if tracks.len()!=1 {return Err("Preview requires exactly one video track".into());}
    let id=tracks[0]; let track=reader.tracks().get(&id).ok_or("Video track absent")?;
    if !matches!(track.media_type(),Ok(mp4::MediaType::H264))||track.trak.edts.is_some()||!track.trafs.is_empty()||track.trak.tkhd.matrix!=Default::default(){return Err("Unsupported video codec, edit list, fragmentation or track transform".into());}
    let count=track.sample_count();let scale=track.timescale();
    if count==0||count>MAX_SAMPLES||scale==0||TICKS_PER_SECOND%u64::from(scale)!=0 {return Err("Unsupported sample count or inexact MP4 timebase".into());}
    if let Some(ctts)=&track.trak.mdia.minf.stbl.ctts {
        let mut total=0u32;
        for entry in &ctts.entries {
            if entry.sample_count==0 || (ctts.version==0 && entry.sample_offset<0){return Err("Invalid composition offset entry".into());}
            total=total.checked_add(entry.sample_count).ok_or("Composition sample count overflow")?;
        }
        if total!=count{return Err("Composition table does not cover all samples".into());}
    }
    // `mp4::read_sample` allocates the declared stsz size before EOF checks.
    // Validate all declared sizes against the actual admitted mdat first.
    let sizes=&track.trak.mdia.minf.stbl.stsz;
    validate_sizes(sizes.sample_size,sizes.sample_count,&sizes.sample_sizes,count,(mdat.1-mdat.0) as usize)?;
    validate_offsets(track,count,mdat)?;
    let avc=track.trak.mdia.minf.stbl.stsd.avc1.as_ref().ok_or("Missing AVC sample entry")?;
    let cfg=&avc.avcc;
    if cfg.configuration_version!=1||cfg.sequence_parameter_sets.len()!=1||cfg.picture_parameter_sets.len()!=1 {return Err("Preview requires exactly one SPS and PPS".into());}
    let sps: &[u8]=cfg.sequence_parameter_sets[0].bytes.as_ref();let pps: &[u8]=cfg.picture_parameter_sets[0].bytes.as_ref();
    if sps.len()<4||sps.len()>4096||pps.is_empty()||pps.len()>4096||sps[0]&31!=7||pps[0]&31!=8{return Err("Invalid AVC parameter sets".into());}
    let parsed=video_h264_codec::params::Sps::parse(&bitstream_codec::unescape_rbsp(&sps[1..])).map_err(|_|"Invalid H.264 SPS")?;
    parsed.check_supported().map_err(|_|"Unsupported H.264 profile")?;
    let mut spss=vec![None;32];spss[parsed.id as usize]=Some(parsed.clone());
    let parsed_pps=video_h264_codec::params::Pps::parse(&bitstream_codec::unescape_rbsp(&pps[1..]),&spss).map_err(|_|"Invalid H.264 PPS")?;
    if parsed.width()==0||parsed.height()==0||parsed.width()>MAX_DIM||parsed.height()>MAX_DIM||parsed.max_num_ref_frames>4||parsed.max_dpb_frames()>4||parsed.max_num_reorder_frames()>4 {return Err("H.264 dimensions or reference budget exceeded".into());}
    let color=parsed.vui.as_ref().ok_or("H.264 color metadata is required")?;
    if color.colour_primaries!=1||color.transfer_characteristics!=1||color.matrix_coefficients!=1||color.sar!=(1,1) {return Err("Explicit BT.709 colour and square pixels are required".into());}
    let length_size=usize::from((cfg.length_size_minus_one&3)+1);
    let mut avcc=vec![1,cfg.avc_profile_indication,cfg.profile_compatibility,cfg.avc_level_indication,0xfc|((length_size-1) as u8),0xe1];
    for (tag,nal) in [(false,sps),(true,pps)] {if tag {avcc.push(1);}avcc.extend_from_slice(&(nal.len() as u16).to_be_bytes());avcc.extend_from_slice(nal);}
    let unit=TICKS_PER_SECOND/u64::from(scale);let mut expected=0u64;
    let mut timings=Vec::new();let mut gaps=GapBudget::default();
    for sample_id in 1..=count {
        live(cancelled)?;
        let sample=reader.read_sample(id,sample_id).map_err(|_|"MP4 sample read failed")?.ok_or("Missing MP4 video sample")?;
        if sample.duration==0||sample.start_time!=expected{return Err("Only contiguous decode timestamps are supported".into());}
        let max_mb=parsed.pic_width_in_mbs.checked_mul(parsed.frame_height_in_mbs()).ok_or("H.264 macroblock count overflow")?;
        let is_idr=check_sample(&sample.bytes,length_size,max_mb)?;
        let first=validate_picture_headers(&sample.bytes,length_size,&parsed_pps,&parsed)?;
        gaps.admit(first.frame_num,parsed.max_frame_num(),first.idr,first.nal_ref_idc!=0,first.has_mmco5())?;
        if sample_id==1 && (!sample.is_sync||!is_idr){return Err("The first video sample must be an IDR sync sample".into());}
        let sample_end=sample.start_time.checked_add(u64::from(sample.duration)).ok_or("MP4 sample time overflow")?;
        let display=i64::try_from(sample.start_time).map_err(|_|"MP4 decode time overflow")?.checked_add(i64::from(sample.rendering_offset)).filter(|&v|v>=0).ok_or("Invalid MP4 presentation time")? as u64;
        let begin=display.checked_mul(unit).ok_or("MP4 time conversion overflow")?;let end=display.checked_add(u64::from(sample.duration)).and_then(|x|x.checked_mul(unit)).ok_or("MP4 time conversion overflow")?;
        timings.push((begin,end,sample_id));
        expected=sample_end;
    }
    timings.sort_unstable_by_key(|t|t.0);
    let mut presentation_end=0;
    for &(begin,end,_) in &timings {if begin!=presentation_end{return Err("Presentation intervals must be contiguous and non-overlapping".into());}presentation_end=end;}
    if presentation_end<end_tick||timings.is_empty(){return Err("Clip range exceeds the video track".into());}
    let mut decoder=video_h264_codec::Decoder::with_threads(1);
    decoder.configure_avcc(&avcc).map_err(|_|"H.264 configuration refused")?;
    let mut index=0usize;let mut emitted=0usize;
    let mut consume=|pictures:Vec<video_h264_codec::Picture>|->Result<(),String>{
        for frame in pictures {
            live(cancelled)?;
            let &(begin,end,id)=timings.get(index).ok_or("H.264 decoder returned extra pictures")?;
            if frame.pts!=i64::from(id) {return Err("H.264 picture order differs from composition timeline".into());}
            if frame.width==0||frame.height==0||frame.width>MAX_DIM||frame.height>MAX_DIM||frame.color.primaries!=1||frame.color.transfer!=1||frame.color.matrix!=1||frame.sar!=(1,1){return Err("Decoded frame geometry or colour is outside preview profile".into());}
            if end>start_tick&&begin<end_tick {
                let rgba=rgba709(&frame,cancelled)?;
                emit(Frame{width:frame.width,height:frame.height,duration_ticks:end.min(end_tick)-begin.max(start_tick),rgba,index,time:(begin as f64)/(TICKS_PER_SECOND as f64)})?;emitted+=1;
            }
            index+=1;
        }Ok(())
    };
    for sample_id in 1..=count {
        live(cancelled)?;
        let sample=reader.read_sample(id,sample_id).map_err(|_|"MP4 sample read failed")?.ok_or("Missing MP4 video sample")?;
        consume(decoder.decode(&sample.bytes,i64::from(sample_id)).map_err(|_|"H.264 frame decode failed")?)?;
    }
    let final_pictures=decoder.flush();
    if decoder.take_error().is_some(){return Err("H.264 frame decode failed during flush".into());}
    consume(final_pictures)?;
    drop(consume);
    if index!=timings.len(){return Err("H.264 decoder returned missing pictures".into());}
    if emitted==0{return Err("No video frame in selected clip range".into());}
    live(cancelled)?;Ok(())
}
fn validate_offsets(track:&mp4::Mp4Track,count:u32,mdat:(u64,u64))->Result<(),String>{
    let stbl=&track.trak.mdia.minf.stbl;
    let chunks=match (&stbl.stco,&stbl.co64){
        (Some(x),None)=>x.entries.iter().map(|&v|u64::from(v)).collect::<Vec<_>>(),
        (None,Some(x))=>x.entries.clone(),
        _=>return Err("MP4 requires exactly one chunk-offset table".into()),
    };
    if chunks.is_empty()||chunks.len()>MAX_SAMPLES as usize{return Err("MP4 chunk count exceeds preview profile".into());}
    let entries=&stbl.stsc.entries;
    if entries.is_empty()||entries.len()>MAX_SAMPLES as usize||entries[0].first_chunk!=1||entries[0].first_sample!=1{return Err("Invalid MP4 sample-to-chunk table".into());}
    for (i,e) in entries.iter().enumerate(){
        if e.samples_per_chunk==0||e.samples_per_chunk>MAX_SAMPLES||e.sample_description_index!=1||e.first_chunk as usize>chunks.len()||
            (i>0&&(entries[i-1].first_chunk>=e.first_chunk||entries[i-1].first_sample>=e.first_sample)){return Err("Invalid MP4 sample-to-chunk entry".into());}
    }
    let size=|id:u32|->Result<u64,String>{
        if stbl.stsz.sample_size!=0 {Ok(u64::from(stbl.stsz.sample_size))} else {stbl.stsz.sample_sizes.get((id-1) as usize).copied().map(u64::from).ok_or("Missing MP4 sample size".into())}
    };
    let mut spans=Vec::with_capacity(count as usize);
    for id in 1..=count {
        let e=entries.iter().rev().find(|e|e.first_sample<=id).ok_or("Missing MP4 chunk mapping")?;
        let rel=id.checked_sub(e.first_sample).ok_or("Invalid MP4 chunk mapping")?;
        let chunk=e.first_chunk.checked_add(rel/e.samples_per_chunk).ok_or("MP4 chunk index overflow")?;
        let mut offset=*chunks.get((chunk-1) as usize).ok_or("Missing MP4 chunk offset")?;
        let first=id-(rel%e.samples_per_chunk);
        for prior in first..id {offset=offset.checked_add(size(prior)?).ok_or("MP4 sample offset overflow")?;}
        let end=offset.checked_add(size(id)?).ok_or("MP4 sample extent overflow")?;
        if offset<mdat.0||end>mdat.1{return Err("MP4 sample points outside selected media data".into());}
        spans.push((offset,end));
    }
    spans.sort_unstable();
    if spans.windows(2).any(|pair|pair[0].1>pair[1].0){return Err("Overlapping MP4 samples are unsupported".into());}Ok(())
}
fn validate_sizes(fixed:u32,declared:u32,variable:&[u32],count:u32,mdat:usize)->Result<(),String>{
    if declared!=count||count==0||count>MAX_SAMPLES {return Err("Invalid MP4 sample count".into());}
    let mut total=0usize;
    if fixed!=0 {
        if !variable.is_empty(){return Err("Conflicting MP4 sample sizes".into());}
        total=(fixed as usize).checked_mul(count as usize).ok_or("MP4 sample byte count overflow")?;
        if fixed as usize>mdat{return Err("MP4 sample size exceeds media data".into());}
    } else {
        if variable.len()!=count as usize{return Err("MP4 sample size count mismatch".into());}
        for &size in variable {let size=size as usize;if size==0||size>mdat{return Err("MP4 sample size exceeds media data".into());}total=total.checked_add(size).ok_or("MP4 sample byte count overflow")?;}
    }
    if total>mdat{return Err("MP4 samples exceed media data".into());}Ok(())
}
fn live(c:&AtomicBool)->Result<(),String>{if c.load(Ordering::Acquire){Err("Video preview cancelled".into())}else{Ok(())}}

/// Reject in-band SPS/PPS changes, B/SP/SI slices and mixed IDR/non-IDR
/// access units before the decoder sees them. P/I slices still use the existing
/// decoder and the checked SPS and PPS. Return true only for an IDR sample.
fn check_sample(bytes:&[u8],length_size:usize,max_mb:u32)->Result<bool,String>{
    if !(1..=4).contains(&length_size){return Err("Invalid AVC length width".into());}
    let mut p=0usize;let mut n=0usize;let mut slices=0usize;let mut mode=None;let mut reference=None;let mut previous_mb=None;
    while p<bytes.len(){
        if bytes.len()-p<length_size{return Err("Truncated AVC NAL length".into());}
        let mut len=0usize;for &b in &bytes[p..p+length_size]{len=len.checked_mul(256).and_then(|v|v.checked_add(b as usize)).ok_or("NAL length overflow")?;}p+=length_size;
        let end=p.checked_add(len).filter(|end|*end<=bytes.len()).ok_or("Truncated AVC NAL")?;
        if len==0{return Err("Empty AVC NAL".into());}n+=1;if n>MAX_NALS{return Err("AVC NAL count exceeded".into());}
        let header=bytes[p];if header&0x80!=0{return Err("Invalid AVC forbidden bit".into());}
        match header&31 {
            kind @ (1|5)=>{
                slices+=1;if slices>MAX_SLICES{return Err("AVC slice count exceeds preview budget".into());}
                let is_idr=kind==5;
                if mode.is_some_and(|old|old!=is_idr){return Err("Mixed IDR and non-IDR slices are unsupported".into());}
                let is_reference=header&0x60!=0;
                if (is_idr&&!is_reference)||reference.is_some_and(|old|old!=is_reference){return Err("Invalid or inconsistent picture reference flags".into());}
                let rbsp=bitstream_codec::unescape_rbsp(&bytes[p+1..end]);
                let mut bits=bitstream_codec::BitReader::new(&rbsp);
                let first_mb=bits.read_ue().map_err(|_|"Malformed AVC slice")?;
                if first_mb>=max_mb || previous_mb.is_none() && first_mb!=0 || previous_mb.is_some_and(|prior|first_mb<=prior) {
                    return Err("Slices must belong to one ordered picture".into());
                }
                let slice=bits.read_ue().map_err(|_|"Malformed AVC slice")?;
                if slice>9 || !matches!(slice%5,0|1|2) || (is_idr && slice%5!=2) {return Err("Only P/I/B slices are supported; SP/SI slices are refused".into());}
                mode=Some(is_idr);reference=Some(is_reference);previous_mb=Some(first_mb);
            },
            6|9|12=>{},
            _=>return Err("Unsupported AVC NAL or in-band parameter-set change".into())
        }p=end;
    }
    mode.ok_or("Sample has no video slice".into())
}
/// Match the codec's own primary-picture boundary test before it can create
/// a picture or synthesize gap frames. `check_sample` has already admitted NAL
/// lengths, type, slice count, and strictly increasing macroblock starts.
fn validate_picture_headers(bytes:&[u8],length_size:usize,pps:&video_h264_codec::params::Pps,sps:&video_h264_codec::params::Sps)->Result<video_h264_codec::slice::SliceHeader,String>{
    use video_h264_codec::slice::{is_new_picture,NalHeader,SliceHeader};
    let mut p=0usize;let mut first:Option<SliceHeader>=None;
    while p<bytes.len(){
        let mut len=0usize;for &b in &bytes[p..p+length_size]{len=len*256+b as usize;}p+=length_size;
        let end=p+len;let header=bytes[p];
        if matches!(header&31,1|5){
            let nal=NalHeader::parse(header).map_err(|_|"Invalid AVC NAL header")?;
            let rbsp=bitstream_codec::unescape_rbsp(&bytes[p+1..end]);
            let (sh,_,_)=SliceHeader::parse(&rbsp,nal,|id|{
                if id==pps.id {Ok((pps,sps))}else{Err(video_h264_codec::Error::Invalid("Unknown PPS".into()))}
            }).map_err(|_|"Invalid AVC slice header")?;
            if first.as_ref().is_some_and(|prev|is_new_picture(prev,&sh,sps)){return Err("Sample contains multiple AVC pictures".into());}
            if first.is_none(){first=Some(sh);}
        }
        p=end;
    }
    first.ok_or_else(||"Sample has no video slice".into())
}

fn rgba709(p:&video_h264_codec::Picture,c:&AtomicBool)->Result<Vec<u8>,String>{
    use color_transforms::{linear_to_srgb_u8,normalize_c,normalize_y,to_linear,ycbcr_to_rgb,Matrix,Range,Transfer};
    let w=p.width as usize;let h=p.height as usize;let cw=p.chroma_width as usize;let ch=p.chroma_height as usize;
    if p.y_stride<w||p.uv_stride<cw||cw!=(w+1)/2||ch!=(h+1)/2||p.y.len()<p.y_stride.checked_mul(h).ok_or("Luma overflow")?||p.u.len()<p.uv_stride.checked_mul(ch).ok_or("Chroma overflow")?||p.v.len()<p.uv_stride.checked_mul(ch).ok_or("Chroma overflow")? {return Err("Decoded plane extent is invalid".into());}
    let len=w.checked_mul(h).and_then(|n|n.checked_mul(4)).ok_or("Video output overflow")?;
    let mut out=Vec::new();out.try_reserve_exact(len).map_err(|_|"Video output allocation refused")?;
    let range=if p.color.full_range{Range::Full}else{Range::Limited};
    for y in 0..h {if y%16==0{live(c)?;}for x in 0..w {
        let yy=normalize_y(u32::from(p.y[y*p.y_stride+x]),8,range);
        let cb=normalize_c(u32::from(p.u[(y/2)*p.uv_stride+x/2]),8,range);
        let cr=normalize_c(u32::from(p.v[(y/2)*p.uv_stride+x/2]),8,range);
        let rgb=ycbcr_to_rgb(yy,cb,cr,Matrix::Bt709);
        for v in rgb {out.push(linear_to_srgb_u8(to_linear(v,Transfer::Bt709)));}out.push(255);
    }} Ok(out)
}

#[cfg(test)] mod tests {
 use super::*;
 use bitstream_codec::{BitWriter,escape_rbsp};
 use mp4::{AvcConfig,Bytes,MediaConfig,Mp4Config,Mp4Sample,Mp4Writer,TrackConfig,TrackType};
 #[test] fn actual_gaps_are_budgeted_across_wrap_nonreference_and_mmco5(){
  let mut g=GapBudget::default();
  assert!(g.admit(0,65536,true,true,false).is_ok());
  assert!(g.admit(1,65536,false,true,false).is_ok());
  assert!(g.admit(1,65536,false,false,false).is_ok());
  assert!(g.admit(4,65536,false,false,false).is_ok());assert_eq!((g.generated,g.prev_ref),(2,3));
  assert!(g.admit(4,65536,false,true,true).is_ok());assert_eq!(g.prev_ref,0);
  assert!(g.admit(18,65536,false,true,false).is_err());
  let mut g=GapBudget{prev_ref:65535,generated:0};assert!(g.admit(0,65536,false,true,false).is_ok());assert_eq!(g.generated,0);
  let mut g=GapBudget::default();assert!(g.admit(65535,65536,false,true,false).is_err());
 }
 // Synthetic one-macroblock I_PCM pictures for clean-room testing;
 // H.264 synthetic tests; no external video or fabricated decode result.
 fn nal(header:u8,w:BitWriter)->Vec<u8>{let mut out=vec![header];out.extend(escape_rbsp(&w.finish()));out}
 fn sps()->Vec<u8>{sps_for_width(1)}
 fn sps_for_width(width:u32)->Vec<u8>{sps_for_params(width,4)}
 fn sps_for_params(width:u32,frame_bits:u32)->Vec<u8>{
  let mut w=BitWriter::new();w.write_bits(66,8);w.write_bits(0,8);w.write_bits(10,8);
  w.write_ue(0);w.write_ue(frame_bits-4);w.write_ue(2);w.write_ue(1);w.write_bit(false);
  w.write_ue(width-1);w.write_ue(0);w.write_bit(true);w.write_bit(true);w.write_bit(false);
  w.write_bit(true);w.write_bit(true);w.write_bits(1,8);w.write_bit(false);
  w.write_bit(true);w.write_bits(5,3);w.write_bit(false);w.write_bit(true);
  w.write_bits(1,8);w.write_bits(1,8);w.write_bits(1,8);
  w.write_bit(false);w.write_bit(false);w.write_bit(false);w.write_bit(false);w.write_bit(false);
  w.write_bit(true);w.write_bit(true);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(1);
  w.rbsp_trailing();nal(0x67,w)
 }
 fn pps()->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(0);w.write_bit(false);w.write_bit(false);w.write_ue(0);
  w.write_ue(0);w.write_ue(0);w.write_bit(false);w.write_bits(0,2);w.write_se(0);w.write_se(0);w.write_se(0);
  w.write_bit(true);w.write_bit(false);w.write_bit(false);w.rbsp_trailing();nal(0x68,w)
 }
 fn idr(luma:u8)->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(7);w.write_ue(0);w.write_bits(0,4);w.write_ue(0);
  w.write_bit(false);w.write_bit(false);w.write_se(0);w.write_ue(0);w.write_se(0);w.write_se(0);
  w.write_ue(25);w.align_zero();w.write_bytes(&vec![luma;256]);w.write_bytes(&vec![128;128]);w.rbsp_trailing();
  let data=nal(0x65,w);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);sample
 }
 #[test] fn metadata_names_in_pixel_payload_do_not_reject_valid_video(){
  for marker in [b"colr",b"pasp",b"clap"] {
   let mut bits=BitWriter::new();bits.write_ue(0);bits.write_ue(7);bits.write_ue(0);bits.write_bits(0,4);bits.write_ue(0);
   bits.write_bit(false);bits.write_bit(false);bits.write_se(0);bits.write_ue(0);bits.write_se(0);bits.write_se(0);
   bits.write_ue(25);bits.align_zero();let mut luma=vec![64;256];luma[..4].copy_from_slice(marker);bits.write_bytes(&luma);bits.write_bytes(&vec![128;128]);bits.rbsp_trailing();
   let data=nal(0x65,bits);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);
   let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
   let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
   w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
   w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(sample)}).unwrap();
   w.write_end().unwrap();let file=w.into_writer().into_inner();assert!(file.windows(4).any(|x|x==marker));
   assert!(decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).is_ok());
   let mut bad=file;let avcc=bad.windows(4).position(|x|x==b"avcC").unwrap();bad[avcc..avcc+4].copy_from_slice(marker);
   assert!(check_top_level(&bad).is_err());
  }
 }
 fn idr_slice(first:u32,frame_num:u32,luma:u8)->Vec<u8>{idr_slice_bits(first,frame_num,luma,4)}
 fn idr_slice_bits(first:u32,frame_num:u32,luma:u8,frame_bits:u32)->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(first);w.write_ue(7);w.write_ue(0);w.write_bits(frame_num,frame_bits);w.write_ue(0);
  w.write_bit(false);w.write_bit(false);w.write_se(0);w.write_ue(0);w.write_se(0);w.write_se(0);
  w.write_ue(25);w.align_zero();w.write_bytes(&vec![luma;256]);w.write_bytes(&vec![128;128]);w.rbsp_trailing();
  let data=nal(0x65,w);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);sample
 }
 #[test] fn wider_frame_number_sps_decodes_within_gap_budget(){
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps_for_params(1,5),pic_param_set:pps()})}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr_slice_bits(0,0,64,5))}).unwrap();
  w.write_end().unwrap();let file=w.into_writer().into_inner();
  let frames=decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).unwrap();assert_eq!(frames.len(),1);
 }
 #[test] fn two_slice_picture_decodes_and_changed_picture_identity_is_refused(){
  let mut sample=idr_slice(0,0,64);sample.extend(idr_slice(1,0,192));
  let sps_bytes=sps_for_width(2);let pps_bytes=pps();
  let parsed=video_h264_codec::params::Sps::parse(&bitstream_codec::unescape_rbsp(&sps_bytes[1..])).unwrap();
  let mut spss=vec![None;32];spss[parsed.id as usize]=Some(parsed.clone());
  let parsed_pps=video_h264_codec::params::Pps::parse(&bitstream_codec::unescape_rbsp(&pps_bytes[1..]),&spss).unwrap();
  assert!(check_sample(&sample,4,2).is_ok());assert!(validate_picture_headers(&sample,4,&parsed_pps,&parsed).is_ok());
  let mut bad=idr_slice(0,0,64);bad.extend(idr_slice(1,1,192));
  assert!(check_sample(&bad,4,2).is_ok());assert!(validate_picture_headers(&bad,4,&parsed_pps,&parsed).is_err());
  let mut too_many=Vec::new();for i in 0..9 {too_many.extend(idr_slice(i,0,64));}
  assert!(check_sample(&too_many,4,9).is_err());
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:32,height:16,seq_param_set:sps_bytes,pic_param_set:pps_bytes})}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(sample)}).unwrap();
  w.write_end().unwrap();let file=w.into_writer().into_inner();
  let frames=decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).unwrap();
  assert_eq!((frames[0].width,frames[0].height),(32,16));assert!(frames[0].rgba[0]<frames[0].rgba[31*4]);
 }
 fn p_skip()->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(5);w.write_ue(0);w.write_bits(1,4);
  w.write_bit(false);w.write_bit(false);w.write_bit(false);w.write_se(0);w.write_ue(0);w.write_se(0);w.write_se(0);
  w.write_ue(1);w.rbsp_trailing();let data=nal(0x41,w);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);sample
 }
 // Clean-room synthetic POC/B-skip test; one 16x16 macroblock.
 fn reordered_sps()->Vec<u8>{
  let mut w=BitWriter::new();w.write_bits(77,8);w.write_bits(0,8);w.write_bits(10,8);
  w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(2);w.write_bit(false);
  w.write_ue(0);w.write_ue(0);w.write_bit(true);w.write_bit(true);w.write_bit(false);
  w.write_bit(true);w.write_bit(true);w.write_bits(1,8);w.write_bit(false);
  w.write_bit(true);w.write_bits(5,3);w.write_bit(false);w.write_bit(true);
  w.write_bits(1,8);w.write_bits(1,8);w.write_bits(1,8);
  w.write_bit(false);w.write_bit(false);w.write_bit(false);w.write_bit(false);w.write_bit(false);
  w.write_bit(true);w.write_bit(true);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(0);w.write_ue(2);w.write_ue(2);
  w.rbsp_trailing();nal(0x67,w)
 }
 fn reordered_pcm(idr:bool,frame_num:u32,poc:u32,luma:u8)->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(7);w.write_ue(0);w.write_bits(frame_num,4);
  if idr{w.write_ue(0);}w.write_bits(poc,4);
  w.write_bit(false);if idr{w.write_bit(false);}w.write_se(0);w.write_ue(0);w.write_se(0);w.write_se(0);
  w.write_ue(25);w.align_zero();w.write_bytes(&vec![luma;256]);w.write_bytes(&vec![128;128]);w.rbsp_trailing();
  let data=nal(if idr{0x65}else{0x41},w);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);sample
 }
 fn reordered_b_skip()->Vec<u8>{
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(6);w.write_ue(0);w.write_bits(2,4);w.write_bits(2,4);
  w.write_bit(true);w.write_bit(false);w.write_bit(false);w.write_bit(false);
  w.write_se(0);w.write_ue(0);w.write_se(0);w.write_se(0);w.write_ue(1);w.rbsp_trailing();
  let data=nal(0x01,w);let mut sample=(data.len() as u32).to_be_bytes().to_vec();sample.extend(data);sample
 }
 fn synthetic_reordered()->Vec<u8>{
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:reordered_sps(),pic_param_set:pps()})}).unwrap();
  for (i,(sample,offset)) in [(reordered_pcm(true,0,0,64),0),(reordered_pcm(false,1,4,192),100),(reordered_b_skip(),-100)].into_iter().enumerate(){
   w.write_sample(1,&Mp4Sample{start_time:(i*100) as u64,duration:100,rendering_offset:offset,is_sync:i==0,bytes:Bytes::from(sample)}).unwrap();
  }
  w.write_end().unwrap();let mut file=w.into_writer().into_inner();
  let at=file.windows(4).position(|w|w==b"ctts").unwrap();file[at+4]=1;file
 }
 #[test] fn b_frame_follows_composition_order_and_selection(){
  let file=synthetic_reordered();let cancel=AtomicBool::new(false);
  let frames=decode_mp4(&file,0,TICKS_PER_SECOND*3/10,&cancel).unwrap();
  assert_eq!(frames.len(),3);assert!(frames[0].rgba[0]<frames[2].rgba[0]);
  let selected=decode_mp4(&file,TICKS_PER_SECOND/10,TICKS_PER_SECOND/10,&cancel).unwrap();
  assert_eq!(selected.len(),1);assert_eq!(selected[0].rgba,frames[1].rgba);
 }
 #[test] fn inconsistent_composition_table_is_refused(){
  let mut file=synthetic_reordered();let at=file.windows(4).position(|w|w==b"ctts").unwrap();
  file[at+12..at+16].copy_from_slice(&99u32.to_be_bytes());
  assert!(decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).is_err());
  let mut file=synthetic_reordered();let at=file.windows(4).position(|w|w==b"ctts").unwrap();file[at+4]=0;
  assert!(decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).is_err());
 }
 fn synthetic_mp4()->Vec<u8>{
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
  for (i,y) in [64,192].into_iter().enumerate(){w.write_sample(1,&Mp4Sample{start_time:(i*100) as u64,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr(y))}).unwrap();}
  w.write_end().unwrap();w.into_writer().into_inner()
 }
 fn synthetic_av_mp4()->Vec<u8>{
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Audio,timescale:48000,language:"und".into(),media_conf:MediaConfig::AacConfig(mp4::AacConfig::default())}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr(64))}).unwrap();
  // Synthetic opaque audio packet tests container selection only, not AAC decode.
  w.write_sample(2,&Mp4Sample{start_time:0,duration:1024,rendering_offset:0,is_sync:true,bytes:Bytes::from(vec![1,2,3])}).unwrap();
  w.write_end().unwrap();w.into_writer().into_inner()
 }
 fn add_synthetic_audio_edit(mut file:Vec<u8>)->Vec<u8>{
  // The synthetic writer puts moov after mdat, so inserting inside moov
  // preserves both media offsets. Locate box boundaries, not byte patterns.
  fn boxes(file:&[u8],mut pos:usize,end:usize)->Vec<(usize,usize)>{
   let mut out=Vec::new();while pos<end {let n=u32::from_be_bytes(file[pos..pos+4].try_into().unwrap()) as usize;assert!(n>=8&&pos+n<=end);out.push((pos,pos+n));pos+=n;}out
  }
  let roots=boxes(&file,0,file.len());let moov=roots.iter().find(|&&(p,_)|&file[p+4..p+8]==b"moov").copied().unwrap();
  let mdat=roots.iter().find(|&&(p,_)|&file[p+4..p+8]==b"mdat").copied().unwrap();assert!(mdat.1<=moov.0);
  let tracks:Vec<_>=boxes(&file,moov.0+8,moov.1).into_iter().filter(|&(p,_)|&file[p+4..p+8]==b"trak").collect();
  assert_eq!(tracks.len(),2);let audio=tracks[1];
  let mut edit=Vec::new();edit.extend_from_slice(&36u32.to_be_bytes());edit.extend_from_slice(b"edts");
  edit.extend_from_slice(&28u32.to_be_bytes());edit.extend_from_slice(b"elst");
  edit.extend_from_slice(&0u32.to_be_bytes());edit.extend_from_slice(&1u32.to_be_bytes());
  edit.extend_from_slice(&10u32.to_be_bytes());edit.extend_from_slice(&7i32.to_be_bytes());edit.extend_from_slice(&0x10000i32.to_be_bytes());
  file.splice(audio.1..audio.1,edit);
  let asize=(audio.1-audio.0+36) as u32;file[audio.0..audio.0+4].copy_from_slice(&asize.to_be_bytes());
  let msize=(moov.1-moov.0+36) as u32;file[moov.0..moov.0+4].copy_from_slice(&msize.to_be_bytes());file
 }
 #[test] fn combined_audio_edit_preserves_video_timeline(){
  let original=synthetic_av_mp4();let edited=add_synthetic_audio_edit(original.clone());
  let c=AtomicBool::new(false);
  let before=decode_mp4(&original,0,TICKS_PER_SECOND/10,&c).unwrap();
  let after=decode_mp4(&edited,0,TICKS_PER_SECOND/10,&c).unwrap();
  assert_eq!(before.len(),after.len());assert_eq!(before[0].rgba,after[0].rgba);assert_eq!(before[0].duration_ticks,after[0].duration_ticks);
  let e=media_isobmff::Edit{segment_duration:10,media_time:7,media_rate:0x10000};
  assert!(admit_audio_edit(&[e],1000,48000,1024).is_ok());
  assert!(admit_audio_edit(&[e,e],1000,48000,1024).is_err());
  assert!(admit_audio_edit(&[media_isobmff::Edit{media_time:-1,..e}],1000,48000,1024).is_err());
  assert!(admit_audio_edit(&[media_isobmff::Edit{media_rate:0,..e}],1000,48000,1024).is_err());
  assert!(admit_audio_edit(&[e],1001,48000,1024).is_err());
  assert!(admit_audio_edit(&[e],1000,48000,486).is_err());
 }
 #[test] fn video_projects_combined_av_without_changing_source_or_offsets(){
  let file=synthetic_av_mp4();let original=file.clone();let projected=project_video_container(&file).unwrap();
  assert!(matches!(projected,Cow::Owned(_)));assert_eq!(file,original);
  let decoded=decode_mp4(&file,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)).unwrap();assert_eq!(decoded.len(),1);
  let mut bad=file;let positions:Vec<_>=bad.windows(4).enumerate().filter_map(|(i,b)|(b==b"stco").then_some(i)).collect();
  assert_eq!(positions.len(),2);let audio_offset=positions[1];bad[audio_offset+12..audio_offset+16].copy_from_slice(&0u32.to_be_bytes());
  assert!(project_video_container(&bad).is_err());
  let mut bad=synthetic_av_mp4();let offsets:Vec<_>=bad.windows(4).enumerate().filter_map(|(i,b)|(b==b"stco").then_some(i)).collect();
  bad[offsets[1]+8..offsets[1]+12].copy_from_slice(&u32::MAX.to_be_bytes());
  let result=std::panic::catch_unwind(||decode_mp4(&bad,0,TICKS_PER_SECOND/10,&AtomicBool::new(false)));
  assert!(matches!(result,Ok(Err(_))));
 }
 #[test] fn video_requires_local_data_reference_and_entry_binding(){
  let good=synthetic_mp4();assert!(check_top_level(&good).is_ok());
  let mut missing=good.clone();let at=missing.windows(4).position(|x|x==b"dinf").unwrap();missing[at..at+4].copy_from_slice(b"free");assert!(check_top_level(&missing).is_err());
  let mut external=good.clone();let at=external.windows(4).position(|x|x==b"url ").unwrap();external[at+4..at+8].copy_from_slice(&0u32.to_be_bytes());assert!(check_top_level(&external).is_err());
  let mut wrong=good;let at=wrong.windows(4).position(|x|x==b"avc1").unwrap();wrong[at+10..at+12].copy_from_slice(&2u16.to_be_bytes());assert!(check_top_level(&wrong).is_err());
 }
 #[test] fn stream_delivers_more_than_32_frames_and_sink_can_stop(){
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
  for i in 0..40 {w.write_sample(1,&Mp4Sample{start_time:i*100,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr(64))}).unwrap();}
  w.write_end().unwrap();let file=w.into_writer().into_inner();let cancel=AtomicBool::new(false);
  let mut count=0;decode_mp4_stream(&file,0,TICKS_PER_SECOND*4,&cancel,|frame|{assert_eq!(frame.duration_ticks,TICKS_PER_SECOND/10);count+=1;Ok(())}).unwrap();assert_eq!(count,40);
  let mut count=0;let stopped=decode_mp4_stream(&file,0,TICKS_PER_SECOND*4,&cancel,|_|{count+=1;if count==3{Err("sink closed".into())}else{Ok(())}});
  assert!(stopped.is_err());assert_eq!(count,3);
 }
 fn synthetic_gop()->Vec<u8>{
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr(64))}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:100,duration:100,rendering_offset:0,is_sync:false,bytes:Bytes::from(p_skip())}).unwrap();
  w.write_end().unwrap();w.into_writer().into_inner()
 }
 #[test] fn sample_tables_reject_hostile_counts_before_mp4_parser(){
  for name in [b"stco",b"co64",b"stsc",b"stss",b"stts",b"stsz"] {
   let header=if name==b"stsz"{20}else{16};
   let mut table=vec![0;header];table[4..8].copy_from_slice(name);
   let offset=if name==b"stsz"{16}else{12};table[offset..offset+4].copy_from_slice(&u32::MAX.to_be_bytes());
   assert!(admit_sample_table(name,&table).is_err());
   table[offset..offset+4].copy_from_slice(&1u32.to_be_bytes());
   assert!(admit_sample_table(name,&table).is_err());
  }
  let mut fixed=vec![0;20];fixed[12..16].copy_from_slice(&1u32.to_be_bytes());fixed[16..20].copy_from_slice(&1u32.to_be_bytes());
  assert!(admit_sample_table(b"stsz",&fixed).is_ok());
  let mut file=synthetic_mp4();let at=file.windows(4).position(|w|w==b"stco").unwrap();
  file[at+8..at+12].copy_from_slice(&u32::MAX.to_be_bytes());
  assert!(check_top_level(&file).is_err());
 }
 #[test] fn malformed_final_slice_is_not_presented(){
  // Valid slice header and I_PCM mode, but no required PCM sample payload.
  let mut bits=BitWriter::new();bits.write_ue(0);bits.write_ue(7);bits.write_ue(0);bits.write_bits(0,4);bits.write_ue(0);
  bits.write_bit(false);bits.write_bit(false);bits.write_se(0);bits.write_ue(0);bits.write_se(0);bits.write_se(0);
  bits.write_ue(25);bits.align_zero();bits.rbsp_trailing();
  let data=nal(0x65,bits);let mut bad=(data.len() as u32).to_be_bytes().to_vec();bad.extend(data);
  assert!(check_sample(&bad,4,1).is_ok());
  let c=Mp4Config{major_brand:"isom".parse().unwrap(),minor_version:0,compatible_brands:vec!["isom".parse().unwrap()],timescale:1000};
  let mut w=Mp4Writer::write_start(Cursor::new(Vec::new()),&c).unwrap();
  w.add_track(&TrackConfig{track_type:TrackType::Video,timescale:1000,language:"und".into(),media_conf:MediaConfig::AvcConfig(AvcConfig{width:16,height:16,seq_param_set:sps(),pic_param_set:pps()})}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:0,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(idr(64))}).unwrap();
  w.write_sample(1,&Mp4Sample{start_time:100,duration:100,rendering_offset:0,is_sync:true,bytes:Bytes::from(bad)}).unwrap();
  w.write_end().unwrap();let file=w.into_writer().into_inner();
  assert!(decode_mp4(&file,0,TICKS_PER_SECOND/5,&AtomicBool::new(false)).is_err());
 }
 #[test] fn actual_idr_samples_decode_to_different_srgb_frames(){
  let data=synthetic_mp4();let frames=decode_mp4(&data,0,TICKS_PER_SECOND/5,&AtomicBool::new(false)).unwrap();
  assert_eq!(frames.len(),2);assert_eq!((frames[0].width,frames[0].height),(16,16));
  assert_eq!(frames[0].duration_ticks,TICKS_PER_SECOND/10);assert!(frames[0].rgba[0]<frames[1].rgba[0]);
 }
 #[test] fn p_slice_uses_prior_reference_when_range_starts_after_idr(){
  let file=synthetic_gop();let cancel=AtomicBool::new(false);
  let both=decode_mp4(&file,0,TICKS_PER_SECOND/5,&cancel).unwrap();
  assert_eq!(both.len(),2);assert_eq!(both[0].rgba,both[1].rgba);
  let second=decode_mp4(&file,TICKS_PER_SECOND/10,TICKS_PER_SECOND/10,&cancel).unwrap();
  assert_eq!(second.len(),1);assert_eq!(second[0].rgba,both[1].rgba);
 }
 #[test] fn selected_p_frame_without_prior_idr_is_refused(){
  let mut file=synthetic_gop();
  // Replace the first sync sample payload's IDR NAL header with non-IDR;
  // the MP4 writer may put padding inside mdat, so use its chunk offset.
  let reader=mp4::Mp4Reader::read_header(Cursor::new(&file),file.len() as u64).unwrap();
  let track=reader.tracks().values().next().unwrap();
  let first=track.trak.mdia.minf.stbl.stco.as_ref().unwrap().entries[0] as usize;
  assert_eq!(file[first+4]&31,5);file[first+4]=0x41;
  assert!(decode_mp4(&file,TICKS_PER_SECOND/10,TICKS_PER_SECOND/10,&AtomicBool::new(false)).is_err());
 }
 #[test] fn malformed_nals_refuse_parameter_sets_and_sp_slices(){
  assert_eq!(check_sample(&idr(64),4,1).unwrap(),true);
  assert_eq!(check_sample(&p_skip(),4,1).unwrap(),false);
  assert!(check_sample(&[0,0,0,1,0x67],4,1).is_err());
  assert!(check_sample(&[0,0,0,1,0x41],4,1).is_err());
  assert!(check_sample(&[0,0,0,9,0x65],4,1).is_err());
  let mut w=BitWriter::new();w.write_ue(0);w.write_ue(3);w.rbsp_trailing();let b=nal(0x41,w);
  let mut sample=(b.len() as u32).to_be_bytes().to_vec();sample.extend(b);
  assert!(check_sample(&sample,4,1).is_err());
  let mut second=BitWriter::new();second.write_ue(1);second.write_ue(7);second.rbsp_trailing();let second=nal(0x65,second);
  let mut multi=idr(64);multi.extend_from_slice(&(second.len() as u32).to_be_bytes());multi.extend(second);
  assert!(check_sample(&multi,4,2).is_ok());
  let mut two=idr(64);two.extend(idr(64));assert!(check_sample(&two,4,1).is_err());
 }
 #[test] fn declared_sample_sizes_cannot_trigger_unbounded_reads(){
  assert!(validate_sizes(u32::MAX,1,&[],1,512).is_err());
  assert!(validate_sizes(0,2,&[1,u32::MAX],2,512).is_err());
  assert!(validate_sizes(0,2,&[300,300],2,512).is_err());
  assert!(validate_sizes(256,2,&[],2,512).is_ok());
 }
 #[test] fn hostile_nested_box_and_sample_size_are_refused_before_decode(){
  let mut hostile=Vec::new();hostile.extend_from_slice(&8u32.to_be_bytes());hostile.extend_from_slice(b"ftyp");
  hostile.extend_from_slice(&16u32.to_be_bytes());hostile.extend_from_slice(b"moov");hostile.extend_from_slice(&u32::MAX.to_be_bytes());hostile.extend_from_slice(b"meta");
  hostile.extend_from_slice(&8u32.to_be_bytes());hostile.extend_from_slice(b"mdat");
  assert!(check_top_level(&hostile).is_err());
  let original=synthetic_mp4();let index=original.windows(4).position(|w|w==b"stsz").unwrap();
  let mut fixed=original.clone();fixed[index+8..index+12].copy_from_slice(&u32::MAX.to_be_bytes());
  assert!(decode_mp4(&fixed,0,1,&AtomicBool::new(false)).is_err());
  let mut variable=original;variable[index+16..index+20].copy_from_slice(&u32::MAX.to_be_bytes());
  assert!(decode_mp4(&variable,0,1,&AtomicBool::new(false)).is_err());
 }
 #[test] fn container_and_cancellation_refuse_safely(){
  assert!(decode_mp4(b"not an mp4",0,1,&AtomicBool::new(false)).is_err());
  assert!(decode_mp4(b"not an mp4",0,1,&AtomicBool::new(true)).is_err());
  assert!(decode_mp4(&vec![0;MAX_INPUT+1],0,1,&AtomicBool::new(false)).is_err());
 }
 #[test] fn movie_inspection_and_random_access_h264(){
  let bytes = synth::h264_mp4(6, 3).unwrap();
  let mut m = Movie::open(bytes).unwrap();
  assert_eq!(m.info().width, 64);
  assert_eq!(m.info().height, 48);
  assert_eq!(m.info().codec, "h264");
  assert_eq!(m.info().frame_count, 6);
  assert!(m.info().decodable);
  let f0 = m.frame(0).unwrap();
  assert_eq!(f0.width, 64);
  assert_eq!(f0.height, 48);
  assert_eq!(f0.rgba.len(), 64 * 48 * 4);
  assert!(m.is_cached(0));
  let thumb = m.thumbnail_at(0.0, 24).unwrap();
  assert_eq!(thumb.height, 24);
  let thumbs = m.thumbnails(3, 24);
  assert_eq!(thumbs.len(), 3);
 }
 #[test] fn movie_prores_playback(){
  let bytes = synth::prores_mov(3).unwrap();
  let mut m = Movie::open(bytes).unwrap();
  assert_eq!(m.info().width, 64);
  assert_eq!(m.info().height, 48);
  assert_eq!(m.info().codec, "prores");
  assert!(m.info().decodable);
  let f = m.frame(0).unwrap();
  assert_eq!(f.width, 64);
  assert_eq!(f.height, 48);

 }
}
