//! Unit tests for locus-parametric-solver.

#![forbid(unsafe_code)]

use drafting_geometry::Vec2;

use super::solver::auto::{InferOptions, infer};
use super::solver::conflict::{SolveError, SolveOptions, solve_system};
use super::solver::expr::{Env, ExprError, parse as parse_expr};
use super::solver::lm::{jacobian, rank, solve, solve_dense};
use super::solver::model::{
    EntityKind, GeomRef, Handle, Model, SketchCircle, SketchLine, SketchPoint, Sub,
};
use super::solver::residual::{Constraint, ConstraintKind, DistAxis};
use super::typography::mtext::{MTextParams, decode_controls, layout_mtext, plain_mtext};
use super::typography::stroke::{char_advance, glyph, layout_stroke_line};

#[test]
fn test_dense_linear_solver_and_rank() {
    let a = vec![2.0, 1.0, 1.0, 3.0];
    let b = vec![3.0, 5.0];
    let y = solve_dense(a, b, 2).expect("invertible matrix");
    assert!((y[0] - 0.8).abs() < 1e-10);
    assert!((y[1] - 1.4).abs() < 1e-10);

    let singular_a = vec![1.0, 2.0, 2.0, 4.0];
    let singular_b = vec![1.0, 1.0];
    assert!(solve_dense(singular_a, singular_b, 2).is_none());

    let m_rank2 = vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    assert_eq!(rank(&m_rank2, 3, 2), 2);
}

#[test]
fn test_lm_minimum_norm() {
    // x + y = 2 starting from (0, 0) -> minimum-norm step is (1, 1)
    let f = |x: &[f64], out: &mut Vec<f64>| out.push(x[0] + x[1] - 2.0);
    let outcome = solve(&f, &[0.0, 0.0], &[0, 1], &[1.0, 1.0], 1e-12, 50);
    assert!(outcome.converged);
    assert!((outcome.x[0] - 1.0).abs() < 1e-8);
    assert!((outcome.x[1] - 1.0).abs() < 1e-8);
}

#[test]
fn test_parametric_sketch_horizontal_and_distance() {
    let h1 = Handle::new(1);
    let line1 = SketchLine {
        a: Vec2::new(0.0, 0.0),
        b: Vec2::new(4.0, 2.0),
    };
    let entities = vec![(h1, EntityKind::Line(line1))];

    let constraints = vec![
        // Make line horizontal
        Constraint {
            id: 101,
            kind: ConstraintKind::Horizontal,
            refs: vec![GeomRef::whole(h1)],
            name: String::new(),
            expr: String::new(),
        },
        // Fix start point at (0, 0)
        Constraint {
            id: 102,
            kind: ConstraintKind::Fix,
            refs: vec![GeomRef::new(h1, Sub::Start)],
            name: String::new(),
            expr: String::new(),
        },
        // Distance = 10.0
        Constraint {
            id: 103,
            kind: ConstraintKind::Distance(DistAxis::Aligned),
            refs: vec![GeomRef::whole(h1)],
            name: "L".to_string(),
            expr: "10.0".to_string(),
        },
    ];

    let mut env = Env::new();
    env.insert("L", "10.0");
    let opts = SolveOptions::default();

    let (solved_entities, report) = solve_system(&entities, &constraints, &env, &opts).expect("solve success");
    assert_eq!(solved_entities.len(), 1);
    if let EntityKind::Line(solved_line) = &solved_entities[0].1 {
        assert!((solved_line.a.x - 0.0).abs() < 1e-6);
        assert!((solved_line.a.y - 0.0).abs() < 1e-6);
        assert!((solved_line.b.y - 0.0).abs() < 1e-6);
        assert!((solved_line.b.x - 10.0).abs() < 1e-6);
    } else {
        panic!("expected Line entity");
    }
    assert!(report.moved.contains(&h1));
}

#[test]
fn test_coincident_and_tangent_solve() {
    let h_line = Handle::new(10);
    let h_circle = Handle::new(20);

    let entities = vec![
        (
            h_line,
            EntityKind::Line(SketchLine {
                a: Vec2::new(0.0, 0.0),
                b: Vec2::new(10.0, 0.0),
            }),
        ),
        (
            h_circle,
            EntityKind::Circle(SketchCircle {
                center: Vec2::new(5.0, 6.0),
                radius: 4.0,
            }),
        ),
    ];

    let constraints = vec![
        // Fix line
        Constraint {
            id: 1,
            kind: ConstraintKind::Fix,
            refs: vec![GeomRef::whole(h_line)],
            name: String::new(),
            expr: String::new(),
        },
        // Make circle tangent to line
        Constraint {
            id: 2,
            kind: ConstraintKind::Tangent,
            refs: vec![GeomRef::whole(h_line), GeomRef::whole(h_circle)],
            name: String::new(),
            expr: String::new(),
        },
    ];

    let env = Env::new();
    let opts = SolveOptions::default();
    let (solved_entities, _) = solve_system(&entities, &constraints, &env, &opts).expect("tangent solve");

    if let EntityKind::Circle(solved_circle) = &solved_entities[1].1 {
        // Distance from center y to line y=0 should equal radius 4.0
        assert!((solved_circle.center.y.abs() - 4.0).abs() < 1e-6);
    } else {
        panic!("expected circle entity");
    }
}

