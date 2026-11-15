use super::*;
use qualia_core_db::q42::emf_profiles::*;
use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
fn emitter(wavelength_m: Option<f64>) -> Emitter {
    Emitter {
        position: [0.0; 3],
        electric_amplitude: [1.0 + 2f64.powi(-40), 0.0, 0.0],
        wavelength_m,
        phase_rad: 0.0,
        authored_stokes: Some([1.0, 0.0, 0.0]),
    }
}
fn generated(emitters: &[Emitter], epoch: f64) -> Vec<u8> {
    let spec = GenerationSpec {
        origin_m: [0.0; 3],
        spacing_m: 0.25,
        brick_grid: [1; 3],
        snapshot: super::clock::from_f64_epoch(epoch).unwrap(),
        emitters,
        lineage: "synthetic explicit test input; no measured-source/Maxwell claim",
    };
    compile_generated(
        &spec,
        CompileLimits {
            file_bytes: 4 << 20,
            peak_bytes: 16 << 20,
            work_units: 50_000_000,
        },
    )
    .unwrap()
}
fn open(bytes: &[u8]) -> CurrentDocument<'_> {
    CurrentDocument::open(
        bytes,
        ReadLimits {
            file_bytes: 4 << 20,
            cold_work: 50_000_000,
            cache_bytes: 8 << 20,
        },
    )
    .unwrap()
}
#[path = "clock_tests.rs"]
mod clock_tests;
#[path = "model_tests.rs"]
mod model_tests;
#[path = "projection_tests.rs"]
mod projection_tests;
