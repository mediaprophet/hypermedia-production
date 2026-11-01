use super::*;

fn sample() -> StrokeSample {
    StrokeSample {
        stroke_id: 1, timestamp_ms: 100, center_x_m: 8.0, center_y_m: 8.0,
        radius_m: 4.0, strength: 2.0, target_height_m: 12.0, step_m: 2.0,
        op_code: 0, falloff_code: 0, layer_index: 0, flags: 0, seed: 42, pad: [0; 2],
    }
}

#[test]
fn smoothing_uses_one_input_snapshot_and_preserves_sequential_samples() {
    let mut grid = CowTerrainTileGrid::new_with_neighborhood_sampling(5, 1.0, 100_000, NeighborhoodSampling::SnapshotV1).unwrap();
    let mut heights = [0.0; 25];
    heights[12] = 16.0;
    grid.load_tile(0, 0, &heights).unwrap();
    grid.begin_stroke().unwrap();
    let mut s = sample();
    s.op_code = 3;
    s.falloff_code = 3; // plateau is one across this test's window
    s.center_x_m = 2.0;
    s.center_y_m = 2.0;
    s.radius_m = 100.0;
    s.strength = 1.0;
    grid.apply_stroke_sample(s).unwrap();
    let first = grid.tile_arc(0, 0).unwrap();
    assert_eq!(first[12], 0.0);
    assert_eq!((first[11], first[13], first[7], first[17]), (4.0, 4.0, 4.0, 4.0));
    for y in 0..5 {
        for x in 0..5 {
            assert_eq!(first[y * 5 + x], first[y * 5 + (4 - x)]);
            assert_eq!(first[y * 5 + x], first[(4 - y) * 5 + x]);
        }
    }
    grid.apply_stroke_sample(s).unwrap();
    assert_eq!(grid.tile_arc(0, 0).unwrap()[12], 4.0);
    grid.commit_stroke("snapshot").unwrap();
    let journal = grid.undo_history()[0].stroke_samples.clone();
    let expected = grid.tile_arc(0, 0).unwrap().as_slice().to_vec();
    let mode = grid.neighborhood_sampling();
    let mut replay = CowTerrainTileGrid::new_with_neighborhood_sampling(5, 1.0, 100_000, mode).unwrap();
    replay.load_tile(0, 0, &heights).unwrap();
    replay.replay_stroke_journal(&journal, "snapshot replay").unwrap();
    assert_eq!(replay.tile_arc(0, 0).unwrap().as_slice(), expected);
}

#[test]
fn default_neighborhood_is_legacy_for_existing_journal_callers() {
    let grid = CowTerrainTileGrid::new(5, 1.0, 100_000).unwrap();
    assert_eq!(grid.neighborhood_sampling(), NeighborhoodSampling::LegacySequential);
}

#[test]
fn hydraulic_drop_is_symmetric_within_one_sample() {
    let mut grid = CowTerrainTileGrid::new_with_neighborhood_sampling(5, 1.0, 100_000, NeighborhoodSampling::SnapshotV1).unwrap();
    let mut heights = [0.0; 25];
    heights[12] = 16.0;
    grid.load_tile(0, 0, &heights).unwrap();
    grid.begin_stroke().unwrap();
    let mut s = sample();
    s.op_code = 5;
    s.falloff_code = 3;
    s.center_x_m = 2.0;
    s.center_y_m = 2.0;
    s.radius_m = 100.0;
    s.strength = 1.0;
    grid.apply_stroke_sample(s).unwrap();
    let tile = grid.tile_arc(0, 0).unwrap();
    assert_eq!(tile[12], 8.0);
    assert_eq!((tile[11], tile[13], tile[7], tile[17]), (2.0, 2.0, 2.0, 2.0));
}

fn grid(budget: usize) -> CowTerrainTileGrid {
    let mut grid = CowTerrainTileGrid::new(16, 1.0, budget).unwrap();
    grid.ensure_tile(0, 0, 5.0).unwrap();
    grid
}

