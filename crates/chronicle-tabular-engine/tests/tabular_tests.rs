use chronicle_tabular_engine::*;
use chronicle_tabular_engine::core::Value;
use chronicle_tabular_engine::formula::parse;

#[test]
fn test_formula_parsing() {
    let expr = parse("SUM(A1:B2) * 2 + 10").expect("parse should succeed");
    assert!(matches!(expr, Expr::Binary(BinOp::Add, _, _)));
    let text = print(&expr);
    assert!(text.contains("SUM(A1:B2)"));
}

#[test]
fn test_dag_recalculation_and_cycles() {
    let mut wb = Workbook::new();
    let s0 = 0;

    // A1 = 10
    wb.sheet_mut(s0).unwrap().set_value(CellRef::new(0, 0), Value::Number(10.0));
    // A2 = A1 * 2
    wb.sheet_mut(s0).unwrap().set_formula(CellRef::new(1, 0), Formula::new("=A1*2"));
    // A3 = A2 + 5
    wb.sheet_mut(s0).unwrap().set_formula(CellRef::new(2, 0), Formula::new("=A2+5"));

    let mut calc = Calc::new();
    calc.recalc_all(&mut wb);

    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(1, 0)), Value::Number(20.0));
    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(2, 0)), Value::Number(25.0));

    // Circular dependency: B1 = B2, B2 = B1
    wb.sheet_mut(s0).unwrap().set_formula(CellRef::new(0, 1), Formula::new("=B2"));
    wb.sheet_mut(s0).unwrap().set_formula(CellRef::new(1, 1), Formula::new("=B1"));

    calc.recalc_all(&mut wb);

    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(0, 1)), Value::Error(CellError::Circ));
    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(1, 1)), Value::Error(CellError::Circ));
}

#[test]
fn test_financial_pmt_npv_xirr() {
    // PMT(0.08 / 12, 10, 10000)
    let rate = Value::Number(0.08 / 12.0);
    let nper = Value::Number(10.0);
    let pv = Value::Number(10000.0);
    let res = chronicle_tabular_engine::functions::financial::pmt(&[rate, nper, pv]).unwrap();
    if let Value::Number(p) = res {
        assert!((p - (-1037.03)).abs() < 1.0);
    } else {
        panic!("expected number");
    }

    // NPV(0.1, -1000, 300, 420, 680)
    let npv_res = chronicle_tabular_engine::functions::financial::npv(&[
        Value::Number(0.1),
        Value::Number(300.0),
        Value::Number(420.0),
        Value::Number(680.0),
    ]).unwrap();
    if let Value::Number(v) = npv_res {
        assert!(v > 1000.0);
    } else {
        panic!("expected number");
    }

    // XIRR
    let flows = Value::Array(std::sync::Arc::new(Array::new(3, 1, vec![
        Value::Number(-10000.0),
        Value::Number(2750.0),
        Value::Number(8500.0),
    ]).unwrap()));
    let dates = Value::Array(std::sync::Arc::new(Array::new(3, 1, vec![
        Value::Number(40000.0),
        Value::Number(40100.0),
        Value::Number(40365.0),
    ]).unwrap()));
    let xirr_res = chronicle_tabular_engine::functions::financial::xirr(&[flows, dates]).unwrap();
    if let Value::Number(r) = xirr_res {
        assert!(r > 0.05 && r < 0.20);
    } else {
        panic!("expected number");
    }
}

#[test]
fn test_dynamic_arrays_and_spill() {
    let mut wb = Workbook::new();
    let s0 = 0;

    // A1 = SEQUENCE(3, 2, 1, 1)
    wb.sheet_mut(s0).unwrap().set_formula(CellRef::new(0, 0), Formula::new("=SEQUENCE(3, 2, 1, 1)"));

    let mut calc = Calc::new();
    calc.recalc_all(&mut wb);

    // Anchor has the array, and spilled cells are populated
    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(0, 1)), Value::Number(2.0));
    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(1, 0)), Value::Number(3.0));
    assert_eq!(wb.sheet(s0).unwrap().value(CellRef::new(2, 1)), Value::Number(6.0));
}
