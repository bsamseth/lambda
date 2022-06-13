use lambda::church;
use lambda::normalize_variables;
use lambda::Expression;
use pretty_assertions::assert_eq;

fn check_normalized_correctly(code: &str, expected: &str) {
    let expr: Expression = code
        .parse()
        .expect("Failed to parse expression before normalization.");
    let normalized = normalize_variables(expr);
    assert_eq!(normalized.to_string(), expected);
}

#[test]
fn single_lambda() {
    check_normalized_correctly("λx.x", "λ1.1");
    check_normalized_correctly("λx.λy.x y x y", "λ1.λ2.((1 2) 1) 2");
}

#[test]
fn free_variables_remain() {
    check_normalized_correctly("x", "x");
    check_normalized_correctly("x y", "x y");
    check_normalized_correctly("x y", "x y");
    check_normalized_correctly("λf.f (y x)", "λ1.1 (y x)");
}

#[test]
fn constants() {
    check_normalized_correctly(church::THREE, "λ1.λ2.1 (1 (1 2))");
    check_normalized_correctly(church::PRED, "λ1.λ2.λ3.((1 (λ4.λ5.5 (4 2))) (λ6.3)) (λ7.7)");
}

#[test]
fn compound_expressions() {
    check_normalized_correctly(
        "(λb.λe.e b) (λf.λx.f (f x)) (λf.λx.f (f x))",
        "((λ1.λ2.2 1) (λ3.λ4.3 (3 4))) (λ5.λ6.5 (5 6))",
    )
}
