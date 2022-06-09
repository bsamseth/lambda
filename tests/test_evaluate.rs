use lambda::constants::church;
use lambda::evaluator::evaluate;
use lambda::parser::expr::Expression;
use pretty_assertions::assert_eq;

fn check_evaluation(code: &str, expected: &str) {
    let expr: Expression = code
        .parse()
        .expect("Failed to parse expression before evaluation.");
    let result = evaluate(expr);
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

#[test]
fn more_applications() {
    check_evaluation("(λx.x x) y", "y y");
    check_evaluation("(λx.λy.y) z", "λy.y");
    check_evaluation("(λx.λy.y x) z", "λy.y z");
    check_evaluation("((λx.λy.x y) (λx.x x)) (λx.x)", "λx.x");
}

#[test]
fn succ() {
    let one = evaluate(church::succ() * church::zero());
    assert_eq!(one.to_string(), church::ONE);

    let two_by_succ_of_one = evaluate(church::succ() * church::one());
    assert_eq!(two_by_succ_of_one.to_string(), church::TWO);

    let two_by_repeated_succ = evaluate(church::succ() * (church::succ() * church::zero()));
    assert_eq!(two_by_repeated_succ.to_string(), church::TWO);
}

#[test]
fn add() {
    let three_by_adding_one_and_two = evaluate(church::add() * church::one() * church::two());
    assert_eq!(three_by_adding_one_and_two.to_string(), church::THREE);
}

#[test]
fn mul() {
    let two_by_multiplying_one_and_two = evaluate(church::mul() * church::one() * church::two());
    assert_eq!(two_by_multiplying_one_and_two.to_string(), church::TWO);

    let four_by_multiplying_two_and_two = evaluate(church::mul() * church::two() * church::two());
    let four_by_succ_of_three = evaluate(church::succ() * church::three());
    assert_eq!(
        four_by_multiplying_two_and_two.to_string(),
        four_by_succ_of_three.to_string()
    );
}
