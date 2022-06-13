use std::collections::HashMap;

use crate::expr::Expression;

/// Normalize variables in an expression to avoid name shadowing.
///
/// This will replace all variables with unique numerical identifiers, so that no function
/// shadows a variable from a containing scope.
///
/// # Examples
/// ```rust
/// use lambda::normalize_variables;
/// assert_eq!(normalize_variables("λx.x".parse().unwrap()).to_string(), "λ1.1");
/// assert_eq!(
///     normalize_variables("λx.λy.λx.x y".parse().unwrap()).to_string(),
///     "λ1.λ2.λ3.3 2"
/// );
/// ```
pub fn normalize_variables(expr: Expression) -> Expression {
    let mut next_name = (1..).map(|n| format!("{}", n));
    let mut names: HashMap<String, String> = HashMap::new();
    normalize_variables_with_bindings(expr, &mut names, &mut next_name)
}

fn normalize_variables_with_bindings(
    expr: Expression,
    names: &mut HashMap<String, String>,
    next_name: &mut dyn Iterator<Item = String>,
) -> Expression {
    match expr {
        Expression::Application(lhs, rhs) => Expression::new_application(
            normalize_variables_with_bindings(*lhs, names, next_name),
            normalize_variables_with_bindings(*rhs, names, next_name),
        ),
        Expression::Variable(label) => {
            if let Some(name) = names.get(&label) {
                Expression::new_variable(name)
            } else {
                Expression::new_variable(&label)
            }
        }
        Expression::Function(param, body) => {
            let new_name = next_name.next().unwrap();
            let prev = names.insert(param.clone(), new_name.clone());
            let normalized = Expression::new_function(
                new_name,
                normalize_variables_with_bindings(*body, names, next_name),
            );
            if let Some(p) = prev {
                names.insert(param, p);
            } else {
                names.remove(&param);
            }
            normalized
        }
    }
}
