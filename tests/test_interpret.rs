use lambda::interpret::evaluate;
use lambda::parser::expr::Expression;

fn check_evaluation(code: &str, expected: &str) {
    let expr: Expression = code
        .parse()
        .expect("Failed to parse expression before evaluation.");
    let result = evaluate(&expr).expect("Failed to evaluate expression.");
    assert_eq!(result.to_string(), expected);
}

#[test]
fn single_variable() {
    check_evaluation("x", "x");
}

#[test]
fn single_function() {
    check_evaluation("λx.x", "λx.x");
}

#[test]
fn single_application() {
    check_evaluation("(λx.x) y", "y");
}
