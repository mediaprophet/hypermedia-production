//! Exact admitted f64 snapshot instant -> shared integer/rational clock.
//! Uses IEEE bits and Euclidean integer division; never subtracts floor in f64.
use qualia_core_db::q42::emf_profiles::FieldClock;
#[derive(Clone, Copy, Debug)]
pub(crate) struct SnapshotClock {
    pub clock: FieldClock,
    pub start_tick: i64,
}
pub(crate) fn from_f64_epoch(value: f64) -> Result<SnapshotClock, String> {
    if !value.is_finite() {
        return Err("Snapshot epoch must be finite".into());
    }
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let encoded = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    let mut significand = if encoded == 0 {
        fraction
    } else {
        fraction | (1u64 << 52)
    };
    if significand == 0 {
        return Ok(snapshot(0, 1, 1, 0));
    }
    let mut exponent = if encoded == 0 {
        -1074
    } else {
        encoded - 1023 - 52
    };
    let trailing = significand.trailing_zeros();
    significand >>= trailing;
    exponent += trailing as i32;
    if exponent < 0 {
        let power = (-exponent) as u32;
        if power > 49 {
            return Err("Exact binary epoch fraction exceeds shared clock numerator bound".into());
        }
        let denominator = 1i128 << power;
        let signed = if negative {
            -i128::from(significand)
        } else {
            i128::from(significand)
        };
        let seconds = signed.div_euclid(denominator);
        let tick = signed.rem_euclid(denominator);
        return Ok(snapshot(
            i64::try_from(seconds).map_err(|_| "Epoch seconds exceed i64")?,
            denominator as u64,
            1,
            i64::try_from(tick).map_err(|_| "Epoch tick exceeds i64")?,
        ));
    }
    if exponent >= 127 || i128::from(significand) > (i128::MAX >> (exponent as u32)) {
        return Err("Exact epoch exceeds supported integer range".into());
    }
    let magnitude = i128::from(significand) << (exponent as u32);
    let signed = if negative { -magnitude } else { magnitude };
    if let Ok(seconds) = i64::try_from(signed) {
        return Ok(snapshot(seconds, 1, 1, 0));
    }
    // Maximum admitted seconds/tick is1e12. Epoch seconds absorb the bounded
    // remainder, admitting all exact integers within epoch+tick i64 bounds.
    let seconds_per_tick = 1_000_000_000_000i128;
    let quotient = signed.div_euclid(seconds_per_tick);
    let tick = quotient.clamp(i128::from(i64::MIN), i128::from(i64::MAX));
    let seconds = signed
        .checked_sub(
            tick.checked_mul(seconds_per_tick)
                .ok_or("Epoch product overflow")?,
        )
        .ok_or("Epoch subtraction overflow")?;
    let seconds =
        i64::try_from(seconds).map_err(|_| "Exact epoch exceeds shared clock/i64 range")?;
    Ok(snapshot(seconds, 1, seconds_per_tick as u64, tick as i64))
}
fn snapshot(seconds: i64, numerator: u64, denominator: u64, tick: i64) -> SnapshotClock {
    SnapshotClock {
        clock: FieldClock {
            scale: 1,
            epoch_seconds: seconds,
            epoch_nanoseconds: 0,
            ticks_per_second_num: numerator,
            ticks_per_second_den: denominator,
        },
        start_tick: tick,
    }
}
