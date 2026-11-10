# `crates/kinetics-block-simulation`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Standard simulation blocks library._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Param::as_num` | 1 | `crates/kinetics-block-simulation/src/block.rs:23` | - |  |
| `impl Param::as_vec` | 1 | `crates/kinetics-block-simulation/src/block.rs:30` | - |  |
| `impl Param::as_str` | 1 | `crates/kinetics-block-simulation/src/block.rs:38` | - |  |
| `impl Param::as_bool` | 1 | `crates/kinetics-block-simulation/src/block.rs:45` | - |  |
| `trait Block::inputs` | 1 | `crates/kinetics-block-simulation/src/block.rs:56` | - |  |
| `trait Block::n_outputs` | 1 | `crates/kinetics-block-simulation/src/block.rs:57` | - |  |
| `trait Block::resolve_out` | 3 | `crates/kinetics-block-simulation/src/block.rs:58` | - |  |
| `trait Block::outputs` | 5 | `crates/kinetics-block-simulation/src/block.rs:59` | - |  |
| `trait Block::derivatives` | 5 | `crates/kinetics-block-simulation/src/block.rs:61` | - |  |
| `trait Block::update` | 3 | `crates/kinetics-block-simulation/src/block.rs:65` | - |  |
| `trait Block::log` | 3 | `crates/kinetics-block-simulation/src/block.rs:69` | - |  |
| `trait Block::sample_time` | 1 | `crates/kinetics-block-simulation/src/block.rs:73` | - |  |
| `trait Block::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/block.rs:77` | - |  |
| `trait Block::n_states` | 1 | `crates/kinetics-block-simulation/src/block.rs:81` | - |  |
| `trait Block::get_x` | 2 | `crates/kinetics-block-simulation/src/block.rs:85` | - |  |
| `trait Block::set_x` | 2 | `crates/kinetics-block-simulation/src/block.rs:89` | - |  |
| `trait Block::as_any` | 1 | `crates/kinetics-block-simulation/src/block.rs:93` | - |  |
| `trait Block::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/block.rs:94` | - |  |
| `trait BlockFactory::make` | 4 | `crates/kinetics-block-simulation/src/block.rs:98` | - |  |
| `trait BlockFactory::known_types` | 1 | `crates/kinetics-block-simulation/src/block.rs:99` | - |  |
| `impl Integrator::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:15` | - |  |
| `impl Block for Integrator::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:26` | - |  |
| `impl Block for Integrator::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:29` | - |  |
| `impl Block for Integrator::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:32` | - |  |
| `impl Block for Integrator::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:39` | - |  |
| `impl Block for Integrator::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:45` | - |  |
| `impl Block for Integrator::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:52` | - |  |
| `impl Block for Integrator::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:55` | - |  |
| `impl Block for Integrator::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:59` | - |  |
| `impl Block for Integrator::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:63` | - |  |
| `impl Block for Integrator::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:66` | - |  |
| `impl Block for Integrator::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:69` | - |  |
| `impl Derivative::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:81` | - |  |
| `impl Block for Derivative::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:92` | - |  |
| `impl Block for Derivative::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:95` | - |  |
| `impl Block for Derivative::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:98` | - |  |
| `impl Block for Derivative::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:102` | - |  |
| `impl Block for Derivative::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:119` | - |  |
| `impl Block for Derivative::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:122` | - |  |
| `impl Block for Derivative::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:125` | - |  |
| `impl PidController::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:141` | - |  |
| `impl Block for PidController::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:156` | - |  |
| `impl Block for PidController::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:159` | - |  |
| `impl Block for PidController::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:162` | - |  |
| `impl Block for PidController::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:165` | - |  |
| `impl Block for PidController::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:180` | - |  |
| `impl Block for PidController::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:187` | - |  |
| `impl Block for PidController::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:190` | - |  |
| `impl Block for PidController::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:194` | - |  |
| `impl Block for PidController::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:198` | - |  |
| `impl Block for PidController::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:201` | - |  |
| `impl Block for PidController::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:204` | - |  |
| `impl TransferFcn::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:217` | - |  |
| `impl Block for TransferFcn::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:232` | - |  |
| `impl Block for TransferFcn::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:235` | - |  |
| `impl Block for TransferFcn::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:238` | - |  |
| `impl Block for TransferFcn::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:241` | - |  |
| `impl Block for TransferFcn::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:262` | - |  |
| `impl Block for TransferFcn::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:282` | - |  |
| `impl Block for TransferFcn::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:285` | - |  |
| `impl Block for TransferFcn::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:289` | - |  |
| `impl Block for TransferFcn::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:293` | - |  |
| `impl Block for TransferFcn::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:296` | - |  |
| `impl Block for TransferFcn::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:299` | - |  |
| `impl TransportDelay::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:311` | - |  |
| `impl Block for TransportDelay::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:323` | - |  |
| `impl Block for TransportDelay::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:326` | - |  |
| `impl Block for TransportDelay::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:329` | - |  |
| `impl Block for TransportDelay::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:333` | - |  |
| `impl Block for TransportDelay::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:356` | - |  |
| `impl Block for TransportDelay::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:359` | - |  |
| `impl Block for TransportDelay::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:362` | - |  |
| `impl UnitDelay::new` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:16` | - |  |
| `impl Block for UnitDelay::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:29` | - |  |
| `impl Block for UnitDelay::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:32` | - |  |
| `impl Block for UnitDelay::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:35` | - |  |
| `impl Block for UnitDelay::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:42` | - |  |
| `impl Block for UnitDelay::update` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:48` | - |  |
| `impl Block for UnitDelay::sample_time` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:53` | - |  |
| `impl Block for UnitDelay::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:56` | - |  |
| `impl Block for UnitDelay::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:59` | - |  |
| `impl Block for UnitDelay::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:62` | - |  |
| `impl ZeroOrderHold::new` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:74` | - |  |
| `impl Block for ZeroOrderHold::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:86` | - |  |
| `impl Block for ZeroOrderHold::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:89` | - |  |
| `impl Block for ZeroOrderHold::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:92` | - |  |
| `impl Block for ZeroOrderHold::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:97` | - |  |
| `impl Block for ZeroOrderHold::update` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:103` | - |  |
| `impl Block for ZeroOrderHold::sample_time` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:108` | - |  |
| `impl Block for ZeroOrderHold::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:111` | - |  |
| `impl Block for ZeroOrderHold::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:114` | - |  |
| `impl Block for ZeroOrderHold::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:117` | - |  |
| `impl BlockFactory for StandardBlockFactory::make` | 4 | `crates/kinetics-block-simulation/src/blocks/factory.rs:48` | - |  |
| `impl BlockFactory for StandardBlockFactory::known_types` | 1 | `crates/kinetics-block-simulation/src/blocks/factory.rs:83` | - |  |
| `impl Gain::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:15` | - |  |
| `impl Block for Gain::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:26` | - |  |
| `impl Block for Gain::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:29` | - |  |
| `impl Block for Gain::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:32` | - |  |
| `impl Block for Gain::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:36` | - |  |
| `impl Block for Gain::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:46` | - |  |
| `impl Block for Gain::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:49` | - |  |
| `impl Sum::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:60` | - |  |
| `impl Block for Sum::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:80` | - |  |
| `impl Block for Sum::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:83` | - |  |
| `impl Block for Sum::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:86` | - |  |
| `impl Block for Sum::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:90` | - |  |
| `impl Block for Sum::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:108` | - |  |
| `impl Block for Sum::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:111` | - |  |
| `impl Product::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:122` | - |  |
| `impl Block for Product::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:142` | - |  |
| `impl Block for Product::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:145` | - |  |
| `impl Block for Product::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:148` | - |  |
| `impl Block for Product::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:152` | - |  |
| `impl Block for Product::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:170` | - |  |
| `impl Block for Product::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:173` | - |  |
| `impl Saturation::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:185` | - |  |
| `impl Block for Saturation::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:196` | - |  |
| `impl Block for Saturation::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:199` | - |  |
| `impl Block for Saturation::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:202` | - |  |
| `impl Block for Saturation::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:206` | - |  |
| `impl Block for Saturation::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:216` | - |  |
| `impl Block for Saturation::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:219` | - |  |
| `impl DeadZone::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:231` | - |  |
| `impl Block for DeadZone::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:242` | - |  |
| `impl Block for DeadZone::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:245` | - |  |
| `impl Block for DeadZone::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:248` | - |  |
| `impl Block for DeadZone::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:252` | - |  |
| `impl Block for DeadZone::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:268` | - |  |
| `impl Block for DeadZone::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:271` | - |  |
| `impl Switch::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:282` | - |  |
| `impl Block for Switch::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:292` | - |  |
| `impl Block for Switch::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:295` | - |  |
| `impl Block for Switch::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:298` | - |  |
| `impl Block for Switch::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:302` | - |  |
| `impl Block for Switch::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:318` | - |  |
| `impl Block for Switch::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:321` | - |  |
| `impl Relational::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:332` | - |  |
| `impl Block for Relational::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:342` | - |  |
| `impl Block for Relational::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:345` | - |  |
| `impl Block for Relational::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:348` | - |  |
| `impl Block for Relational::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:352` | - |  |
| `impl Block for Relational::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:372` | - |  |
| `impl Block for Relational::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:375` | - |  |
| `impl LogicalOperator::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:386` | - |  |
| `impl Block for LogicalOperator::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:402` | - |  |
| `impl Block for LogicalOperator::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:405` | - |  |
| `impl Block for LogicalOperator::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:408` | - |  |
| `impl Block for LogicalOperator::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:412` | - |  |
| `impl Block for LogicalOperator::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:439` | - |  |
| `impl Block for LogicalOperator::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:442` | - |  |
| `impl Abs::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:452` | - |  |
| `impl Block for Abs::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:461` | - |  |
| `impl Block for Abs::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:464` | - |  |
| `impl Block for Abs::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:467` | - |  |
| `impl Block for Abs::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:471` | - |  |
| `impl Block for Abs::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:480` | - |  |
| `impl Block for Abs::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:483` | - |  |
| `check_keys` | 2 | `crates/kinetics-block-simulation/src/blocks/params.rs:6` | - |  |
| `num` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:15` | - |  |
| `vec` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:25` | - |  |
| `req_vec` | 2 | `crates/kinetics-block-simulation/src/blocks/params.rs:35` | - |  |
| `string` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:45` | - |  |
| `count` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:55` | - |  |
| `impl Mux::new` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:14` | - |  |
| `impl Block for Mux::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:24` | - |  |
| `impl Block for Mux::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:27` | - |  |
| `impl Block for Mux::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/routing.rs:30` | - |  |
| `impl Block for Mux::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/routing.rs:34` | - |  |
| `impl Block for Mux::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:44` | - |  |
| `impl Block for Mux::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:47` | - |  |
| `impl Demux::new` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:58` | - |  |
| `impl Block for Demux::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:69` | - |  |
| `impl Block for Demux::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:72` | - |  |
| `impl Block for Demux::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/routing.rs:75` | - |  |
| `impl Block for Demux::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/routing.rs:85` | - |  |
| `impl Block for Demux::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:94` | - |  |
| `impl Block for Demux::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:97` | - |  |
| `impl Trace::new` | 0 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:16` | - |  |
| `impl Trace::push` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:20` | - |  |
| `impl Scope::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:32` | - |  |
| `impl Block for Scope::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:42` | - |  |
| `impl Block for Scope::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:45` | - |  |
| `impl Block for Scope::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:48` | - |  |
| `impl Block for Scope::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:51` | - |  |
| `impl Block for Scope::log` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:54` | - |  |
| `impl Block for Scope::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:59` | - |  |
| `impl Block for Scope::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:62` | - |  |
| `impl Block for Scope::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:65` | - |  |
| `impl ToWorkspace::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:77` | - |  |
| `impl Block for ToWorkspace::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:89` | - |  |
| `impl Block for ToWorkspace::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:92` | - |  |
| `impl Block for ToWorkspace::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:95` | - |  |
| `impl Block for ToWorkspace::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:98` | - |  |
| `impl Block for ToWorkspace::log` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:101` | - |  |
| `impl Block for ToWorkspace::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:106` | - |  |
| `impl Block for ToWorkspace::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:109` | - |  |
| `impl Block for ToWorkspace::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:112` | - |  |
| `impl Constant::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:15` | - |  |
| `impl Block for Constant::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:23` | - |  |
| `impl Block for Constant::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:26` | - |  |
| `impl Block for Constant::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:29` | - |  |
| `impl Block for Constant::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:32` | - |  |
| `impl Block for Constant::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:38` | - |  |
| `impl Block for Constant::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:41` | - |  |
| `impl Step::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:54` | - |  |
| `impl Block for Step::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:66` | - |  |
| `impl Block for Step::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:69` | - |  |
| `impl Block for Step::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:72` | - |  |
| `impl Block for Step::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:75` | - |  |
| `impl Block for Step::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:82` | - |  |
| `impl Block for Step::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:85` | - |  |
| `impl Sine::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:99` | - |  |
| `impl Block for Sine::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:112` | - |  |
| `impl Block for Sine::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:115` | - |  |
| `impl Block for Sine::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:118` | - |  |
| `impl Block for Sine::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:121` | - |  |
| `impl Block for Sine::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:128` | - |  |
| `impl Block for Sine::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:131` | - |  |
| `impl Ramp::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:144` | - |  |
| `impl Block for Ramp::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:156` | - |  |
| `impl Block for Ramp::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:159` | - |  |
| `impl Block for Ramp::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:162` | - |  |
| `impl Block for Ramp::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:165` | - |  |
| `impl Block for Ramp::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:176` | - |  |
| `impl Block for Ramp::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:179` | - |  |
| `impl Clock::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:189` | - |  |
| `impl Block for Clock::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:196` | - |  |
| `impl Block for Clock::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:199` | - |  |
| `impl Block for Clock::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:202` | - |  |
| `impl Block for Clock::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:205` | - |  |
| `impl Block for Clock::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:211` | - |  |
| `impl Block for Clock::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:214` | - |  |
| `split_endpoint` | 1 | `crates/kinetics-block-simulation/src/document.rs:26` | - |  |
| `impl Document::parse` | 1 | `crates/kinetics-block-simulation/src/document.rs:40` | - |  |
| `impl Document::to_model` | 2 | `crates/kinetics-block-simulation/src/document.rs:108` | - |  |
| `impl fmt::Display for SimError::fmt` | 2 | `crates/kinetics-block-simulation/src/lib.rs:42` | - |  |
| `test_integrator_simulation` | 0 | `crates/kinetics-block-simulation/src/lib.rs:66` | physics |  |
| `test_algebraic_loop_detection` | 0 | `crates/kinetics-block-simulation/src/lib.rs:101` | - |  |
| `test_json_document_parse_and_run` | 0 | `crates/kinetics-block-simulation/src/lib.rs:123` | - |  |
| `impl Default for Model::default` | 0 | `crates/kinetics-block-simulation/src/model.rs:24` | - |  |
| `impl Model::new` | 0 | `crates/kinetics-block-simulation/src/model.rs:30` | - |  |
| `impl Model::add_block` | 3 | `crates/kinetics-block-simulation/src/model.rs:39` | - |  |
| `impl Model::add_wire` | 5 | `crates/kinetics-block-simulation/src/model.rs:53` | - |  |
| `impl Model::index_of` | 2 | `crates/kinetics-block-simulation/src/model.rs:86` | - |  |
| `impl Model::block_names` | 1 | `crates/kinetics-block-simulation/src/model.rs:90` | - |  |
| `impl Model::block` | 2 | `crates/kinetics-block-simulation/src/model.rs:94` | - |  |
| `impl Model::block_mut` | 2 | `crates/kinetics-block-simulation/src/model.rs:99` | - |  |
| `impl Model::compile` | 1 | `crates/kinetics-block-simulation/src/model.rs:104` | - |  |
| `impl SigMeta::real` | 1 | `crates/kinetics-block-simulation/src/signal.rs:20` | - |  |
| `impl SigMeta::boolean` | 1 | `crates/kinetics-block-simulation/src/signal.rs:27` | - |  |
| `impl PortReq::fixed` | 1 | `crates/kinetics-block-simulation/src/signal.rs:71` | - |  |
| `impl PortReq::accepts` | 2 | `crates/kinetics-block-simulation/src/signal.rs:78` | - |  |
| `impl SigVal::scalar` | 1 | `crates/kinetics-block-simulation/src/signal.rs:100` | - |  |
| `impl SigVal::boolean` | 1 | `crates/kinetics-block-simulation/src/signal.rs:104` | - |  |
| `impl SigVal::width` | 1 | `crates/kinetics-block-simulation/src/signal.rs:108` | - |  |
| `impl SigVal::dtype` | 1 | `crates/kinetics-block-simulation/src/signal.rs:115` | - |  |
| `impl SigVal::meta` | 1 | `crates/kinetics-block-simulation/src/signal.rs:122` | - |  |
| `in_real` | 2 | `crates/kinetics-block-simulation/src/signal.rs:130` | - |  |
| `in_bool` | 2 | `crates/kinetics-block-simulation/src/signal.rs:138` | - |  |
| `impl Solver::parse` | 1 | `crates/kinetics-block-simulation/src/sim.rs:16` | - |  |
| `impl Default for SimConfig::default` | 0 | `crates/kinetics-block-simulation/src/sim.rs:38` | - |  |
| `impl Sim::new` | 2 | `crates/kinetics-block-simulation/src/sim.rs:70` | - |  |
| `impl Sim::eval_outputs` | 3 | `crates/kinetics-block-simulation/src/sim.rs:90` | - |  |
| `impl Sim::eval_derivatives` | 4 | `crates/kinetics-block-simulation/src/sim.rs:118` | - |  |
| `impl Sim::log_sinks` | 2 | `crates/kinetics-block-simulation/src/sim.rs:141` | - |  |
| `impl Sim::update_discrete` | 2 | `crates/kinetics-block-simulation/src/sim.rs:156` | - |  |
| `impl Sim::run` | 1 | `crates/kinetics-block-simulation/src/sim.rs:175` | - |  |
