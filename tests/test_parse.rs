use lambda::parser::ParseResult;

fn check_parsed_correctly(code: &str, expected: &str) {
    let expr: ParseResult = code.parse();
    assert!(expr.is_ok());
    assert_eq!(expr.unwrap().to_string(), expected);
}

#[test]
fn single_variable() {
    check_parsed_correctly("x", "x");
}
#[test]
fn two_variable_application() {
    check_parsed_correctly("x y", "x y");
}
#[test]
fn variable_application_with_parens() {
    check_parsed_correctly("(x y)", "x y");
    check_parsed_correctly("(x (y))", "x y");
    check_parsed_correctly("z ((x (y)))", "z (x y)");
    check_parsed_correctly("(z ((x (y))))", "z (x y)");
    check_parsed_correctly("((z u) ((x (y))))", "(z u) (x y)");
    check_parsed_correctly("(x y) (u v)", "(x y) (u v)");
}
#[test]
fn left_associativity() {
    check_parsed_correctly("x y z", "(x y) z");
    check_parsed_correctly("x (y z)", "x (y z)");
    check_parsed_correctly("x (y z) u", "(x (y z)) u");
}
#[test]
fn lambdas() {
    check_parsed_correctly("λx.x", "λx.x");
    check_parsed_correctly("λx.λy.x (x y)", "λx.λy.x (x y)");
    check_parsed_correctly("λf.λx.f (f (f x))", "λf.λx.f (f (f x))");
    check_parsed_correctly("λf.λx.f (f (f x))", "λf.λx.f (f (f x))");
}
#[test]
fn combined_expressions() {
    check_parsed_correctly("(λx.x) (λy.y) z", "((λx.x) (λy.y)) z");
    check_parsed_correctly("λu.(λx.x) (λy.y)", "λu.(λx.x) (λy.y)");
    check_parsed_correctly(
        "(λh.(λx.h (x x)) (λx.h (x x))) g",
        "(λh.(λx.h (x x)) (λx.h (x x))) g",
    );
    check_parsed_correctly(
        "g (\n(λx.g(x x))(λx.g (  x   x)))",
        "g ((λx.g (x x)) (λx.g (x x)))",
    );
}
