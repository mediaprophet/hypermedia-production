//! Local domain-operation consumer; no native/HMC admission is implied.
use terrain_brushes::{CowTerrainTileGrid, StrokeSample};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = CowTerrainTileGrid::new(16, 1.0, 100_000)?;
    grid.ensure_tile(0, 0, 5.0)?;
    let sample = StrokeSample {
        stroke_id: 1, timestamp_ms: 0, center_x_m: 8.0, center_y_m: 8.0,
        radius_m: 4.0, strength: 2.0, target_height_m: 0.0, step_m: 0.0,
        op_code: 0, falloff_code: 0, layer_index: 0, flags: 0, seed: 42, pad: [0; 2],
    };
    grid.replay_stroke_journal(&[sample], "raise local tile")?;
    assert_eq!(grid.sample_height(0, 0, 8, 8), Some(7.0));
    grid.undo()?;
    assert_eq!(grid.sample_height(0, 0, 8, 8), Some(5.0));
    grid.redo()?;
    assert_eq!(grid.sample_height(0, 0, 8, 8), Some(7.0));
    println!("terrain brush local commit/undo/redo passed");
    Ok(())
}
