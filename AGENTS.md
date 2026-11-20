# Hypermedia-Production — Agent Coordination & Licensing Guide

## Intellectual Property & Licensing Notice

QualiaDB, Webizen AI Synth, Hypermedia-Production, and all Qualia-specific architectures, integrations, modifications, and extensions are governed by:

**Copyright (c) 2026 Timothy Charles Holborn**  
**https://www.linkedin.com/in/ubiquitous/ | timothy.holborn@gmail.com**  
**License:** Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)  
Full terms: [`LICENSE`](LICENSE), [`LICENSE-QUALIA`](LICENSE-QUALIA), and [Creative Commons Legal Code](https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode).  
Upstream historical code retains its respective MIT/Apache-2.0 attribution notices where applicable inside crate-local notice files.

All participating AI agents and contributors must strictly honor this copyright, moral rights, and non-commercial attribution. Never replace root `LICENSE` or `Cargo.toml` license declarations with Apache-2.0 or MIT.

---

## Ecosystem Role & Architectural Invariants

`hypermedia-production` (`C:\github\hypermedia-production`) is the studio and domain-engine workspace consumed by **Webizen AI Synth** (`C:\github\synth`) and backed by **QualiaDB** (`C:\github\qualiaDB`) and **Etalon** (`C:\github\etalon`).

Read [`C:\github\synth\AGENTS.md`](../synth/AGENTS.md) and [`C:\github\synth\agent-instructions\README.md`](../synth/agent-instructions/README.md) before modifying or claiming a morphism is missing.

### 1. 150-Morphism Catalog & Function Registry
- **Catalog Owner:** [`crates/asset-production-vibe/src/operations_catalog.rs`](crates/asset-production-vibe/src/operations_catalog.rs) (`OPERATIONS`) and [`crates/asset-production-vibe/src/ontology/`](crates/asset-production-vibe/src/ontology/) (`ALL_MORPHISMS`). Counts must remain strictly aligned (`OPERATIONS.len() == ontology::MORPHISMS.len() == 150`).
- **Audit Index:** [`audit-docs/FUNCTIONS.md`](audit-docs/FUNCTIONS.md) and [`audit-docs/modules/`](audit-docs/modules/) index all functions across the workspace. Grep `audit-docs/FUNCTIONS.md` before adding or duplicating any capability.

### 2. Holographic 10D v2 Primacy (`HOLO` `0x0306`) & Two-Tier Constituency
- **Mandatory Holographic Seal:** Every `.10d` container sealed in [`crates/asset-production/src/compile_10d.rs`](crates/asset-production/src/compile_10d.rs) (`compile_10d`, `compile_10d_pbr`, and `convert_3d_asset_or_scene_with_license`) calls `qualia_core_db::container_10d::upgrade_legacy_10d_to_holographic_v2` so that `FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE = 0x0306` (`HOLO`, containing `M64HoloRecord` and `EulerianWavefieldRecord`) is always present.
- **Continuous Phase-Space Over Triangle-Soup Drift:** Continuous fields, scale-space harmonic LODs ($k=0..K$, with $k=0$ base envelope, $k=1..3$ structural detail, $k \ge 4$ procedural surface microstructure), Hamiltonian stationary-action trajectories ($\delta S = 0$), and hyperbolic $v=2$ Möbius morphogenesis ($w(z) = \frac{az+b}{cz+d}$) are primary. When explicit triangle vertices are omitted (`vertices.is_empty()`), `qualia-core-db`'s `decode_load_10d` synthesizes a 12-triangle bounding-box proxy lens from the `HOLO` bounds for Etalon's `< 32`-step raymarching shader (`holographic_standalone.wgsl`).
- **Two-Tier `.10d` Constituency (`AREF` / `IOVR` / `INST`):** Multi-asset scenes and assemblies reference external leaf `.10d` containers via `AREF` (Asset Reference Registry), `IOVR` (Instance Override Table), and `INST` (PGA Motor Transform Table) rather than flattening leaf geometry into a monolithic mesh.

### 3. 3D Scene/Asset Conversion & Source-Asset Licensing Ontology
- [`convert_3d_asset_or_scene_with_license`](crates/asset-production/src/compile_10d.rs) converts external 3D formats (`.glb`, `.gltf`, `.obj`, `.stl`) into Holographic 10D v2 `.10d` + `.q42` packages.
- It enforces `C:\github\qualiaDB\core-ontologies\source-asset-licensing.n3` fail-closed via `SourceAssetLicenseMetadata::validate_for_10d_conversion()`, seals 6-column `NQuin` provenance triples into `SECTION_KIND_SYMBOLIC_RDF_CBOR_LD` (`CBLD`) and `ProvenanceSection` (`PROV`), upgrades the container to Holographic 10D v2 (`HOLO` `0x0306`), and emits a companion `.n3` / `.q42` licensing graph.

### 4. 7-Layer Multi-Scale Computational Hierarchy (`L0`–`L6`) & No-ASCII Policy
- Every `yaml-ld-q42` v2 manifest (`manifest.q42.yaml`) maps across `L0` (Continuum), `L1` (Rigid/Articulated PGA/XPBD/SKL2), `L2` (Soft-Body/Fluid), `L3` (Molecular/Chemical), `L4` (Cellular/Biological), `L5` (Ecological/Geomorphic), and `L6` (**Quantum Physics** / `L6QuantumPhysics` — encompassing quantum dynamics, quantum physics, and quantum chemistry).
- Honor the **No-ASCII Policy**: textual files and manifests use UTF-8 or UTF-16; binary semantic payloads inside `.10d` and `.q42` containers use deterministic CBOR / CBOR-LD (`CBLD`).

### 5. Memory Budgets vs. 42 MiB Prolog Sentinel
- The **42 MiB Prolog Sentinel** applies strictly to `.q42` symbolic reasoning hot-paths (`FixedArena` zero-heap evaluation).
- Graphics and scene containers (`.10d`, `.hmc`, `.emf`, `.m64`) and LLM weight vaults (`.p64`) are governed by explicit streaming/container budgets (`production16` admits 128 MiB per `.10d` ring, 2 GiB aggregate resources, and 2,304 MiB package budget).
