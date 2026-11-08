//! Function library and dispatcher.

pub mod dynamic;
pub mod financial;
pub mod lookup;
pub mod math;
pub mod stat;
pub mod text;
pub mod util;

use crate::core::date::DateSystem;
use crate::core::value::{CellError, Value};

pub fn dispatch(name: &str, args: &[Value], _sys: DateSystem) -> Value {
    let r = match name {
        // Financial (34 functions)
        "PMT" => financial::pmt(args),
        "FV" => financial::fv(args),
        "PV" => financial::pv(args),
        "NPER" => financial::nper(args),
        "IPMT" => financial::ipmt(args),
        "PPMT" => financial::ppmt(args),
        "CUMIPMT" => financial::cumipmt(args),
        "CUMPRINC" => financial::cumprinc(args),
        "RATE" => financial::rate(args),
        "NPV" => financial::npv(args),
        "XNPV" => financial::xnpv(args),
        "IRR" => financial::irr(args),
        "XIRR" => financial::xirr(args),
        "MIRR" => financial::mirr(args),
        "EFFECT" => financial::effect(args),
        "NOMINAL" => financial::nominal(args),
        "SLN" => financial::sln(args),
        "SYD" => financial::syd(args),
        "DB" => financial::db(args),
        "DDB" => financial::ddb(args),
        "VDB" => financial::vdb(args),
        "DOLLARDE" => financial::dollarde(args),
        "DOLLARFR" => financial::dollarfr(args),
        "DURATION" => financial::duration(args),
        "MDURATION" => financial::mduration(args),
        "PRICE" => financial::price_bond(args),
        "YIELD" => financial::yield_maturity(args),
        "DISC" => financial::disc(args),
        "TBILLEQ" => financial::tbilleq(args),
        "TBILLPRICE" => financial::tbillprice(args),
        "TBILLYIELD" => financial::tbillyield(args),
        "RECEIVED" => financial::received(args),
        "INTRATE" => financial::intrate(args),
        "PRICEDISC" => financial::pricedisc(args),
        "YIELDDISC" => financial::yielddisc(args),

        // Statistics & Distributions
        "NORM.DIST" | "NORMDIST" => stat::norm_dist(args),
        "NORM.S.DIST" | "NORMSDIST" => stat::norm_s_dist(args),
        "NORM.INV" | "NORMINV" => stat::norm_inv(args),
        "POISSON.DIST" | "POISSON" => stat::poisson_dist(args),
        "GAMMA.DIST" | "GAMMADIST" => stat::gamma_dist(args),
        "WEIBULL.DIST" | "WEIBULL" => stat::weibull_dist(args),
        "CHISQ.DIST" | "CHIDIST" => stat::chisq_dist(args),
        "T.DIST" | "TDIST" => stat::t_dist(args),
        "AVERAGE" => stat::average(args),
        "COUNT" => stat::count(args),
        "MAX" => stat::max(args),
        "MIN" => stat::min(args),
        "STDEV.S" | "STDEV" => stat::stdev_s(args),
        "VAR.S" | "VAR" => stat::var_s(args),

        // Dynamic Arrays
        "SEQUENCE" => dynamic::sequence(args),
        "FILTER" => dynamic::filter(args),
        "UNIQUE" => dynamic::unique(args),
        "SORT" => dynamic::sort(args),
        "TAKE" => dynamic::take(args),
        "DROP" => dynamic::drop(args),

        // Lookup & Reference
        "MATCH" | "XMATCH" => lookup::match_fn(args),
        "INDEX" => lookup::index_fn(args),
        "XLOOKUP" => lookup::xlookup(args),
        "VLOOKUP" => lookup::vlookup(args),

        // Math
        "SUM" => math::sum(args),
        "PRODUCT" => math::product(args),
        "ABS" => math::abs(args),
        "ROUND" => math::round(args),
        "FLOOR" => math::floor(args),
        "CEILING" => math::ceiling(args),
        "MOD" => math::mod_fn(args),
        "SQRT" => math::sqrt(args),
        "POWER" => math::power(args),
        "LN" => math::ln(args),
        "LOG10" => math::log10(args),
        "EXP" => math::exp_fn(args),

        // Text
        "CONCAT" | "CONCATENATE" => text::concat(args),
        "TEXTJOIN" => text::textjoin(args),
        "LEFT" => text::left(args),
        "RIGHT" => text::right(args),
        "MID" => text::mid(args),
        "LEN" => text::len(args),
        "TRIM" => text::trim(args),
        "UPPER" => text::upper(args),
        "LOWER" => text::lower(args),
        "EXACT" => text::exact(args),

        _ => Err(CellError::Name),
    };

    match r {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    }
}
