# `crates/manifold-bim-csg`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`manifold-bim-csg`

UI-agnostic parametric BIM and solid geometry engine:
- Work-metered, non-recursive 3D BSP CSG booleans (`csg`)
- ISO 10303-21 STEP physical file parser (`step`)
- IFC4 / ISO 16739 architectural entity exporter (`ifc`)
- Parametric multi-layer walls, stairs, and MEP runs (`parametric`)_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `add` | 2 | `crates/manifold-bim-csg/src/csg.rs:10` | - |  |
| `sub` | 2 | `crates/manifold-bim-csg/src/csg.rs:15` | - |  |
| `mul` | 2 | `crates/manifold-bim-csg/src/csg.rs:20` | - |  |
| `dot` | 2 | `crates/manifold-bim-csg/src/csg.rs:25` | - |  |
| `cross` | 2 | `crates/manifold-bim-csg/src/csg.rs:30` | - |  |
| `impl Default for Meter::default` | 0 | `crates/manifold-bim-csg/src/csg.rs:53` | - |  |
| `impl Meter::new` | 2 | `crates/manifold-bim-csg/src/csg.rs:64` | - |  |
| `impl Meter::work` | 2 | `crates/manifold-bim-csg/src/csg.rs:68` | - |  |
| `impl Meter::split` | 1 | `crates/manifold-bim-csg/src/csg.rs:73` | - |  |
| `impl Poly::of` | 1 | `crates/manifold-bim-csg/src/csg.rs:88` | - |  |
| `impl Poly::flip` | 1 | `crates/manifold-bim-csg/src/csg.rs:101` | - |  |
| `impl Poly::triangles` | 1 | `crates/manifold-bim-csg/src/csg.rs:107` | - |  |
| `divide` | 3 | `crates/manifold-bim-csg/src/csg.rs:131` | - |  |
| `impl Bsp::of` | 2 | `crates/manifold-bim-csg/src/csg.rs:197` | - |  |
| `impl Bsp::node` | 1 | `crates/manifold-bim-csg/src/csg.rs:203` | graph |  |
| `impl Bsp::build` | 3 | `crates/manifold-bim-csg/src/csg.rs:213` | - |  |
| `impl Bsp::clip` | 3 | `crates/manifold-bim-csg/src/csg.rs:268` | - |  |
| `impl Bsp::clip_to` | 3 | `crates/manifold-bim-csg/src/csg.rs:302` | - |  |
| `impl Bsp::invert` | 1 | `crates/manifold-bim-csg/src/csg.rs:309` | - |  |
| `impl Bsp::polygons` | 1 | `crates/manifold-bim-csg/src/csg.rs:318` | - |  |
| `bounds` | 1 | `crates/manifold-bim-csg/src/csg.rs:323` | - |  |
| `subtract` | 3 | `crates/manifold-bim-csg/src/csg.rs:336` | - | Subtracts `void` solid from `solid`. |
| `union` | 3 | `crates/manifold-bim-csg/src/csg.rs:363` | - | Unions two solids together. |
| `intersect` | 3 | `crates/manifold-bim-csg/src/csg.rs:384` | - | Intersects two solids. |
| `volume` | 1 | `crates/manifold-bim-csg/src/csg.rs:410` | computational geometry | Signed volume of a closed triangle mesh. |
| `area` | 1 | `crates/manifold-bim-csg/src/csg.rs:415` | computational geometry | Surface area of a triangle mesh. |
| `cuboid` | 2 | `crates/manifold-bim-csg/src/csg.rs:428` | - |  |
| `csg_subtract_pocket` | 0 | `crates/manifold-bim-csg/src/csg.rs:444` | - |  |
| `csg_union_and_intersect` | 0 | `crates/manifold-bim-csg/src/csg.rs:454` | - |  |
| `meter_budget_exhaustion` | 0 | `crates/manifold-bim-csg/src/csg.rs:470` | - |  |
| `step_string` | 1 | `crates/manifold-bim-csg/src/ifc.rs:9` | - | Formats a string as an ISO 10303-21 STEP string literal with Unicode escapes. |
| `ifc_guid` | 2 | `crates/manifold-bim-csg/src/ifc.rs:29` | - | Generates a deterministic 22-character IFC GlobalId from a 64-bit integer seed. |
| `impl Default for IfcWriter::default` | 0 | `crates/manifold-bim-csg/src/ifc.rs:48` | - |  |
| `impl IfcWriter::new` | 0 | `crates/manifold-bim-csg/src/ifc.rs:54` | - |  |
| `impl IfcWriter::next_id` | 1 | `crates/manifold-bim-csg/src/ifc.rs:61` | - |  |
| `impl IfcWriter::write_entity` | 3 | `crates/manifold-bim-csg/src/ifc.rs:67` | - |  |
| `impl IfcWriter::finish` | 2 | `crates/manifold-bim-csg/src/ifc.rs:74` | - | Finishes the document, wrapping it in standard ISO-10303-21 HEADER and DATA sections. |
| `export_minimal_building` | 8 | `crates/manifold-bim-csg/src/ifc.rs:94` | - | Architectural project exporter creating standard hierarchy: Project -> Site -> Building -> Storey -> Wall. |
| `ifc_export_and_guid` | 0 | `crates/manifold-bim-csg/src/ifc.rs:218` | - |  |
| `dist_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:16` | - |  |
| `sub_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:21` | - |  |
| `add_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:26` | - |  |
| `mul_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:31` | - |  |
| `unit_2d` | 1 | `crates/manifold-bim-csg/src/parametric.rs:36` | - |  |
| `perp_2d` | 1 | `crates/manifold-bim-csg/src/parametric.rs:46` | - |  |
| `impl WallAxis::straight` | 2 | `crates/manifold-bim-csg/src/parametric.rs:62` | - |  |
| `impl WallAxis::new_arc` | 3 | `crates/manifold-bim-csg/src/parametric.rs:70` | - |  |
| `impl WallAxis::length` | 1 | `crates/manifold-bim-csg/src/parametric.rs:90` | - |  |
| `impl WallAxis::tangent` | 2 | `crates/manifold-bim-csg/src/parametric.rs:97` | - |  |
| `impl WallAxis::at` | 3 | `crates/manifold-bim-csg/src/parametric.rs:107` | - |  |
| `mitre_corner` | 4 | `crates/manifold-bim-csg/src/parametric.rs:130` | computational geometry | Calculates mitred corner intersection between two straight walls meeting at an angle. |
| `impl StairRun::end` | 1 | `crates/manifold-bim-csg/src/parametric.rs:172` | - |  |
| `impl StairRun::outline` | 2 | `crates/manifold-bim-csg/src/parametric.rs:176` | - |  |
| `generate_straight_stair` | 7 | `crates/manifold-bim-csg/src/parametric.rs:205` | - | Generates a straight stair flight with uniform risers and treads. |
| `impl MepCategory::may_stand` | 1 | `crates/manifold-bim-csg/src/parametric.rs:260` | - |  |
| `impl MepCategory::ifc_class_and_type` | 1 | `crates/manifold-bim-csg/src/parametric.rs:264` | - |  |
| `impl MepGeom::new_pipe` | 3 | `crates/manifold-bim-csg/src/parametric.rs:290` | - |  |
| `impl MepGeom::new_duct` | 4 | `crates/manifold-bim-csg/src/parametric.rs:300` | - |  |
| `impl MepGeom::length` | 1 | `crates/manifold-bim-csg/src/parametric.rs:310` | - |  |
| `impl MepGeom::is_vertical` | 1 | `crates/manifold-bim-csg/src/parametric.rs:317` | - |  |
| `wall_axis_straight_and_arc` | 0 | `crates/manifold-bim-csg/src/parametric.rs:329` | - |  |
| `mitre_corner_calculation` | 0 | `crates/manifold-bim-csg/src/parametric.rs:341` | - |  |
| `stair_flight_geometry` | 0 | `crates/manifold-bim-csg/src/parametric.rs:349` | - |  |
| `mep_run_classification` | 0 | `crates/manifold-bim-csg/src/parametric.rs:359` | - |  |
| `parse` | 1 | `crates/manifold-bim-csg/src/step.rs:39` | - |  |
| `find` | 2 | `crates/manifold-bim-csg/src/step.rs:104` | - |  |
| `first_string` | 1 | `crates/manifold-bim-csg/src/step.rs:108` | - |  |
| `impl Parser<'_>::peek` | 1 | `crates/manifold-bim-csg/src/step.rs:122` | - |  |
| `impl Parser<'_>::rest` | 1 | `crates/manifold-bim-csg/src/step.rs:126` | - |  |
| `impl Parser<'_>::skip` | 1 | `crates/manifold-bim-csg/src/step.rs:130` | - |  |
| `impl Parser<'_>::error` | 2 | `crates/manifold-bim-csg/src/step.rs:142` | - |  |
| `impl Parser<'_>::expect` | 2 | `crates/manifold-bim-csg/src/step.rs:154` | - |  |
| `impl Parser<'_>::word` | 2 | `crates/manifold-bim-csg/src/step.rs:163` | - |  |
| `impl Parser<'_>::keyword` | 1 | `crates/manifold-bim-csg/src/step.rs:176` | - |  |
| `impl Parser<'_>::uint` | 1 | `crates/manifold-bim-csg/src/step.rs:190` | - |  |
| `impl Parser<'_>::number` | 1 | `crates/manifold-bim-csg/src/step.rs:202` | - |  |
| `impl Parser<'_>::string` | 1 | `crates/manifold-bim-csg/src/step.rs:223` | - |  |
| `impl Parser<'_>::until` | 2 | `crates/manifold-bim-csg/src/step.rs:245` | - |  |
| `impl Parser<'_>::value` | 2 | `crates/manifold-bim-csg/src/step.rs:256` | - |  |
| `impl Parser<'_>::complex` | 1 | `crates/manifold-bim-csg/src/step.rs:293` | - |  |
| `impl Parser<'_>::list` | 2 | `crates/manifold-bim-csg/src/step.rs:306` | - |  |
| `decode` | 1 | `crates/manifold-bim-csg/src/step.rs:333` | - |  |
| `parse_step_file` | 0 | `crates/manifold-bim-csg/src/step.rs:401` | - |  |
| `step_decode_escapes` | 0 | `crates/manifold-bim-csg/src/step.rs:418` | - |  |