#[test]
fn dimensions_and_initial_height_refuse_before_allocation_or_mutation() {
    assert!(CowTerrainTileGrid::new(usize::MAX, 1.0, usize::MAX).is_err());
    assert!(CowTerrainTileGrid::new(16, f32::MAX, usize::MAX).is_err());
    let mut grid = grid(1024);
    assert!(grid.ensure_tile(1, 0, 5.0).is_err());
    assert!(grid.ensure_tile(0, 0, f32::NAN).is_err());
    assert_eq!(grid.tiles().len(), 1);
    grid.begin_stroke().unwrap();
    assert!(matches!(grid.ensure_tile(1, 0, 5.0), Err(GeoStudioError::StrokeAlreadyActive)));
    grid.cancel_stroke().unwrap();
}

#[test]
fn decoded_source_import_preserves_heights_and_refuses_history_replacement() {
    let mut grid = CowTerrainTileGrid::new(2, 1.0, 10_000).unwrap();
    let source = [1.0, 2.0, 3.0, 4.0];
    grid.load_tile(0, 0, &source).unwrap();
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert_eq!(before.as_slice(), source);
    assert!(grid.load_tile(0, 0, &[1.0]).is_err());
    assert!(grid.load_tile(0, 0, &[1.0, 2.0, f32::NAN, 4.0]).is_err());
    assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
    let mut s = sample();
    s.center_x_m = 0.0;
    s.center_y_m = 0.0;
    grid.replay_stroke_journal(&[s], "source edit").unwrap();
    assert!(grid.load_tile(0, 0, &source).is_err());
}

#[test]
fn invalid_and_unsupported_samples_preserve_preview_and_journal() {
    let mut grid = grid(100_000);
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    grid.begin_stroke().unwrap();
    let mut invalid = Vec::new();
    for field in 0..10 {
        let mut s = sample();
        match field {
            0 => s.center_x_m = f32::NAN,
            1 => s.radius_m = f32::MAX,
            2 => s.strength = -1.0,
            3 => s.target_height_m = f32::INFINITY,
            4 => s.op_code = 6,
            5 => s.falloff_code = 4,
            6 => s.layer_index = 1,
            7 => s.flags = 1,
            8 => s.pad = [1; 2],
            _ => { s.op_code = 4; s.step_m = 0.0; }
        }
        invalid.push(s);
    }
    for s in invalid {
        assert!(grid.apply_stroke_sample(s).is_err());
        assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
        assert!(grid.active_samples().is_empty());
    }
    grid.cancel_stroke().unwrap();
}

#[test]
fn cow_and_numeric_budget_refusals_are_atomic() {
    let mut low = grid(2 * 1024 + 63);
    let before = Arc::clone(low.tile_arc(0, 0).unwrap());
    low.begin_stroke().unwrap();
    assert!(matches!(low.apply_stroke_sample(sample()), Err(GeoStudioError::ByteBudgetExceeded { .. })));
    assert!(Arc::ptr_eq(&before, low.tile_arc(0, 0).unwrap()));
    assert!(low.active_samples().is_empty());
    low.cancel_stroke().unwrap();
    let mut overflow = CowTerrainTileGrid::new(16, 1.0, 100_000).unwrap();
    overflow.ensure_tile(0, 0, f32::MAX).unwrap();
    let before = Arc::clone(overflow.tile_arc(0, 0).unwrap());
    overflow.begin_stroke().unwrap();
    let mut s = sample();
    s.strength = f32::MAX;
    assert!(overflow.apply_stroke_sample(s).is_err());
    assert!(Arc::ptr_eq(&before, overflow.tile_arc(0, 0).unwrap()));
    assert!(overflow.active_samples().is_empty());
}

