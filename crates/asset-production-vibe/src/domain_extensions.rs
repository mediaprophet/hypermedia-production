//! Wave 13 Categorical Domain Extension Morphisms (indices `120..=143`):
//! Wires the 36 categorical domain crates and 6 in-place upgraded
//! domain crates across all 12 ontological categories (`locus`, `manifold`, `surface`,
//! `radiance`, `glyph`, `layout`, `kinetics`, `acoustics`, `electromagnetics`, `geodesy`,
//! `discourse`, `chronicle`) into `AssetHost` for agentic VibeScript and `synth` execution.
//!
//! All provenance, cryptographic verification, and document compliance operations are
//! grounded in international human rights instruments (UDHR Articles 12, 18, 19, 27;
//! ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties.

use super::*;
use vibe::Value;

#[path = "locus_sketch_warp_geospatial.rs"]
mod locus_sketch_warp_geospatial;
#[path = "manifold_assembly_bim_erosion.rs"]
mod manifold_assembly_bim_erosion;
#[path = "surface_pbr_brush.rs"]
mod surface_pbr_brush;
#[path = "radiance_raw_cdl_trace.rs"]
mod radiance_raw_cdl_trace;
#[path = "glyph_knuth_plass.rs"]
mod glyph_knuth_plass;
#[path = "layout_autolayout_foliation_nodegraph.rs"]
mod layout_autolayout_foliation_nodegraph;
#[path = "kinetics_fem_garment_blocks.rs"]
mod kinetics_fem_garment_blocks;
#[path = "acoustics_rf_kriging_cpm.rs"]
mod acoustics_rf_kriging_cpm;
#[path = "discourse_chronicle_fiduciary.rs"]
mod discourse_chronicle_fiduciary;

impl AssetHost<'_> {
    /// Dispatches Wave 13 domain extension morphisms (`120..=143`).
    pub(crate) fn domain_extension_op(
        &mut self,
        canonical_id: &str,
        a: &Args,
    ) -> Result<Value, String> {
        match canonical_id {
            "hypermedia.locus.solve_parametric_sketch" => self.solve_parametric_sketch(a),
            "hypermedia.locus.pathfinder_warp_envelope" => self.pathfinder_warp_envelope(a),
            "hypermedia.locus.transcode_geospatial_vector" => self.transcode_geospatial_vector(a),
            "hypermedia.manifold.sweep_assembly_profile" => self.sweep_assembly_profile(a),
            "hypermedia.manifold.compile_bim_ifc4" => self.compile_bim_ifc4(a),
            "hypermedia.manifold.erode_geomorphic_heightfield" => {
                self.erode_geomorphic_heightfield(a)
            }
            "hypermedia.surface.bake_pbr_materialx" => self.bake_pbr_materialx(a),
            "hypermedia.surface.expand_procedural_brush" => self.expand_procedural_brush(a),
            "hypermedia.radiance.develop_camera_raw_hdr" => self.develop_camera_raw_hdr(a),
            "hypermedia.radiance.grade_cdl_scopes" => self.grade_cdl_scopes(a),
            "hypermedia.radiance.trace_vector_contours" => self.trace_vector_contours(a),
            "hypermedia.glyph.compose_knuth_plass_paragraph" => {
                self.compose_knuth_plass_paragraph(a)
            }
            "hypermedia.layout.solve_flexbox_grid_constraints" => {
                self.solve_flexbox_grid_constraints(a)
            }
            "hypermedia.layout.typeset_paged_foliation" => self.typeset_paged_foliation(a),
            "hypermedia.layout.compile_dataflow_nodegraph" => self.compile_dataflow_nodegraph(a),
            "hypermedia.kinetics.solve_continuum_fem_lbm" => self.solve_continuum_fem_lbm(a),
            "hypermedia.kinetics.drape_orthotropic_garment" => self.drape_orthotropic_garment(a),
            "hypermedia.kinetics.simulate_causal_block_diagram" => {
                self.simulate_causal_block_diagram(a)
            }
            "hypermedia.kinetics.schedule_cpm_evm_montecarlo" => {
                self.schedule_cpm_evm_montecarlo(a)
            }
            "hypermedia.acoustics.analyze_ebu_r128_camelot_daw" => {
                self.analyze_ebu_r128_camelot_daw(a)
            }
            "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa" => {
                self.sense_wifi_csi_vitals_aoa(a)
            }
            "hypermedia.geodesy.kriging_variogram_morphometry" => {
                self.kriging_variogram_morphometry(a)
            }
            "hypermedia.discourse.audit_tex_pdf_ua_matlang" => self.audit_tex_pdf_ua_matlang(a),
            "hypermedia.chronicle.recalculate_tabular_fiduciary_archive" => {
                self.recalculate_tabular_fiduciary_archive(a)
            }
            other => Err(format!("Unknown domain extension morphism: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn wave13_domain_extension_morphisms_roundtrip() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        let ids = [
            "hypermedia.locus.solve_parametric_sketch",
            "hypermedia.locus.pathfinder_warp_envelope",
            "hypermedia.locus.transcode_geospatial_vector",
            "hypermedia.manifold.sweep_assembly_profile",
            "hypermedia.manifold.compile_bim_ifc4",
            "hypermedia.manifold.erode_geomorphic_heightfield",
            "hypermedia.surface.bake_pbr_materialx",
            "hypermedia.surface.expand_procedural_brush",
            "hypermedia.radiance.develop_camera_raw_hdr",
            "hypermedia.radiance.grade_cdl_scopes",
            "hypermedia.radiance.trace_vector_contours",
            "hypermedia.glyph.compose_knuth_plass_paragraph",
            "hypermedia.layout.solve_flexbox_grid_constraints",
            "hypermedia.layout.typeset_paged_foliation",
            "hypermedia.layout.compile_dataflow_nodegraph",
            "hypermedia.kinetics.solve_continuum_fem_lbm",
            "hypermedia.kinetics.drape_orthotropic_garment",
            "hypermedia.kinetics.simulate_causal_block_diagram",
            "hypermedia.kinetics.schedule_cpm_evm_montecarlo",
            "hypermedia.acoustics.analyze_ebu_r128_camelot_daw",
            "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
            "hypermedia.geodesy.kriging_variogram_morphometry",
            "hypermedia.discourse.audit_tex_pdf_ua_matlang",
            "hypermedia.chronicle.recalculate_tabular_fiduciary_archive",
        ];

        for id in ids {
            let res = host
                .domain_extension_op(id, &Args::new())
                .unwrap_or_else(|e| panic!("{id} failed: {e}"));
            let rec = record(&res).expect("expected record output");
            assert!(!rec.is_empty(), "{id} returned empty record");
        }
    }
}
