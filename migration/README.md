# Initial source migration — 2026-10-30

The initial extraction preserves source workspaces and names packages by their
actual function. It does not establish complete source-suite migration, usable
native app integration or accepted Ecopunk assets.

| Functional crate | Current source | Migration |
|---|---|---|
| drafting-geometry | Poet `crates/geometry`, matching selected CADCraft/Craft geometry files | Existing public drafting API and all nine Rust source/test files retained; package name changed |
| profile-authoring | Poet `crates/spatial-authoring/src/profile.rs` and existing error/helper definitions | Profile generator preserved; domain-independent error/helper closure extracted; selected tests adapted |
| raster-blend-operators | PhotoCraft `crates/compose/src/psblend.rs` | Four pure channel operations adapted to a functional API; source-text oracle and edge/grid tests retained |

Per-crate notices, source maps and scope limits travel with the libraries.
Historical JSON origin receipts are evidence files, not new application data
formats. QualiaDB remains the authoritative graph/codec/simulation/render and
admission ecosystem. Source-specific compatibility algorithms do not establish
that corresponding QualiaDB numerical facilities are missing or inferior.

The root coordinator's audit index and execution receipts live at
`C:/github/ecopunk/docs/qa/implementation-2026-10-30/multimedia-migration/`.
Source workspace inventory covers eleven workspaces and 303 package manifests.
Body review is bounded and its exact scope is recorded in the audit reports.
The eleven source Git roots have no usable HEAD; source hashes, rather than
an invented upstream revision or assumed clean state, pin the inspection.

Worker staging was copied into the assembled payload. Coordinator changes to
the payload: relocate consumer-check dependency paths; exercise the raster API
in that check; clarify oracle line-ending normalization; inherit the exact
workspace Serde dependency pin. Formula and geometry
source behaviour is otherwise the selected worker candidate. Execution results
must be read from the root receipts; worker statements that tests were unrun
describe the worker stage and remain historically accurate.