#[test]
fn cancel_and_failed_commit_preserve_redo_and_exact_tile_baseline() {
    let mut grid = grid(100_000);
    grid.replay_stroke_journal(&[sample()], "first").unwrap();
    grid.undo().unwrap();
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert_eq!(grid.redo_history().len(), 1);
    grid.begin_stroke().unwrap();
    grid.apply_stroke_sample(sample()).unwrap();
    let preview = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert!(grid.commit_stroke(&"x".repeat(MAX_LABEL_BYTES + 1)).is_err());
    assert!(grid.is_stroke_active());
    assert!(Arc::ptr_eq(&preview, grid.tile_arc(0, 0).unwrap()));
    assert_eq!(grid.redo_history()[0].commit_id, 1);
    assert!(grid.undo_history().is_empty());
    grid.cancel_stroke().unwrap();
    assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
    assert_eq!(grid.redo().unwrap(), Some(1));
    assert_eq!(grid.replay_stroke_journal(&[sample()], "second").unwrap(), 2);
}

#[test]
fn replay_failure_cancels_all_prior_samples_and_does_not_advance_id() {
    let mut grid = grid(100_000);
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    let mut bad = sample();
    bad.op_code = 99;
    assert!(grid.replay_stroke_journal(&[sample(), bad], "refused").is_err());
    assert!(!grid.is_stroke_active());
    assert!(grid.undo_history().is_empty());
    assert!(grid.redo_history().is_empty());
    assert!(grid.active_samples().is_empty());
    assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
    assert_eq!(grid.replay_stroke_journal(&[sample()], "accepted").unwrap(), 1);
}

#[test]
fn commit_identifier_exhaustion_does_not_destroy_active_preview() {
    let mut grid = grid(100_000);
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    grid.next_commit_id = u64::MAX;
    grid.begin_stroke().unwrap();
    grid.apply_stroke_sample(sample()).unwrap();
    assert!(grid.commit_stroke("overflow").is_err());
    assert!(grid.is_stroke_active());
    assert!(grid.undo_history().is_empty());
    grid.cancel_stroke().unwrap();
    assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
}

#[test]
fn late_second_tile_failure_leaves_all_tiles_journal_and_prior_history_unchanged() {
    let mut grid = grid(1_000_000);
    grid.ensure_tile(1, 0, f32::MAX).unwrap();
    grid.replay_stroke_journal(&[sample()], "prior accepted edit").unwrap();
    grid.undo().unwrap();
    let first = Arc::clone(grid.tile_arc(0, 0).unwrap());
    let second = Arc::clone(grid.tile_arc(1, 0).unwrap());
    let first_height = grid.sample_height(0, 0, 15, 8);
    let mut s = sample();
    s.center_x_m = 16.0;
    s.radius_m = 6.0;
    // First tile can represent this increment; f32::MAX + effective > 0 on second overflows.
    s.strength = f32::MAX;
    grid.begin_stroke().unwrap();
    assert!(grid.apply_stroke_sample(s).is_err());
    assert!(Arc::ptr_eq(&first, grid.tile_arc(0, 0).unwrap()));
    assert!(Arc::ptr_eq(&second, grid.tile_arc(1, 0).unwrap()));
    assert_eq!(grid.sample_height(0, 0, 15, 8), first_height);
    assert_eq!(grid.sample_height(1, 0, 0, 8), Some(f32::MAX));
    assert!(grid.active_samples().is_empty());
    assert!(grid.active_baseline.is_empty());
    assert!(grid.undo_history().is_empty());
    assert_eq!(grid.redo_history().len(), 1);
    assert_eq!(grid.redo_history()[0].commit_id, 1);
    grid.cancel_stroke().unwrap();
}