#[test]
fn test_conflict_detection_fails_closed() {
    let h = Handle::new(1);
    let entities = vec![(
        h,
        EntityKind::Line(SketchLine {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(5.0, 0.0),
        }),
    )];

    let constraints = vec![
        Constraint {
            id: 1,
            kind: ConstraintKind::Distance(DistAxis::Aligned),
            refs: vec![GeomRef::whole(h)],
            name: "d1".to_string(),
            expr: "10.0".to_string(),
        },
        Constraint {
            id: 2,
            kind: ConstraintKind::Distance(DistAxis::Aligned),
            refs: vec![GeomRef::whole(h)],
            name: "d2".to_string(),
            expr: "20.0".to_string(),
        },
    ];

    let env = Env::new();
    let opts = SolveOptions::default();
    let res = solve_system(&entities, &constraints, &env, &opts);
    match res {
        Err(SolveError::Conflict { ids }) => {
            assert!(ids.contains(&1) || ids.contains(&2));
        }
        other => panic!("expected conflict error, got {other:?}"),
    }
}

#[test]
fn test_expression_evaluation() {
    let mut env = Env::new();
    env.insert("W", "100");
    env.insert("H", "W / 2 + 10");
    env.insert("diag", "sqrt_mock"); // we can test nested expr

    assert_eq!(env.eval("W + 5").unwrap(), 105.0);
    assert_eq!(env.eval("H").unwrap(), 60.0);
    assert_eq!(env.eval("2 * pi").unwrap(), 2.0 * std::f64::consts::PI);
    assert_eq!(env.eval("-(10 + 5)").unwrap(), -15.0);

    // Division by zero
    assert_eq!(env.eval("10 / 0"), Err(ExprError::DivZero));

    // Cycle detection
    env.insert("A", "B + 1");
    env.insert("B", "A + 1");
    assert_eq!(env.eval("A"), Err(ExprError::Cycle("A".to_string())));
}

#[test]
fn test_autoconstrain_inference() {
    let entities = vec![
        (
            Handle::new(1),
            EntityKind::Line(SketchLine {
                a: Vec2::new(0.0, 0.0),
                b: Vec2::new(10.0, 0.001), // nearly horizontal
            }),
        ),
        (
            Handle::new(2),
            EntityKind::Line(SketchLine {
                a: Vec2::new(10.002, 0.001), // nearly coincident with end of line 1
                b: Vec2::new(10.002, 5.0),   // nearly vertical
            }),
        ),
    ];

    let opts = InferOptions {
        distance_tolerance: 0.05,
        angle_tolerance: 1.0,
        types: Vec::new(),
    };

    let candidates = infer(&entities, &opts);
    assert!(candidates.iter().any(|c| c.kind == ConstraintKind::Horizontal));
    assert!(candidates.iter().any(|c| c.kind == ConstraintKind::Vertical));
    assert!(candidates.iter().any(|c| c.kind == ConstraintKind::Coincident));
}

#[test]
fn test_single_stroke_cam_rendering() {
    assert!(glyph('A').is_some());
    assert!(glyph('0').is_some());
    assert!(char_advance('A') > 0.0);

    let layout = layout_stroke_line("CAD 2026", 10.0, 1.0, 0.0);
    assert!(!layout.strokes.is_empty());
    assert!(layout.width > 20.0);
}

#[test]
fn test_mtext_parsing_and_layout() {
    // Control codes
    let decoded = decode_controls("%%d %%p %%c %%% %%uUNDER%%u %%065");
    let chars: String = decoded.into_iter().map(|(c, _, _)| c).collect();
    assert!(chars.contains('°'));
    assert!(chars.contains('±'));
    assert!(chars.contains('⌀'));
    assert!(chars.contains('%'));
    assert!(chars.contains('A')); // %%065

    // Plain text extraction with stacked fraction
    let plain = plain_mtext(r"Depth: \S1/2; in\PNew line");
    assert_eq!(plain, "Depth: 1/2 in\nNew line");

    // Layout
    let params = MTextParams {
        insert: Vec2::new(10.0, 20.0),
        height: 2.5,
        width: 100.0,
        attach: 1, // TL
        rotation: 0.0,
        line_spacing: 1.5,
        width_factor: 1.0,
    };
    let layout = layout_mtext(r"Title\P\S3/4; inch diameter", &params);
    assert_eq!(layout.pieces.len(), 2);
    assert!(layout.total_height > 0.0);
    assert!(layout.bounds.width() > 0.0);
}
