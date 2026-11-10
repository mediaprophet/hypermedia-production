# `crates/pcm-device-output`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Native PCM device lifecycle; source, decoding, scheduling and authority retain
their existing QualiaDB/host owners. No application-shell dependency._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl DeviceSlot<'a>::acquire` | 2 | `crates/pcm-device-output/src/device.rs:28` | - |  |
| `impl DeviceSlot<'a>::before_play` | 1 | `crates/pcm-device-output/src/device.rs:36` | - |  |
| `impl Drop for DeviceSlot<'_>::drop` | 1 | `crates/pcm-device-output/src/device.rs:44` | - |  |
| `impl DeviceOutput::prepare` | 6 | `crates/pcm-device-output/src/device.rs:63` | - | `live` is called on the native device worker, NEVER inside the realtime |
| `impl DeviceOutput::enqueue` | 4 | `crates/pcm-device-output/src/device.rs:171` | - |  |
| `impl DeviceOutput::request` | 2 | `crates/pcm-device-output/src/device.rs:177` | - |  |
| `impl DeviceOutput::play` | 1 | `crates/pcm-device-output/src/device.rs:193` | - |  |
| `impl DeviceOutput::stop` | 1 | `crates/pcm-device-output/src/device.rs:196` | - |  |
| `impl DeviceOutput::binding` | 1 | `crates/pcm-device-output/src/device.rs:209` | - |  |
| `impl DeviceOutput::cancel` | 1 | `crates/pcm-device-output/src/device.rs:212` | - |  |
| `impl DeviceOutput::consumed_frames` | 1 | `crates/pcm-device-output/src/device.rs:215` | - |  |
| `impl DeviceOutput::status` | 1 | `crates/pcm-device-output/src/device.rs:218` | - |  |
| `impl Drop for DeviceOutput::drop` | 1 | `crates/pcm-device-output/src/device.rs:229` | - |  |
| `prepare_stream` | 3 | `crates/pcm-device-output/src/device.rs:234` | - |  |
| `stop_and_reset` | 1 | `crates/pcm-device-output/src/device.rs:282` | - |  |
| `stop_and_reset` | 1 | `crates/pcm-device-output/src/device.rs:290` | - |  |
| `never_started_preparation_failure_releases_the_actual_guard` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:5` | - |  |
| `failed_play_without_reset_ack_poison_blocks_reacquisition` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:16` | - |  |
| `acknowledged_reset_releases_but_a_later_play_invalidates_old_ack` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:28` | - |  |
| `panic_after_play_attempt_poison_is_retained` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:43` | - |  |
| `impl Queue::new` | 7 | `crates/pcm-device-output/src/queue.rs:31` | - |  |
| `impl Queue::enqueue` | 4 | `crates/pcm-device-output/src/queue.rs:80` | - |  |
| `impl Queue::ready` | 1 | `crates/pcm-device-output/src/queue.rs:122` | - |  |
| `impl Queue::render` | 2 | `crates/pcm-device-output/src/queue.rs:127` | - | All slot storage is retained: no allocation/deallocation or source lookup |
| `impl Queue::fail` | 2 | `crates/pcm-device-output/src/queue.rs:171` | - |  |
| `binding` | 0 | `crates/pcm-device-output/src/queue_tests.rs:6` | - |  |
| `receipt` | 1 | `crates/pcm-device-output/src/queue_tests.rs:16` | - |  |
| `queue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:40` | - |  |
| `callback_crosses_slots_in_order_and_starvation_cannot_resume_old_audio` | 0 | `crates/pcm-device-output/src/queue_tests.rs:53` | - |  |
| `cancellation_silences_pending_samples_and_rejects_new_enqueue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:70` | - |  |
| `stale_lease_and_wrong_cursor_do_not_enter_the_device_queue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:82` | - |  |