#[test]
fn full_history_accounting_is_bounded_and_explicit_discard_restores_commit_capacity() {
    let mut grid = grid(4_000_000);
    for _ in 0..MAX_HISTORY_ENTRIES {
        grid.replay_stroke_journal(&[sample()], "bounded history").unwrap();
    }
    // This formula traverses no individual patches/samples and allocates no scratch.
    let expected = (1 + 2 * MAX_HISTORY_ENTRIES) * 1024 + MAX_HISTORY_ENTRIES * 64;
    assert_eq!(grid.payload_memory_bytes(), expected);
    let current = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert!(grid.replay_stroke_journal(&[sample()], "cap refusal").is_err());
    assert!(!grid.is_stroke_active());
    assert!(Arc::ptr_eq(&current, grid.tile_arc(0, 0).unwrap()));
    grid.begin_stroke().unwrap();
    assert!(matches!(grid.discard_local_history(), Err(GeoStudioError::StrokeAlreadyActive)));
    grid.cancel_stroke().unwrap();
    assert_eq!(grid.discard_local_history().unwrap(), MAX_HISTORY_ENTRIES);
    assert!(Arc::ptr_eq(&current, grid.tile_arc(0, 0).unwrap()));
    assert!(grid.undo_history().is_empty() && grid.redo_history().is_empty());
    assert_eq!(grid.payload_memory_bytes(), 1024);
    assert_eq!(grid.replay_stroke_journal(&[sample()], "after explicit discard").unwrap(), MAX_HISTORY_ENTRIES as u64 + 1);
}

#[test]
fn explicit_history_discard_drops_redo_without_changing_accepted_heights_or_ids() {
    let mut grid = grid(100_000);
    grid.replay_stroke_journal(&[sample()], "first").unwrap();
    grid.replay_stroke_journal(&[sample()], "second").unwrap();
    grid.undo().unwrap();
    let current = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert_eq!(grid.discard_local_history().unwrap(), 2);
    assert!(Arc::ptr_eq(&current, grid.tile_arc(0, 0).unwrap()));
    assert_eq!(grid.undo().unwrap(), None);
    assert_eq!(grid.redo().unwrap(), None);
    assert_eq!(grid.replay_stroke_journal(&[sample()], "third").unwrap(), 3);
}

#[test]
fn conservative_history_reference_budget_refuses_commit_without_destroying_preview() {
    let mut grid = grid(2 * 1024 + 64);
    let before = Arc::clone(grid.tile_arc(0, 0).unwrap());
    grid.begin_stroke().unwrap();
    grid.apply_stroke_sample(sample()).unwrap();
    let preview = Arc::clone(grid.tile_arc(0, 0).unwrap());
    assert!(matches!(grid.commit_stroke("history would exceed conservative budget"), Err(GeoStudioError::ByteBudgetExceeded { .. })));
    assert!(grid.is_stroke_active());
    assert!(Arc::ptr_eq(&preview, grid.tile_arc(0, 0).unwrap()));
    assert!(grid.undo_history().is_empty());
    grid.cancel_stroke().unwrap();
    assert!(Arc::ptr_eq(&before, grid.tile_arc(0, 0).unwrap()));
}

#[test]
fn admitted_operations_match_original_same_platform_float_behavior() {
    let mut adapted = grid(4_000_000);
    let mut original = crate::source_oracle::CowTerrainTileGrid::new(16, 1.0, 4_000_000).unwrap();
    original.ensure_tile(0, 0, 5.0).unwrap();
    for op in 0..=5 {
        for falloff in 0..=3 {
            let mut s = sample();
            s.op_code = op;
            s.falloff_code = falloff;
            s.strength = 0.5;
            let old = crate::source_oracle::StrokeSample {
                stroke_id:s.stroke_id,timestamp_ms:s.timestamp_ms,center_x_m:s.center_x_m,center_y_m:s.center_y_m,
                radius_m:s.radius_m,strength:s.strength,target_height_m:s.target_height_m,step_m:s.step_m,
                op_code:s.op_code,falloff_code:s.falloff_code,layer_index:s.layer_index,flags:s.flags,seed:s.seed,pad:s.pad,
            };
            adapted.replay_stroke_journal(&[s], "adapted").unwrap();
            original.replay_stroke_journal(&[old], "original").unwrap();
            assert_eq!(adapted.tile_arc(0, 0).unwrap().as_slice(), original.tile_arc(0, 0).unwrap().as_slice());
        }
    }
}
