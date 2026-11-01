//! Pure queue regressions, authored only; execution is deferred.
use super::*;
use qualia_core_db::audio::scene_audio_graph::{
    pcm_schedule::ClipScheduleReport, source_coupling::SoundBlockMode,
};
fn binding() -> SoundBinding {
    SoundBinding {
        model_sha256: [1; 32],
        source_sha256: [2; 32],
        graph_sha256: [3; 32],
        source_generation: 1,
        graph_generation: 1,
        transport_epoch: 1,
    }
}
fn receipt(first: u64) -> SoundBlockReceipt {
    SoundBlockReceipt {
        binding: binding(),
        pose_sequence: 1,
        pose_tick: 0,
        first_output_frame: first,
        first_source_frame: first,
        output_frames: 2,
        mode: SoundBlockMode::Playing,
        schedule: ClipScheduleReport {
            event_spans: 1,
            eligible_clip_frames: 2,
            selected_clip_frames: 2,
            virtualized_clip_frames: 0,
            max_selected_clips: 1,
            limited_input_samples: 0,
            limited_output_samples: 0,
            peak_before_limit: 1.,
            timeline_discontinuity: false,
            source_rebound: false,
            histories_reset: false,
        },
    }
}
fn queue() -> Queue {
    Queue::new(
        binding(),
        2,
        2,
        4096,
        Arc::new(AtomicBool::new(false)),
        Arc::new(AtomicU8::new(PREPARED)),
        Arc::new(AtomicU64::new(0)),
    )
    .unwrap()
}
#[test]
fn callback_crosses_slots_in_order_and_starvation_cannot_resume_old_audio() {
    let mut q = queue();
    q.enqueue(&receipt(0), &[0.25, 0.5], &[-0.25, -0.5])
        .unwrap();
    q.enqueue(&receipt(2), &[0.75, 1.], &[-0.75, -1.]).unwrap();
    q.status.store(PLAYING, Ordering::Release);
    let mut out = [0.; 8];
    q.render(&mut out);
    assert_eq!(out, [0.25, -0.25, 0.5, -0.5, 0.75, -0.75, 1., -1.]);
    assert_eq!(q.consumed.load(Ordering::Acquire), 4);
    q.render(&mut out);
    assert_eq!(out, [0.; 8]);
    assert!(q.cancel.load(Ordering::Acquire));
    assert_eq!(q.status.load(Ordering::Acquire), STARVED);
    assert!(q.enqueue(&receipt(4), &[0.25; 2], &[0.25; 2]).is_err());
}
#[test]
fn cancellation_silences_pending_samples_and_rejects_new_enqueue() {
    let mut q = queue();
    q.enqueue(&receipt(0), &[0.5; 2], &[0.5; 2]).unwrap();
    q.status.store(PLAYING, Ordering::Release);
    q.cancel.store(true, Ordering::Release);
    let mut out = [1.; 4];
    q.render(&mut out);
    assert_eq!(out, [0.; 4]);
    assert_eq!(q.consumed.load(Ordering::Acquire), 0);
    assert!(q.enqueue(&receipt(2), &[0.5; 2], &[0.5; 2]).is_err());
}
#[test]
fn stale_lease_and_wrong_cursor_do_not_enter_the_device_queue() {
    let mut q = queue();
    let mut stale = receipt(0);
    stale.binding.transport_epoch = 2;
    assert!(q.enqueue(&stale, &[0.5; 2], &[0.5; 2]).is_err());
    assert!(q.enqueue(&receipt(2), &[0.5; 2], &[0.5; 2]).is_err());
    assert_eq!(q.count, 0);
    assert_eq!(q.next_frame, None);
}
