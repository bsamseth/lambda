use pretty_assertions::assert_eq;

use lambda::constants::church;
use lambda::evaluator::evaluate;
use lambda::expr::Expression;

fn check_is_equal(code: &str, expected: &str) {
    let expr = code
        .parse::<Expression>()
        .expect("Failed to parse test code.")
        .evaluate();
    check_expr_is(&expr, expected);
}

fn check_expr_is(expr: &Expression, expected: &str) {
    let expected = expected
        .parse::<Expression>()
        .expect("Failed to parse expected test result.")
        .normalize();
    assert_eq!(expr.to_string(), expected.to_string());
}

fn check_expr_against_expr(expr: &Expression, expected: &Expression) {
    check_expr_is(expr, &expected.to_string());
}

#[test]
fn single_variable() {
    check_is_equal("x", "x");
}

#[test]
fn single_function() {
    check_is_equal("λx.x", "λx.x");
}

#[test]
fn single_application() {
    check_is_equal("(λx.x) y", "y");
}

#[test]
fn more_applications() {
    check_is_equal("(λx.x x) y", "y y");
    check_is_equal("(λx.λy.y) z", "λy.y");
    check_is_equal("(λx.λy.y x) z", "λy.y z");
    check_is_equal("((λx.λy.x y) (λx.x x)) (λx.x)", "λx.x");
}

#[test]
fn succ() {
    let one = evaluate(church::succ() * church::zero());
    check_expr_is(&one, church::ONE);

    let two_by_succ_of_one = evaluate(church::succ() * church::one());
    check_expr_is(&two_by_succ_of_one, church::TWO);

    let two_by_repeated_succ = evaluate(church::succ() * (church::succ() * church::zero()));
    check_expr_is(&two_by_repeated_succ, church::TWO);
}

#[test]
fn pred() {
    let one_as_pred_of_two = evaluate(church::pred() * church::two());
    check_expr_is(&one_as_pred_of_two, church::ONE);

    let one_by_repeated_pred = evaluate(church::pred() * (church::pred() * church::three()));
    check_expr_is(&one_by_repeated_pred, church::ONE);
}

#[test]
fn add() {
    let three_by_adding_one_and_two = evaluate(church::add() * church::one() * church::two());
    check_expr_is(&three_by_adding_one_and_two, church::THREE);
}

#[test]
fn mul() {
    let two_by_multiplying_one_and_two = evaluate(church::mul() * church::one() * church::two());
    check_expr_is(&two_by_multiplying_one_and_two, church::TWO);

    let four_by_multiplying_two_and_two = evaluate(church::mul() * church::two() * church::two());
    let four_by_succ_of_three = evaluate(church::succ() * church::three());
    check_expr_against_expr(&four_by_multiplying_two_and_two, &four_by_succ_of_three);
}

#[test]
fn pow() {
    let four_by_succ_of_three = evaluate(church::succ() * church::three());
    let four_by_pow = evaluate(church::pow() * church::two() * church::two());
    check_expr_against_expr(&four_by_pow, &four_by_succ_of_three);

    let sixteen_by_pow = evaluate(church::pow() * church::two() * four_by_pow.clone());
    let sixteen_by_mul = evaluate(church::mul() * four_by_pow.clone() * four_by_pow);
    check_expr_against_expr(&sixteen_by_pow, &sixteen_by_mul);
}
