use super::*;
use std::sync::atomic::AtomicBool;

fn value(json:serde_json::Value)->Value {
    match json {
        serde_json::Value::Null=>Value::Null,serde_json::Value::Bool(v)=>Value::Bool(v),
        serde_json::Value::String(v)=>Value::String(v),
        serde_json::Value::Number(v)=>if let Some(n)=v.as_i64(){Value::I64(n)}else if let Some(n)=v.as_u64(){Value::U64(n)}else{Value::F64(v.as_f64().unwrap())},
        serde_json::Value::Array(v)=>Value::List(v.into_iter().map(value).collect()),
        serde_json::Value::Object(v)=>Value::Record(v.into_iter().map(|(k,v)|(k,value(v))).collect()),
    }
}
fn interpretation()->serde_json::Value {
    serde_json::json!({"colour":{"model":"spectral_only"},"hilbert_order":2,
        "spectral_semantics":"amplitude_centroid_legacy_sigma",
        "timebase":{"epoch":{"scale":"tai","seconds":10,"nanoseconds":250000000},
            "ticks_per_second_num":48000,"ticks_per_second_den":1,"slice_start_tick":100,"slice_step_ticks":480}})
}
fn compile_args()->Args {
    record(&value(serde_json::json!({"grid_dims":[1,1,1],"voxel_size":0.5,
        "sources":[{"position":[-1,0,0],"e_amplitude":[100,0,0],"wavelength_nm":1000000}],
        "interpretation":interpretation()}))).unwrap().clone()
}
fn host<'a>(inputs:&'a[Resource],cancel:&'a AtomicBool,limits:&'a ToolLimits)->AssetHost<'a>{
    AssetHost{workspace:AssetWorkspace::default(),inputs,cancel,limits,work:0}
}
fn call(host:&mut AssetHost<'_>,id:&str,args:Args)->Result<Value,String>{host.invoke(id,&Value::Record(args))}

#[test]
fn compile_v3_preserves_original_pcm_bits_and_reports_typed_metadata_while_default_stays_v2(){
    let samples=[0.0f32,-0.0,2.5,-3.75];
    let inputs=[Resource{key:"pressure".into(),kind:"pcm_f32le".into(),bytes:samples.iter().flat_map(|s|s.to_le_bytes()).collect()}];
    let cancel=AtomicBool::new(false);let limits=ToolLimits::default();let mut host=host(&inputs,&cancel,&limits);
    let mut args=compile_args();let mut exact_clock=interpretation();
    exact_clock["timebase"]["epoch"]["seconds"]=serde_json::json!(9_007_199_254_740_993i64);
    exact_clock["timebase"]["slice_start_tick"]=serde_json::json!(9_007_199_254_740_993i64);
    args.insert("interpretation".into(),value(exact_clock));
    args.insert("audio_tracks".into(),value(serde_json::json!([{"input":"pressure","spec":{
        "id":"pressure","purpose":"recorded","channels":1,"channel_layout":"mono","sample_rate_num":1000000,
        "sample_rate_den":1,"start_tick":-24,"units":{"unit":"pascal"},"units_per_sample":0.1}}])));
    let emf=call(&mut host,"hypermedia.electromagnetics.compile_emf",args).unwrap();
    let bytes=host.workspace.encoded(&emf).unwrap().bytes.clone();let reader=EmfContainerReader::parse(&bytes).unwrap();
    assert_eq!(reader.header().version,3);let extension=reader.extension().unwrap();
    assert_eq!(extension.metadata().field.timebase.epoch.nanoseconds,250000000);
    assert_eq!(extension.audio_payload(0).unwrap(),inputs[0].bytes.as_slice());
    for (frame,sample) in samples.iter().enumerate(){assert_eq!(extension.audio_sample(0,frame as u64,0).unwrap().to_bits(),sample.to_bits());}
    let report=call(&mut host,"hypermedia.electromagnetics.verify_emf",[("emf".into(),emf)].into_iter().collect()).unwrap();
    let report=record(&report).unwrap();assert_eq!(report["container_version"],Value::U64(3));
    assert_eq!(report["audio_track_count"],Value::U64(1));assert_eq!(report["audio_integrity"],Value::String("full_blake3_256".into()));
    let extension=record(&report["extension"]).unwrap();let field=record(&extension["field"]).unwrap();
    let timebase=record(&field["timebase"]).unwrap();let epoch=record(&timebase["epoch"]).unwrap();
    assert_eq!(epoch["seconds"],Value::I64(9_007_199_254_740_993));
    assert_eq!(timebase["slice_start_tick"],Value::I64(9_007_199_254_740_993));
    let mut plain=compile_args();plain.remove("interpretation");
    let old=call(&mut host,"hypermedia.electromagnetics.compile_emf",plain).unwrap();
    let reader=EmfContainerReader::parse(&host.workspace.encoded(&old).unwrap().bytes).unwrap();
    assert_eq!(reader.header().version,2);assert!(reader.extension().is_none());
}

#[test]
fn compile_v3_refuses_ambiguous_metadata_and_both_previews_refuse_unsupported_rgb_conversion(){
    let cancel=AtomicBool::new(false);let limits=ToolLimits::default();let mut host=host(&[],&cancel,&limits);
    let mut bad=compile_args();bad.insert("epoch_s".into(),Value::F64(50.));
    assert!(call(&mut host,"hypermedia.electromagnetics.compile_emf",bad).unwrap_err().contains("contradicts"));
    let mut bad=compile_args();let mut unknown=interpretation();unknown["colour"]["model"]=serde_json::json!("unknown");bad.insert("interpretation".into(),value(unknown));
    assert!(call(&mut host,"hypermedia.electromagnetics.compile_emf",bad).is_err());
    let mut short_wave=compile_args();
    if let Value::List(sources)=short_wave.get_mut("sources").unwrap(){
        if let Value::Record(source)=&mut sources[0]{source.insert("wavelength_nm".into(),Value::F64(0.01));}
    }
    assert!(call(&mut host,"hypermedia.electromagnetics.compile_emf",short_wave.clone()).is_ok(),"v3 retains sub-nanometre physical wavelengths");
    short_wave.remove("interpretation");
    assert!(call(&mut host,"hypermedia.electromagnetics.compile_emf",short_wave).is_err(),"v2 emitter bounds remain unchanged");
    let mut args=compile_args();let mut rgb=interpretation();rgb["colour"]=serde_json::json!({"model":"rgb_preview","primaries":"rec709","white_point":"d65","transfer":"srgb","observer":"cie1931_2deg","reference_white_nits":80});
    args.insert("interpretation".into(),value(rgb));let emf=call(&mut host,"hypermedia.electromagnetics.compile_emf",args).unwrap();
    let args:Args=[("emf".into(),emf)].into_iter().collect();
    assert!(call(&mut host,"hypermedia.electromagnetics.verify_emf",args.clone()).is_ok());
    for id in ["hypermedia.electromagnetics.render_spectral_slice","hypermedia.electromagnetics.raymarch_vox0_dda"]{
        assert!(call(&mut host,id,args.clone()).unwrap_err().contains("unsupported"));
    }
}

#[test]
fn both_emf_previews_select_same_key_time_slices_and_default_to_zero(){
    let field:EmfFieldInterpretation=serde_json::from_value(interpretation()).unwrap();
    let mut writer=EmfContainerWriter::new([0.;3],1.,[1;3],field.timebase.epoch.header_seconds());
    for (time,wavelength) in [(1,650.),(0,450.)]{
        let mut cells=[EmfCellRecord64::default();512];for cell in &mut cells {cell.wavelength_nm=wavelength;cell.e_field=[1000.,0.,0.];cell.confidence_alpha=1.;}
        writer.add_brick_chunk(0,&cells,time,1,0).unwrap();
    }
    let bytes=writer.to_bytes_v3(&field,&[],EmfExtensionLimits::conservative()).unwrap();
    let cancel=AtomicBool::new(false);let limits=ToolLimits::default();let mut host=host(&[],&cancel,&limits);
    let emf=host.admit_encoded("emf",bytes).unwrap();
    for id in ["hypermedia.electromagnetics.render_spectral_slice","hypermedia.electromagnetics.raymarch_vox0_dda"]{
        let mut images=Vec::new();
        for time in [None,Some(0),Some(1)]{
            let mut args:Args=[("emf".into(),emf.clone()),("width".into(),Value::U64(8)),("height".into(),Value::U64(8))].into_iter().collect();
            if let Some(time)=time {args.insert("time_slice".into(),Value::U64(time));}
            let output=call(&mut host,id,args).unwrap();let rec=record(&output).unwrap();
            assert_eq!(rec["source_time_slice"],Value::U64(time.unwrap_or(0)));
            assert!(text(&rec["projection_scope"]).unwrap().contains("derivative"));
            let image=rec.get("image").unwrap_or(&output);images.push(host.rgba(image).unwrap().pixels.clone());
        }
        assert_eq!(images[0],images[1],"default selects t0: {id}");
        assert_ne!(images[0],images[2],"explicit t1 changes visible output: {id}");
        assert!(images[0].iter().any(|p|p[0]>0||p[1]>0||p[2]>0));
        let missing=[("emf".into(),emf.clone()),("time_slice".into(),Value::U64(2))].into_iter().collect();
        assert!(call(&mut host,id,missing).unwrap_err().contains("no stored chunks"));
    }
}


#[test]
fn v3_admits_combined_owned_tracks_workspace_inputs_and_output_before_pcm_copy(){
    let input=Resource{key:"raw".into(),kind:"pcm_f32le".into(),bytes:(0..8192).flat_map(|_|0.25f32.to_le_bytes()).collect()};
    let inputs=[input];let cancel=AtomicBool::new(false);
    let mut args=compile_args();
    let tracks=(0..2).map(|index|serde_json::json!({"input":"raw","spec":{
        "id":format!("track_{index}"),"purpose":"recorded","channels":1,"channel_layout":"mono",
        "sample_rate_num":48000,"sample_rate_den":1,"start_tick":0,"units":{"unit":"pascal"},"units_per_sample":1}})).collect::<Vec<_>>();
    args.insert("audio_tracks".into(),value(serde_json::json!(tracks)));
    let limits=ToolLimits{resource_bytes:256*1024,total_resource_bytes:256*1024,..ToolLimits::default()};
    let mut limited=host(&inputs,&cancel,&limits);let marker=limited.admit_encoded("bin",vec![0;64*1024]).unwrap();
    let error=call(&mut limited,"hypermedia.electromagnetics.compile_emf",args.clone()).unwrap_err();
    assert!(error.contains("combined payload peak"),"{error}");
    assert_eq!(limited.work,1,"only invoke's dispatch unit is charged; no sample scan/copy or field work");
    assert_eq!(limited.workspace.entries.len(),1);assert_eq!(limited.workspace.encoded(&marker).unwrap().bytes.len(),64*1024);
    let larger=ToolLimits{total_resource_bytes:512*1024,..limits};let mut admitted=host(&inputs,&cancel,&larger);
    admitted.admit_encoded("bin",vec![0;64*1024]).unwrap();
    let emf=call(&mut admitted,"hypermedia.electromagnetics.compile_emf",args).unwrap();
    let bytes=&admitted.workspace.encoded(&emf).unwrap().bytes;
    assert!(bytes.len()+64*1024<256*1024,"final output alone would have fit the smaller workspace; transient peak was the reason for refusal");
    let parsed=EmfContainerReader::parse(bytes).unwrap();let extension=parsed.extension().unwrap();
    assert_eq!(extension.metadata().audio_tracks.len(),2);
    for track in 0..2 {assert_eq!(extension.audio_payload(track).unwrap(),inputs[0].bytes.as_slice());}
}

#[test]
fn v3_metadata_aggregate_is_admitted_before_json_clone_or_typed_deserialization(){
    let mut field=record(&value(interpretation())).unwrap().clone();
    // Every value is under the per-string cap and the tree is shallow, but their
    // aggregate would formerly allocate hundreds of KiB before serde rejected it.
    for index in 0..32 {field.insert(format!("unexpected_{index}"),Value::String("x".repeat(4096)));}
    let mut args=compile_args();args.insert("interpretation".into(),Value::Record(field));
    let cancel=AtomicBool::new(false);let limits=ToolLimits::default();let mut host=host(&[],&cancel,&limits);
    let error=call(&mut host,"hypermedia.electromagnetics.compile_emf",args).unwrap_err();
    assert!(error.contains("aggregate metadata"),"{error}");
    assert_eq!(host.work,1,"only the dispatch unit is charged");assert!(host.workspace.entries.is_empty());
}


fn sealed_v2()->Vec<u8>{
    let mut writer=EmfContainerWriter::new([0.;3],1.,[1;3],0.);
    let mut cells=[EmfCellRecord64::default();512];
    for cell in &mut cells {cell.wavelength_nm=550.;cell.e_field=[100.,0.,0.];}
    writer.add_brick_chunk(0,&cells,0,1,0).unwrap();writer.to_bytes().unwrap()
}

#[test]
fn emf_source_resolution_admits_encoded_or_retained_backing_and_clone_before_copy(){
    let bytes=sealed_v2();let cancel=AtomicBool::new(false);
    let limits=ToolLimits{resource_bytes:128*1024,total_resource_bytes:96*1024,..ToolLimits::default()};
    for retained in [false,true]{
        let mut inputs=vec![Resource{key:"unrelated".into(),kind:"bin".into(),bytes:vec![0;32*1024]}];
        if retained {inputs.push(Resource{key:"field".into(),kind:"emf".into(),bytes:bytes.clone()});}
        let mut host=host(&inputs,&cancel,&limits);host.admit_encoded("bin",vec![0;16*1024]).unwrap();
        let source=if retained{Value::String("field".into())}else{host.admit_encoded("emf",bytes.clone()).unwrap()};
        let entries=host.workspace.entries.len();
        let error=host.resolve_emf_bytes(&source).unwrap_err();
        assert!(error.contains("combined payload peak"),"{error}");
        assert_eq!(host.work,0);assert_eq!(host.workspace.entries.len(),entries);
    }
    let oversized=[Resource{key:"large".into(),kind:"emf".into(),bytes}];
    let small=ToolLimits{resource_bytes:1024,..limits};let mut host=host(&oversized,&cancel,&small);
    assert!(host.resolve_emf_bytes(&Value::String("large".into())).unwrap_err().contains("before copy"));
    assert_eq!(host.work,0);
}

#[test]
fn emf_preview_admits_clone_plus_outputs_and_raymarch_checks_actual_phases(){
    let inputs=[Resource{key:"field".into(),kind:"emf".into(),bytes:sealed_v2()}];
    let cancel=AtomicBool::new(false);
    let limits=ToolLimits{resource_bytes:128*1024,total_resource_bytes:96*1024,..ToolLimits::default()};
    let args=||[("emf".into(),Value::String("field".into())),("width".into(),Value::U64(128)),("height".into(),Value::U64(128))].into_iter().collect::<Args>();
    let mut slice=host(&inputs,&cancel,&limits);slice.admit_encoded("bin",vec![0;16*1024]).unwrap();
    assert!(slice.resolve_emf_bytes(&Value::String("field".into())).is_ok(),"source-only admission fits");
    let error=call(&mut slice,"hypermedia.electromagnetics.render_spectral_slice",args()).unwrap_err();
    assert!(error.contains("combined payload peak"),"{error}");assert_eq!(slice.work,1,"only the dispatch unit is charged");assert_eq!(slice.workspace.entries.len(),1);
    let mut ray=host(&inputs,&cancel,&limits);ray.admit_encoded("bin",vec![0;16*1024]).unwrap();
    let mut small_view=args();small_view.insert("width".into(),Value::U64(8));small_view.insert("height".into(),Value::U64(8));
    let error=call(&mut ray,"hypermedia.electromagnetics.raymarch_vox0_dda",small_view).unwrap_err();
    assert!(error.contains("combined payload peak"),"{error}");assert_eq!(ray.work,1,"only dispatch: clone plus bounded grid refuses before copy/parse");
    let render_limit=ToolLimits{total_resource_bytes:128*1024,..limits.clone()};
    let mut render=host(&inputs,&cancel,&render_limit);render.admit_encoded("bin",vec![0;16*1024]).unwrap();
    let error=call(&mut render,"hypermedia.electromagnetics.raymarch_vox0_dda",args()).unwrap_err();
    assert!(error.contains("combined payload peak"),"{error}");assert_eq!(render.work,1,"only dispatch: later grid+pixels peak is preflighted before source copy or field work");
    assert_eq!(render.workspace.entries.len(),1);
    let sufficient=ToolLimits{total_resource_bytes:256*1024,..limits};
    for id in ["hypermedia.electromagnetics.render_spectral_slice","hypermedia.electromagnetics.raymarch_vox0_dda"]{
        let mut admitted=host(&inputs,&cancel,&sufficient);admitted.admit_encoded("bin",vec![0;16*1024]).unwrap();
        let output=call(&mut admitted,id,args()).unwrap();let rec=record(&output).unwrap();
        let image=rec.get("image").unwrap_or(&output);assert_eq!(admitted.rgba(image).unwrap().pixels.len(),128*128);
    }
    let procedural=ToolLimits{resource_bytes:128*1024,total_resource_bytes:128*1024,..ToolLimits::default()};
    let mut proc=host(&[],&cancel,&procedural);proc.admit_encoded("bin",vec![0;16*1024]).unwrap();
    let error=call(&mut proc,"hypermedia.electromagnetics.raymarch_vox0_dda",Args::new()).unwrap_err();
    assert!(error.contains("combined payload peak"),"procedural dense grid also admits fixed backing: {error}");
    assert_eq!(proc.work,1,"only the dispatch unit is charged");
}


#[test]
fn default_v2_compile_admits_combined_cells_and_output_before_field_work(){
    let cancel=AtomicBool::new(false);let mut args=compile_args();args.remove("interpretation");
    let limits=ToolLimits{resource_bytes:128*1024,total_resource_bytes:48*1024,..ToolLimits::default()};
    let mut small=host(&[],&cancel,&limits);
    let error=call(&mut small,"hypermedia.electromagnetics.compile_emf",args.clone()).unwrap_err();
    assert!(error.contains("combined payload peak"),"{error}");
    assert_eq!(small.work,1,"only the dispatch unit is charged");assert!(small.workspace.entries.is_empty());
    let larger=ToolLimits{total_resource_bytes:192*1024,..limits};let mut admitted=host(&[],&cancel,&larger);
    let emf=call(&mut admitted,"hypermedia.electromagnetics.compile_emf",args).unwrap();
    let reader=EmfContainerReader::parse(&admitted.workspace.encoded(&emf).unwrap().bytes).unwrap();
    assert_eq!(reader.header().version,2);assert!(reader.extension().is_none());
}
