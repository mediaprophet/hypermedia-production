use super::*;
fn seconds(snapshot: super::super::clock::SnapshotClock) -> (i128, i128) {
    let c = snapshot.clock;
    assert_eq!(c.epoch_nanoseconds, 0);
    (
        i128::from(c.epoch_seconds) * i128::from(c.ticks_per_second_num)
            + i128::from(snapshot.start_tick) * i128::from(c.ticks_per_second_den),
        i128::from(c.ticks_per_second_num),
    )
}
#[test]
fn positive_negative_fractional_epochs_are_exact_euclidean_rationals() {
    for (value, num, den) in [
        (1.5, 3i128, 2i128),
        (-1.5, -3, 2),
        (1_000_000_000.125, 8_000_000_001, 8),
        (-0.125, -1, 8),
        (0.125, 1, 8),
    ] {
        let snapshot = super::super::clock::from_f64_epoch(value).unwrap();
        let (n, d) = seconds(snapshot);
        assert_eq!(n * den, num * d);
        if value < 0.0 {
            assert!(snapshot.start_tick >= 0);
        }
    }
    let value = 2f64.powi(-49);
    for (value, sign) in [(value, 1i128), (-value, -1)] {
        let (n, d) = seconds(super::super::clock::from_f64_epoch(value).unwrap());
        assert_eq!(n * (1i128 << 49), sign * d);
    }
}
#[test]
fn large_exact_epochs_and_shared_bounds_do_not_round_or_shift_wrap() {
    let exact = 1_000_000_000_000_000_019_884_624_838_656i128;
    for (value, target) in [(1e30, exact), (-1e30, -exact)] {
        let (n, d) = seconds(super::super::clock::from_f64_epoch(value).unwrap());
        assert_eq!(n, target * d);
    }
    let max = i128::from(i64::MAX) * (1_000_000_000_000i128 + 1);
    let min = i128::from(i64::MIN) * (1_000_000_000_000i128 + 1);
    assert!(super::super::clock::from_f64_epoch((max as f64).next_down()).is_ok());
    assert!(super::super::clock::from_f64_epoch((max as f64).next_up()).is_err());
    assert!(super::super::clock::from_f64_epoch((min as f64).next_up()).is_ok());
    assert!(super::super::clock::from_f64_epoch((min as f64).next_down()).is_err());
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
        2f64.powi(-50),
        1e40,
        f64::MAX,
    ] {
        assert!(
            super::super::clock::from_f64_epoch(value).is_err(),
            "{value}"
        );
    }
    for value in [0.0, -0.0] {
        assert_eq!(
            seconds(super::super::clock::from_f64_epoch(value).unwrap()),
            (0, 1)
        );
    }
}
#[test]
fn fractional_clock_survives_current_writer_and_typed_cache() {
    for epoch in [1.5, -1.5, 1_000_000_000.125, -0.125] {
        let bytes = generated(&[emitter(None)], epoch);
        let document = open(&bytes);
        let grid = GridAtTime::new(&document, 0).unwrap();
        assert_eq!(
            grid.sample([0; 3]).unwrap().unwrap().tick(),
            super::super::clock::from_f64_epoch(epoch)
                .unwrap()
                .start_tick
        );
        assert!(GridAtTime::new(&document, 1).is_err());
    }
}
