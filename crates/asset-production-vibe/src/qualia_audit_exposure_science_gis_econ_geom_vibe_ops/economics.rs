//! Bounded scenario adapters for native market and engineering operators.

use crate::{
    count, fields, integer, list, num, number, required, text, vector, Args, AssetHost, Value,
};
use qualia_core_db::specialized_libs::{
    computational_economics::{
        derivatives, market_design as market, network_economics, public_finance, risk,
    },
    engineering_analysis as survival,
};
use std::collections::BTreeMap;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    let output = match op {
        "stable_matching" => matching(host, args)?,
        "interbank_clearing" => interbank(host, args)?,
        "survival_floor" => floor(host, args)?,
        "black_scholes" => option(args)?,
        "historical_risk" => historical_risk(host, args)?,
        "basquin_miner" => fatigue(host, args)?,
        "vickrey_auction" | "double_auction" | "uniform_price_auction" => auction(host, op, args)?,
        _ => return Err(format!("Unsupported market or engineering operation: {op}")),
    };
    Ok(Value::Record(output))
}

fn series(value: &Value, maximum: usize) -> Result<Vec<f64>, String> {
    let values = list(value)?;
    if values.is_empty() || values.len() > maximum {
        return Err(format!(
            "Expected a nonempty numerical series of at most {maximum} values"
        ));
    }
    values.iter().map(num).collect()
}

fn scalar_record<const N: usize>(values: [(&str, f64); N]) -> Result<Args, String> {
    values
        .into_iter()
        .map(|(key, value)| {
            if !value.is_finite() {
                return Err(format!(
                    "The native operator did not produce a finite {key}"
                ));
            }
            Ok((key.into(), Value::F64(value)))
        })
        .collect()
}

fn series_value(values: Vec<f64>) -> Result<Value, String> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err("The native operator produced non-finite results".into());
    }
    Ok(Value::List(values.into_iter().map(Value::F64).collect()))
}

fn matching(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["proposer_preferences", "receiver_preferences"])?;
    let proposers = list(required(args, "proposer_preferences")?)?;
    let receivers = list(required(args, "receiver_preferences")?)?;
    let n = proposers.len();
    if n == 0 || n > market::MAX_AGENTS || receivers.len() != n {
        return Err("Stable matching requires equal, nonempty bounded groups".into());
    }
    let preferences = |rows: &[Value]| -> Result<Vec<usize>, String> {
        let mut preferences = Vec::with_capacity(n * n);
        let mut seen = vec![false; n];
        for row in rows {
            let values = list(row)?;
            if values.len() != n {
                return Err("Preference rows must rank every member of the other group".into());
            }
            seen.fill(false);
            for value in values {
                let index = integer(value, "preference rank")?;
                if index >= n || seen[index] {
                    return Err("Preference rows must be permutations of the opposite group".into());
                }
                seen[index] = true;
                preferences.push(index);
            }
        }
        Ok(preferences)
    };
    host.charge(n * n * n, (2 * n * n + n) * std::mem::size_of::<usize>())?;
    let left = preferences(proposers)?;
    let right = preferences(receivers)?;
    let mut partners = vec![0; n];
    market::deferred_acceptance_into(&left, &right, n, &mut partners)
        .map_err(|error| format!("Stable matching: {error:?}"))?;
    let stable = market::is_stable_matching(&left, &right, &partners, n)
        .map_err(|error| format!("Matching verification: {error:?}"))?;
    if !stable {
        return Err("Native matching did not satisfy stability".into());
    }
    Ok(BTreeMap::from([
        (
            "matches".into(),
            Value::List(partners.into_iter().map(|i| Value::U64(i as u64)).collect()),
        ),
        ("stable".into(), Value::Bool(stable)),
    ]))
}

fn interbank(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["liabilities", "capital", "rounds", "tolerance"])?;
    let capital = series(required(args, "capital")?, network_economics::MAX_NODES)?;
    let n = capital.len();
    let liabilities = list(required(args, "liabilities")?)?;
    if liabilities.len() != n || capital.iter().any(|value| *value < 0.0) {
        return Err(
            "Interbank clearing requires a square liability matrix and nonnegative capital".into(),
        );
    }
    let rounds = count(args, "rounds", 256, 10_000)?;
    let tolerance = number(args, "tolerance", 1e-9)?;
    host.charge(rounds * n * n, (n * n + 2 * n) * std::mem::size_of::<f64>())?;
    let mut exposures = Vec::with_capacity(n * n);
    for row in liabilities {
        let values = series(row, n)?;
        if values.len() != n
            || values.iter().any(|value| *value < 0.0)
            || !values.iter().sum::<f64>().is_finite()
        {
            return Err("Interbank liability rows require finite nonnegative totals".into());
        }
        exposures.extend(values);
    }
    let mut payments = vec![0.0; n];
    let convergence = network_economics::interbank_clearing_into(
        &exposures,
        &capital,
        n,
        rounds as u32,
        tolerance,
        &mut payments,
    )
    .map_err(|error| format!("Interbank clearing: {error:?}"))?;
    let mut output = scalar_record([("residual", convergence.residual)])?;
    output.insert("payments".into(), series_value(payments)?);
    output.insert(
        "iterations".into(),
        Value::U64(u64::from(convergence.iterations)),
    );
    output.insert(
        "status".into(),
        Value::String(format!("{:?}", convergence.status).into()),
    );
    Ok(output)
}

