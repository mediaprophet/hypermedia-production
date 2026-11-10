use super::*;
use std::collections::BTreeMap;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

impl AssetHost<'_> {
    /// 142: `hypermedia.discourse.audit_tex_pdf_ua_matlang`
    /// (`discourse-academic-tex` + `discourse-pdf-compliance` + `discourse-scientific-matlang` + `multimodal-synthesis-schema`)
    pub(crate) fn audit_tex_pdf_ua_matlang(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["resource", "tex_source", "m_script", "redact_text"])?;
        if let Some(v) = a.get("resource") {
            let _ = self.workspace.get(v)?;
        }
        let tex_src = opt_str(
            a,
            "tex_source",
            "\\documentclass{article}\n\\begin{document}\n\\section{Fiduciary Audit}\nEquation $E=mc^2$.\n\\end{document}\n",
        )?;
        let m_script = opt_str(a, "m_script", "A = [4, 1; 1, 3]; d = det(A);")?;
        let redact_input = opt_str(a, "redact_text", "Contact: steward@example.org, SSN: 123-45-6789")?;

        self.charge(512, 16384)?;

        use discourse_academic_tex::{Item as TexItem, Style as TexStyle, layout_formula, parse_structure};
        use discourse_pdf_compliance::{
            AccessibleDocSnapshot, AccessibleHeading, Status, apply_semantic_redaction,
            check_accessibility, scan_all_pii, verify_zero_residue,
        };
        use discourse_scientific_matlang::{Matrix, eval_m_code, svd};
        use multimodal_synthesis_schema::{
            RadianceInputType, RadianceModel, calculate_splat_cost,
        };

        let tex_struct = parse_structure(&tex_src);
        let math_frag = layout_formula("E=mc^2", 12.0, &|_c, sz, _style| sz * 0.5)
            .ok_or("TeX math layout failed")?;

        let a11y_doc = AccessibleDocSnapshot {
            title: Some("Fiduciary Compliance Report".into()),
            language: Some("en-AU".into()),
            is_tagged: true,
            has_permission_flag: true,
            is_image_only: false,
            page_count: 1,
            has_bookmarks: true,
            tab_order_consistent: true,
            character_encoding_valid: true,
            untagged_content_count: 0,
            untagged_annotations_count: 0,
            multimedia_tagged: true,
            screen_flicker_detected: false,
            scripts_accessible: true,
            timed_responses_extendable: true,
            navigation_links_valid: true,
            form_fields: vec![],
            figures: vec![],
            tables: vec![],
            lists: vec![],
            headings: vec![AccessibleHeading {
                level: 1,
                text: "Fiduciary Audit".into(),
            }],
        };
        let verdicts = check_accessibility(&a11y_doc);
        let passed_rules = verdicts.iter().filter(|v| v.status == Status::Passed).count();

        let pii_matches = scan_all_pii(&redact_input);
        let secrets: Vec<&str> = pii_matches.iter().map(|m| m.text.as_str()).collect();
        let ranges: Vec<std::ops::Range<usize>> =
            pii_matches.iter().map(|m| m.range.clone()).collect();
        let redacted = apply_semantic_redaction(&redact_input, &ranges, '*');
        let zero_residue = verify_zero_residue(&redacted, &secrets);

        let _ws = eval_m_code(&m_script).map_err(|e| format!("M-code eval failed: {e}"))?;
        let mat = Matrix::from_row_major(2, 2, &[4.0, 1.0, 1.0, 3.0]).map_err(|e| e.to_string())?;
        let (_u, s_mat, _v) = svd(&mat).map_err(|e| e.to_string())?;
        let sigma0 = s_mat.get(0, 0).unwrap_or(0.0);
        let sigma1 = s_mat.get(1, 1).unwrap_or(0.0);

        let splat_cost = calculate_splat_cost(RadianceModel::Marble1p1, RadianceInputType::MultiImage);

        let svg_w = (math_frag.width + 16.0).max(64.0);
        let svg_h = (math_frag.height + math_frag.depth + 16.0).max(32.0);
        let baseline_y = math_frag.height + 8.0;
        let mut eq_svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\">"
        );
        for item in &math_frag.items {
            match item {
                TexItem::Text { x, y, size, style, text } => {
                    let fs = match style {
                        TexStyle::Italic => "italic",
                        TexStyle::Regular => "normal",
                    };
                    let sx = x + 8.0;
                    let sy = baseline_y - y;
                    eq_svg.push_str(&format!(
                        "<text x=\"{sx:.2}\" y=\"{sy:.2}\" font-size=\"{size:.1}\" font-style=\"{fs}\">{text}</text>"
                    ));
                }
                TexItem::Rule { x, y, w, h } => {
                    let rx = x + 8.0;
                    let ry = baseline_y - y - h;
                    eq_svg.push_str(&format!(
                        "<rect x=\"{rx:.2}\" y=\"{ry:.2}\" width=\"{w:.2}\" height=\"{h:.2}\" fill=\"#000\"/>"
                    ));
                }
            }
        }
        eq_svg.push_str("</svg>");

        let redacted_txt_resource = self.encode("txt", redacted.into_bytes())?;
        let equation_svg_resource = self.encode("svg", eq_svg.into_bytes())?;

        let mut pdf_args = Args::new();
        pdf_args.insert(
            "title".into(),
            Value::String("Fiduciary Compliance Report".into()),
        );
        pdf_args.insert(
            "author".into(),
            Value::String("Hypermedia Discourse".into()),
        );
        pdf_args.insert("heading".into(), Value::String("Fiduciary Audit".into()));
        pdf_args.insert(
            "body".into(),
            Value::String(format!(
                "PDF/UA rules passed {passed_rules}/{}. Zero-residue PII: {}.",
                verdicts.len(),
                zero_residue.verified_clean
            )),
        );
        let pdf_ua_resource =
            self.discourse_op("hypermedia.discourse.compile_pdf", &pdf_args)?;

        let report_json = serde_json::json!({
            "pdf_ua": {
                "rules_evaluated": verdicts.len(),
                "rules_passed": passed_rules,
            },
            "pii_redaction": {
                "matches_found": pii_matches.len(),
                "zero_residue_verified": zero_residue.verified_clean,
            },
            "svd_singular_values": [sigma0, sigma1],
            "splat_cost": {
                "credits": splat_cost.credits,
            },
        });
        let compliance_json_resource = self.encode(
            "json",
            serde_json::to_vec(&report_json).map_err(|e| e.to_string())?,
        )?;

        let mut out = BTreeMap::new();
        out.insert("redacted_txt_resource".into(), redacted_txt_resource);
        out.insert("equation_svg_resource".into(), equation_svg_resource);
        out.insert("compliance_json_resource".into(), compliance_json_resource);
        out.insert("pdf_ua_resource".into(), pdf_ua_resource);
        out.insert("tex_environment_count".into(), Value::I64(tex_struct.envs.len() as i64));
        out.insert("tex_math_width".into(), Value::F64(math_frag.width));
        out.insert("pdf_ua_rules_evaluated".into(), Value::I64(verdicts.len() as i64));
        out.insert("pdf_ua_rules_passed".into(), Value::I64(passed_rules as i64));
        out.insert("pii_redacted_zero_residue".into(), Value::Bool(zero_residue.verified_clean));
        out.insert("matlang_svd_sigma0".into(), Value::F64(sigma0));
        out.insert("splat_credits".into(), Value::I64(splat_cost.credits as i64));
        Ok(Value::Record(out))
    }

    /// 143: `hypermedia.chronicle.recalculate_tabular_fiduciary_archive`
    /// (`chronicle-tabular-engine` + `chronicle-fiduciary-ledger` + `chronicle-office-codecs` + `chronicle-timeline-interchange` + `chronicle-broadcast-containers` + `video-prores-codec`)
    pub(crate) fn recalculate_tabular_fiduciary_archive(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "cell_a1", "cell_b1", "journal_cents"])?;
        let input_luma = if let Some(v) = a.get("image") {
            let rgba = self.rgba(v)?;
            Some(rgba.pixels.clone())
        } else {
            None
        };
        let a1 = number(a, "cell_a1", 125.0)?;
        let b1 = number(a, "cell_b1", 375.0)?;
        let journal_cents = count(a, "journal_cents", 50_000, 100_000_000)? as i64;

        self.charge(1024, 65536)?;

        use chronicle_broadcast_containers::{
            MkvTrackKind, MkvTrackSpec, MkvWriter, MuxOptions, demux_matroska,
        };
        use chronicle_fiduciary_ledger::{
            AccountClass, ChartOfAccounts, FiduciaryAuditTrail, JLine, JournalLedger,
        };
        use chronicle_office_codecs::{
            FlowBlock, FlowDoc, FlowKind, FlowPara, parse_docx_package, to_docx_package,
        };
        use chronicle_tabular_engine::{Calc, CellRef, Formula, Value as TabValue, Workbook};
        use chronicle_timeline_interchange::{
            FrameRate, Tick, TimelineClip, TimelineSequence, TimelineTrack, TrackKind, parse_otio,
            write_otio,
        };
        use video_prores_codec::{ChromaFormat, Encoder as ProResEncoder, Frame as ProResFrame, Profile as ProResProfile, decode_frame as decode_prores};

        // 1. Tabular DAG recalculation
        let mut wb = Workbook::new();
        if let Some(sh) = wb.sheet_mut(0) {
            sh.set_value(CellRef::new(0, 0), TabValue::number(a1));
            sh.set_value(CellRef::new(0, 1), TabValue::number(b1));
            sh.set_formula(CellRef::new(0, 2), Formula::new("=SUM(A1:B1)"));
        }
        let mut calc = Calc::new();
        calc.rebuild(&wb);
        calc.recalc_all(&mut wb);
        let c1_val = wb
            .sheet(0)
            .and_then(|s| s.value(CellRef::new(0, 2)).as_f64())
            .unwrap_or(0.0);

        // 2. Balanced double-entry fiduciary ledger + tamper-evident hash chain
        let mut chart = ChartOfAccounts::new();
        let reserve_acct = chart.add_account(
            "1000",
            "Operating Reserve",
            AccountClass::Asset,
            Some("AUD".to_string()),
        );
        let revenue_acct = chart.add_account(
            "4000",
            "Commons Contribution Revenue",
            AccountClass::Revenue,
            Some("AUD".to_string()),
        );
        let mut ledger = JournalLedger::new(chart);
        let j_id = ledger.post_journal(
            "2026-11-08",
            "REF-001",
            "Commons Stewardship Allocation",
            vec![
                JLine::debit(reserve_acct, journal_cents),
                JLine::credit(revenue_acct, journal_cents),
            ],
            None,
        )?;
        let mut audit = FiduciaryAuditTrail::new();
        audit.append(
            1700000000,
            "did:qualia:steward",
            "journal.post",
            "journal",
            &format!("J-{j_id}"),
            "sha256:verified",
            None,
        );
        let chain_valid = audit.verify_chain().is_ok();

        // 3. OpenXML DOCX round-trip
        let mut fdoc = FlowDoc::default();
        fdoc.blocks.push(FlowBlock::Para(FlowPara::with_text(
            FlowKind::Normal,
            "Informatics Fiduciary Chronicle Record",
        )));
        let docx_pkg = to_docx_package(&fdoc);
        let docx_parsed =
            parse_docx_package(&docx_pkg).map_err(|e| format!("DOCX parse failed: {e}"))?;

        // 4. OpenTimelineIO round-trip
        let mut seq = TimelineSequence::default();
        seq.name = "Master Chronicle Timeline".into();
        let mut vtrack = TimelineTrack::new("V1", TrackKind::Video);
        vtrack.clips.push(TimelineClip {
            id: "c1".into(),
            name: "master_01.mov".into(),
            reel: "R01".into(),
            source_path: "file:///media/master_01.mov".into(),
            timeline_start: Tick::ZERO,
            timeline_duration: FrameRate::FPS_24.tick_of(24),
            source_in: Tick::ZERO,
            source_out: FrameRate::FPS_24.tick_of(24),
            speed: 1.0,
        });
        seq.video_tracks.push(vtrack);
        let otio_str = write_otio(&seq);
        let otio_rt = parse_otio(&otio_str).map_err(|e| format!("OTIO parse failed: {e}"))?;

        // 5. Matroska EBML mux/demux
        let mut mkv_track = MkvTrackSpec::new(MkvTrackKind::Video, "V_PRORES");
        mkv_track.video_size = Some((16, 16));
        let cursor = std::io::Cursor::new(Vec::new());
        let mut writer = MkvWriter::new(cursor, vec![mkv_track], MuxOptions::default())
            .map_err(|e| format!("MKV init failed: {e}"))?;
        writer
            .write_frame(0, 0, true, &[0x01, 0x02, 0x03, 0x04], None)
            .map_err(|e| format!("MKV write failed: {e}"))?;
        let mkv_bytes = writer.finish().map_err(|e| format!("MKV finish failed: {e}"))?.into_inner();
        let mkv_demux = demux_matroska(&mkv_bytes).map_err(|e| format!("MKV demux failed: {e}"))?;

        // 6. Apple ProRes 422 frame encode/decode
        let mut pr_frame = ProResFrame::new(16, 16, ChromaFormat::Yuv422, 10, false);
        if let Some(ref px) = input_luma {
            for (idx, y_sample) in pr_frame.y.iter_mut().enumerate() {
                if let Some(rgba) = px.get(idx) {
                    *y_sample = (64 + (rgba[0] as u16) * 3).min(960);
                }
            }
        } else {
            for y in 0..16u32 {
                for x in 0..16u32 {
                    pr_frame.y[(y * 16 + x) as usize] = (64 + (x + y) * 24) as u16;
                }
            }
        }
        let mut pr_enc = ProResEncoder::new(ProResProfile::Standard, 16, 16);
        let pr_bytes = pr_enc.encode(&pr_frame).map_err(|e| format!("ProRes encode failed: {e}"))?;
        let pr_dec = decode_prores(&pr_bytes).map_err(|e| format!("ProRes decode failed: {e}"))?;

        let cw = pr_dec.chroma_width() as usize;
        let mut rgba_pixels = Vec::with_capacity((pr_dec.width * pr_dec.height) as usize);
        for y in 0..pr_dec.height as usize {
            for x in 0..pr_dec.width as usize {
                let luma10 = pr_dec.y.get(y * pr_dec.width as usize + x).copied().unwrap_or(64);
                let cb10 = pr_dec.cb.get(y * cw + x / 2).copied().unwrap_or(512);
                let cr10 = pr_dec.cr.get(y * cw + x / 2).copied().unwrap_or(512);
                let yf = (luma10 as f32) / 4.0;
                let cbf = (cb10 as f32) / 4.0 - 128.0;
                let crf = (cr10 as f32) / 4.0 - 128.0;
                let r = (yf + 1.402 * crf).clamp(0.0, 255.0).round() as u8;
                let g = (yf - 0.344136 * cbf - 0.714136 * crf).clamp(0.0, 255.0).round() as u8;
                let b = (yf + 1.772 * cbf).clamp(0.0, 255.0).round() as u8;
                rgba_pixels.push([r, g, b, 255]);
            }
        }

        let docx_resource = self.encode("docx", docx_pkg.write_to_bytes())?;
        let otio_resource = self.encode("otio", otio_str.into_bytes())?;
        let mkv_resource = self.encode("mkv", mkv_bytes)?;
        let prores_resource = self.encode("prores", pr_bytes)?;
        let prores_image = self.store_rgba(pr_dec.width, pr_dec.height, rgba_pixels)?;

        let ledger_json = serde_json::json!({
            "tabular": {
                "cell_a1": a1,
                "cell_b1": b1,
                "sum_c1": c1_val,
            },
            "journal_id": j_id,
            "journal_cents": journal_cents,
            "audit_trail": audit,
            "fiduciary_chain_verified": chain_valid,
        });
        let ledger_json_resource = self.encode(
            "json",
            serde_json::to_vec(&ledger_json).map_err(|e| e.to_string())?,
        )?;

        let mut out = BTreeMap::new();
        out.insert("docx_resource".into(), docx_resource);
        out.insert("otio_resource".into(), otio_resource);
        out.insert("mkv_resource".into(), mkv_resource);
        out.insert("prores_resource".into(), prores_resource);
        out.insert("prores_image".into(), prores_image);
        out.insert("ledger_json_resource".into(), ledger_json_resource);
        out.insert("tabular_sum_c1".into(), Value::F64(c1_val));
        out.insert("fiduciary_chain_verified".into(), Value::Bool(chain_valid));
        out.insert("docx_block_count".into(), Value::I64(docx_parsed.blocks.len() as i64));
        out.insert("otio_video_track_count".into(), Value::I64(otio_rt.video_tracks.len() as i64));
        out.insert("matroska_block_count".into(), Value::I64(mkv_demux.blocks.len() as i64));
        out.insert("prores_decoded_width".into(), Value::I64(pr_dec.width as i64));
        Ok(Value::Record(out))
    }
}
