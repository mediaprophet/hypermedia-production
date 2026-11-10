# `crates/layout-nodegraph-compiler`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure-Rust typed dataflow node graph compiler._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `fnv1a_hash_str` | 2 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:9` | - |  |
| `fnv1a_hash_tagged_value` | 2 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:17` | - |  |
| `compute_node_hashes` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:52` | graph | Compute deterministic hash trees for all nodes in topological order. |
| `cse_deduplicate` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:107` | graph | Deduplicate identical subexpressions: |
| `prune_dead_nodes` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:153` | - | Dead-code elimination: |
| `flatten_network` | 1 | `crates/layout-nodegraph-compiler/src/compiler/flatten.rs:9` | - |  |
| `impl NodeId::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:12` | - |  |
| `impl NodeNetwork::new` | 0 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:69` | - |  |
| `impl NodeNetwork::add_node` | 3 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:73` | graph |  |
| `impl NodeNetwork::output_node_id` | 1 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:77` | graph |  |
| `impl CompilerOptions::all_optimizations` | 0 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:20` | - |  |
| `impl Default for NodeGraphCompiler::default` | 0 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:34` | - |  |
| `impl NodeGraphCompiler::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:42` | - |  |
| `impl NodeGraphCompiler::compile` | 2 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:46` | - |  |
| `topological_sort` | 1 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:110` | graph |  |
| `dfs` | 5 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:124` | graph |  |
| `impl ProtoNetwork::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:29` | - |  |
| `impl ProtoNetwork::add_node` | 3 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:37` | graph |  |
| `impl ProtoNetwork::node` | 2 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:41` | graph |  |
| `impl ProtoNetwork::node_mut` | 2 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:45` | graph |  |
| `impl Unit::parse` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/ast.rs:23` | - |  |
| `impl Unit::to_base_scale` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/ast.rs:44` | - | Converts standard unit to its base canonical dimension: |
| `impl MathContext::new` | 0 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:15` | - |  |
| `impl MathContext::set_var` | 3 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:24` | - |  |
| `impl MathContext::eval` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:28` | - |  |
| `impl MathContext::eval_node` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:33` | graph |  |
| `tokenize` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/lexer.rs:20` | - |  |
| `impl Parser<'a>::new` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:14` | - |  |
| `impl Parser<'a>::peek` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:18` | - |  |
| `impl Parser<'a>::next` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:22` | - |  |
| `impl Parser<'a>::parse_expression` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:30` | - |  |
| `impl Parser<'a>::parse_precedence` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:34` | - |  |
| `impl Parser<'a>::peek_binary_op` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:62` | - |  |
| `impl Parser<'a>::parse_primary` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:74` | - |  |
| `op_precedence` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:131` | - |  |
| `parse` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:139` | - |  |
| `test_nodegraph_pipeline_basic` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:17` | - |  |
| `test_composite_subnetwork_inlining` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:74` | - |  |
| `test_common_subexpression_elimination` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:134` | - |  |
| `test_dead_node_pruning` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:185` | graph |  |
| `test_math_parser_algebraic_and_units` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:220` | - |  |