fn floor(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["needs", "budget"])?;
    let needs = series(required(args, "needs")?, 65_536)?;
    if !needs.iter().sum::<f64>().is_finite() {
        return Err("Survival needs total exceeds finite limits".into());
    }
    host.charge(needs.len(), 2 * needs.len() * std::mem::size_of::<f64>())?;
    let mut allocations = vec![0.0; needs.len()];
    let unmet = public_finance::survival_floor_allocation_into(
        &needs,
        num(required(args, "budget")?)?,
        &mut allocations,
    )
    .map_err(|error| format!("Survival-floor allocation: {error:?}"))?;
    let mut output = scalar_record([("unmet_need", unmet)])?;
    output.insert("allocations".into(), series_value(allocations)?);
    Ok(output)
}

fn option(args: &Args) -> Result<Args, String> {
    fields(
        args,
        &[
            "kind",
            "spot",
            "strike",
            "rate",
            "dividend_yield",
            "volatility",
            "time_years",
        ],
    )?;
    let kind = match text(required(args, "kind")?)? {
        "call" => derivatives::OptionKind::Call,
        "put" => derivatives::OptionKind::Put,
        _ => return Err("Option kind must be call or put".into()),
    };
    let scalar = |name| num(required(args, name)?);
    let result = derivatives::black_scholes_price_and_greeks(
        kind,
        scalar("spot")?,
        scalar("strike")?,
        scalar("rate")?,
        number(args, "dividend_yield", 0.0)?,
        scalar("volatility")?,
        scalar("time_years")?,
    )
    .map_err(|error| format!("Black–Scholes calculation: {error:?}"))?;
    scalar_record([
        ("price", result.price),
        ("delta", result.delta),
        ("gamma", result.gamma),
        ("vega", result.vega),
        ("theta", result.theta),
        ("rho", result.rho),
    ])
}

fn historical_risk(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["returns", "confidence"])?;
    let returns = series(required(args, "returns")?, 16_384)?;
    let confidence = num(required(args, "confidence")?)?;
    if confidence <= 0.0 || confidence >= 1.0 {
        return Err("Historical risk requires confidence strictly between zero and one".into());
    }
    host.charge(
        returns.len() * returns.len(),
        2 * returns.len() * std::mem::size_of::<f64>(),
    )?;
    let mut scratch = vec![0.0; returns.len()];
    let var = risk::historical_var(&returns, confidence, &mut scratch)
        .map_err(|error| format!("Historical VaR: {error:?}"))?;
    let cvar = risk::historical_cvar(&returns, confidence, &mut scratch)
        .map_err(|error| format!("Historical CVaR: {error:?}"))?;
    scalar_record([("var", var), ("cvar", cvar)])
}

fn fatigue(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["strength", "exponent", "load_blocks"])?;
    let strength = num(required(args, "strength")?)?;
    let exponent = num(required(args, "exponent")?)?;
    let blocks = list(required(args, "load_blocks")?)?;
    if strength <= 0.0 || exponent >= 0.0 || blocks.is_empty() || blocks.len() > 65_536 {
        return Err(
            "Fatigue requires positive strength, negative exponent, and bounded load blocks".into(),
        );
    }
    host.charge(
        blocks.len(),
        blocks.len() * std::mem::size_of::<(f64, f64)>(),
    )?;
    let mut cycle_limits = Vec::with_capacity(blocks.len());
    for block in blocks {
        let [stress, cycles] = vector::<2>(block)?;
        if stress <= 0.0 || cycles < 0.0 {
            return Err("Load blocks require positive stress and nonnegative cycle count".into());
        }
        let allowable = survival::fatigue_cycles_basquin(stress, strength, exponent);
        if !allowable.is_finite() || allowable <= 0.0 {
            return Err("Basquin evaluation exceeds finite cycle range".into());
        }
        cycle_limits.push((cycles, allowable));
    }
    let damage = survival::miner_cumulative_damage(&cycle_limits);
    let mut output = scalar_record([("damage_fraction", damage)])?;
    output.insert(
        "allowable_cycles".into(),
        series_value(cycle_limits.into_iter().map(|(_, n)| n).collect())?,
    );
    Ok(output)
}

fn auction(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Args, String> {
    let quantity;
    let winner;
    let price;
    match op {
        "vickrey_auction" => {
            fields(args, &["bids"])?;
            let bids = series(required(args, "bids")?, market::MAX_AGENTS)?;
            host.charge(
                bids.len() * bids.len(),
                std::mem::size_of_val(bids.as_slice()),
            )?;
            let (index, clearing_price) = market::vickrey_auction(&bids)
                .map_err(|error| format!("Vickrey auction: {error:?}"))?;
            winner = Some(index);
            quantity = 1;
            price = clearing_price;
        }
        "double_auction" => {
            fields(args, &["bids", "asks"])?;
            let bids = series(required(args, "bids")?, market::MAX_AGENTS)?;
            let asks = series(required(args, "asks")?, market::MAX_AGENTS)?;
            host.charge(
                bids.len().pow(2) + asks.len().pow(2),
                (bids.len() + asks.len()) * std::mem::size_of::<f64>(),
            )?;
            (price, quantity) = market::double_auction(&bids, &asks)
                .map_err(|error| format!("Double auction: {error:?}"))?;
            winner = None;
        }
        _ => {
            fields(args, &["bids", "supply"])?;
            let bids = series(required(args, "bids")?, market::MAX_AGENTS)?;
            let supply = integer(required(args, "supply")?, "supply")?;
            host.charge(
                bids.len() * bids.len(),
                std::mem::size_of_val(bids.as_slice()),
            )?;
            (price, quantity) = market::uniform_price_auction(supply, &bids)
                .map_err(|error| format!("Uniform-price auction: {error:?}"))?;
            winner = None;
        }
    }
    let mut output = scalar_record([("clearing_price", price)])?;
    output.insert("units_traded".into(), Value::U64(quantity as u64));
    if let Some(winner) = winner {
        output.insert("winner".into(), Value::U64(winner as u64));
    }
    Ok(output)
}
