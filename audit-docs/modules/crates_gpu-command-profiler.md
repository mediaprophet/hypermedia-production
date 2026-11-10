# `crates/gpu-command-profiler`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_!

Easy to use profiler scopes for [wgpu](https://github.com/gfx-rs/wgpu) using timer queries.

`wgpu_profiler` manages all the necessary [`wgpu::QuerySet`] and [`wgpu::Buffer`] behind the scenes
and allows you to create to create timer scopes with minimal overhead!

# How to use

```
use wgpu_profiler::*;

# async fn wgpu_init() -> (wgpu::Instance, wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    # let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    # let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    # let (device, queue) = adapter
    #     .request_device(&wgpu::DeviceDescriptor {
    #         required_features: wgpu::Features::TIMESTAMP_QUERY,
    #         ..Default::default()
    #     })
    #     .await
    #     .unwrap();
    # (instance, adapter, device, queue)
# }
# let (instance, adapter, device, queue) = futures_lite::future::block_on(wgpu_init());
# let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
#     label: None,
#     source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../examples/compute_shader.wgsl"))),
# });
# let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
#        label: None,
#        layout: None,
#        module: &cs_module,
#        entry_point: Some("main"),
#        compilation_options: wgpu::PipelineCompilationOptions::default(),
#        cache: None,
#    });
// ...

let mut profiler = GpuProfiler::new(&device, GpuProfilerSettings::default()).unwrap();

// ...

# let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
{
    // You can now open profiling scopes on any encoder or pass:
    let mut scope = profiler.scope("name of your scope", &mut encoder);

    // Scopes can be nested arbitrarily!
    let mut nested_scope = scope.scope("nested!");

    // Scopes on encoders can be used to easily create profiled passes!
    let mut compute_pass = nested_scope.scoped_compute_pass("profiled compute");


    // Scopes expose the underlying encoder or pass they wrap:
    compute_pass.set_pipeline(&pipeline);
    // ...

    // Scopes created this way are automatically closed when dropped.
}

// Wgpu-profiler needs to insert buffer copy commands.
profiler.resolve_queries(&mut encoder);
# drop(encoder);

// ...

// And finally, to end a profiling frame, call `end_frame`.
// This does a few checks and will let you know if something is off!
profiler.end_frame().unwrap();

// Retrieving the oldest available frame and writing it out to a chrome trace file.
if let Some(profiling_data) = profiler.process_finished_frame(queue.get_timestamp_period()) {
    # let button_pressed = false;
    // You usually want to write to disk only under some condition, e.g. press of a key.
    if button_pressed {
        wgpu_profiler::chrometrace::write_chrometrace(
            std::path::Path::new("mytrace.json"), &profiling_data);
    }
}
```
Check also the [Example](https://github.com/Wumpf/wgpu-profiler/blob/main/examples/demo.rs) where everything can be seen in action.

## Tracy integration

If you want to use [tracy](https://github.com/wolfpld/tracy) for profiling, you can enable the `tracy` feature.

This adds `wgpu_profiler::new_with_tracy_client` which will automatically report profiling data to tracy.

For details check the example code.

## Puffin integration

If you want to use [puffin](https://github.com/EmbarkStudios/puffin) for profiling, you can enable the `puffin` feature.

This adds `wgpu_profiler::puffin::output_frame_to_puffin` which makes it easy to report profiling data to a `puffin::GlobalProfiler`.

You can run the demo example with puffin by running `cargo run --example demo --features puffin`.
All CPU profiling goes to port `8585`, all GPU profiling goes to port `8586`. You can open puffin viewers for both.
```sh
puffin_viewer --url 127.0.0.1:8585
puffin_viewer --url 127.0.0.1:8586
```

For details check the example code.

# Internals

For every frame that hasn't completely finished processing yet
(i.e. hasn't returned results via [`GpuProfiler::process_finished_frame`])
we keep a `PendingFrame` around.

Whenever a profiling scope is opened, we allocate two queries.
This is done by either using the most recent `QueryPool` or creating a new one if there's no non-exhausted one ready.
Ideally, we only ever need a single `QueryPool` per frame! In order to converge to this,
we allocate new query pools with the size of all previous query pools in a given frame, effectively doubling the size.
On [`GpuProfiler::end_frame`], we memorize the total size of all `QueryPool`s in the current frame and make this the new minimum pool size.

`QueryPool` from finished frames are re-used, unless they are deemed too small._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `collatz_iterations` | 1 | `crates/gpu-command-profiler/examples/compute_shader.wgsl:11` | - |  |
| `main` | 1 | `crates/gpu-command-profiler/examples/compute_shader.wgsl:34` | 3d |  |
| `scopes_to_console_recursive` | 2 | `crates/gpu-command-profiler/examples/demo.rs:18` | - |  |
| `console_output` | 2 | `crates/gpu-command-profiler/examples/demo.rs:40` | - |  |
| `impl GfxState::new` | 1 | `crates/gpu-command-profiler/examples/demo.rs:76` | - |  |
| `impl ApplicationHandler<()> for State::resumed` | 2 | `crates/gpu-command-profiler/examples/demo.rs:181` | - |  |
| `impl ApplicationHandler<()> for State::window_event` | 4 | `crates/gpu-command-profiler/examples/demo.rs:200` | - |  |
| `impl ApplicationHandler<()> for State::about_to_wait` | 2 | `crates/gpu-command-profiler/examples/demo.rs:318` | - |  |
| `draw` | 4 | `crates/gpu-command-profiler/examples/demo.rs:326` | - |  |
| `main` | 0 | `crates/gpu-command-profiler/examples/demo.rs:422` | - |  |
| `vs_main` | 2 | `crates/gpu-command-profiler/examples/shader.wgsl:7` | 3d |  |
| `fs_main` | 1 | `crates/gpu-command-profiler/examples/shader.wgsl:27` | - |  |
| `write_chrometrace` | 2 | `crates/gpu-command-profiler/src/chrometrace.rs:6` | graph | Writes a .json trace file that can be viewed as a flame graph in Chrome or Edge via <chrome://tracing> |
| `write_results_recursive` | 3 | `crates/gpu-command-profiler/src/chrometrace.rs:28` | - |  |
| `impl PartialEq for CreationError::eq` | 2 | `crates/gpu-command-profiler/src/errors.rs:23` | - |  |
| `impl GpuProfiler::new` | 2 | `crates/gpu-command-profiler/src/profiler.rs:58` | - | Creates a new Profiler object. |
| `impl GpuProfiler::new_with_tracy_client` | 4 | `crates/gpu-command-profiler/src/profiler.rs:92` | - | Creates a new profiler and connects to a running Tracy client. |
| `impl GpuProfiler::settings` | 1 | `crates/gpu-command-profiler/src/profiler.rs:106` | - | Returns currently active settings. |
| `impl GpuProfiler::change_settings` | 2 | `crates/gpu-command-profiler/src/profiler.rs:117` | - | Changes the settings of an existing profiler. |
| `impl GpuProfiler::scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:143` | - | Starts a new auto-closing profiler scope. |
| `impl GpuProfiler::owning_scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:172` | - | Starts a new auto-closing profiler scope that takes ownership of the passed encoder or rendering/compute pass. |
| `impl GpuProfiler::manual_owning_scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:204` | - | Starts a new **manually closed** profiler scope that takes ownership of the passed encoder or rendering/compute pass. |
| `impl GpuProfiler::begin_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:232` | - | Starts a new profiler query on the given encoder or rendering/compute pass (if enabled). |
| `impl GpuProfiler::begin_pass_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:266` | physics | Starts a new profiler query to be used for render/compute pass timestamp writes. |
| `impl GpuProfiler::end_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:284` | - | Ends passed query. |
| `impl GpuProfiler::resolve_queries` | 2 | `crates/gpu-command-profiler/src/profiler.rs:345` | - | Puts query resolve commands in the encoder for all unresolved, pending queries of the active profiler frame. |
| `impl GpuProfiler::end_frame` | 1 | `crates/gpu-command-profiler/src/profiler.rs:394` | - | Marks the end of a frame. |
| `impl GpuProfiler::process_finished_frame` | 2 | `crates/gpu-command-profiler/src/profiler.rs:493` | - | Checks if all timer queries for the oldest pending finished frame are done and returns that snapshot if any. |
| `timestamp_query_support` | 3 | `crates/gpu-command-profiler/src/profiler.rs:542` | - | Returns true if a timestamp query is supported. |
| `impl GpuProfiler::next_scope_tree_handle` | 1 | `crates/gpu-command-profiler/src/profiler.rs:558` | - |  |
| `impl GpuProfiler::reset_and_cache_unused_query_pools` | 2 | `crates/gpu-command-profiler/src/profiler.rs:570` | - |  |
| `impl GpuProfiler::try_reserve_query_pair` | 1 | `crates/gpu-command-profiler/src/profiler.rs:591` | - |  |
| `impl GpuProfiler::reserve_query_pair` | 1 | `crates/gpu-command-profiler/src/profiler.rs:629` | - |  |
| `impl GpuProfiler::begin_query_internal` | 4 | `crates/gpu-command-profiler/src/profiler.rs:685` | - |  |
| `impl GpuProfiler::process_timings_recursive` | 3 | `crates/gpu-command-profiler/src/profiler.rs:740` | - |  |
| `impl QueryPool::new` | 2 | `crates/gpu-command-profiler/src/profiler.rs:860` | - |  |
| `impl QueryPool::reset` | 1 | `crates/gpu-command-profiler/src/profiler.rs:888` | - |  |
| `trait ProfilerCommandRecorder::is_pass` | 1 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:4` | - | Returns `true` if it's a pass or `false` if it's an encoder |
| `trait ProfilerCommandRecorder::write_timestamp` | 3 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:5` | - |  |
| `trait ProfilerCommandRecorder::push_debug_group` | 2 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:6` | - |  |
| `trait ProfilerCommandRecorder::pop_debug_group` | 1 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:7` | - |  |
| `impl GpuProfilerQuery::render_pass_timestamp_writes` | 1 | `crates/gpu-command-profiler/src/profiler_query.rs:74` | - | Use the reserved query for render pass timestamp writes if any. |
| `impl GpuProfilerQuery::compute_pass_timestamp_writes` | 1 | `crates/gpu-command-profiler/src/profiler_query.rs:92` | - | Use the reserved query for compute pass timestamp writes if any. |
| `impl GpuProfilerQuery::with_parent` | 2 | `crates/gpu-command-profiler/src/profiler_query.rs:106` | - | Makes this scope a child of the passed scope. |
| `impl Default for GpuProfilerSettings::default` | 0 | `crates/gpu-command-profiler/src/profiler_settings.rs:40` | - |  |
| `impl GpuProfilerSettings::validate` | 1 | `crates/gpu-command-profiler/src/profiler_settings.rs:50` | - |  |
| `output_frame_to_puffin` | 2 | `crates/gpu-command-profiler/src/puffin.rs:6` | - | Visualize the query results in a `puffin::GlobalProfiler`. |
| `collect_stream_info_recursive` | 4 | `crates/gpu-command-profiler/src/puffin.rs:19` | - |  |
| `impl Drop for Scope<'_, R>::drop` | 1 | `crates/gpu-command-profiler/src/scope.rs:17` | - |  |
| `impl Drop for OwningScope<'_, R>::drop` | 1 | `crates/gpu-command-profiler/src/scope.rs:35` | - |  |
| `impl ManualOwningScope<'_, R>::end_query` | 1 | `crates/gpu-command-profiler/src/scope.rs:58` | - | Ends the scope allowing the extraction of the owned [`ProfilerCommandRecorder`]. |
| `create_tracy_gpu_client` | 3 | `crates/gpu-command-profiler/src/tracy.rs:3` | - |  |
| `handle_dropped_frames_gracefully` | 0 | `crates/gpu-command-profiler/tests/src/dropped_frame_handling.rs:7` | - |  |
| `invalid_pending_frame_count` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:6` | - |  |
| `end_frame_unclosed_query` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:25` | - |  |
| `end_frame_unresolved_query` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:55` | - |  |
| `change_settings_while_query_open` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:85` | - |  |
| `interleaved_scopes` | 0 | `crates/gpu-command-profiler/tests/src/interleaved_command_buffer.rs:8` | - |  |
| `multithreaded_scopes` | 0 | `crates/gpu-command-profiler/tests/src/interleaved_command_buffer.rs:66` | - |  |
| `create_device` | 1 | `crates/gpu-command-profiler/tests/src/mod.rs:7` | - |  |
| `create_default_device_async` | 1 | `crates/gpu-command-profiler/tests/src/mod.rs:10` | - |  |
| `impl Requires::expect_time_result` | 2 | `crates/gpu-command-profiler/tests/src/mod.rs:39` | - |  |
| `expected_scope` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:56` | - |  |
| `validate_results` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:64` | - |  |
| `validate_results_unordered` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:87` | - |  |
| `multiple_resolves_per_frame` | 0 | `crates/gpu-command-profiler/tests/src/multiple_resolves_per_frame.rs:5` | - |  |
| `nested_scopes` | 2 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:7` | - |  |
| `nested_scopes_timestamp_in_passes_and_encoder_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:140` | - |  |
| `nested_scopes_timestamp_in_passes_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:153` | - |  |
| `nested_scopes_timestamp_in_encoders_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:164` | - |  |
| `nested_scopes_timestamp_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:175` | - |  |
| `nested_scopes_no_features` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:184` | - |  |
