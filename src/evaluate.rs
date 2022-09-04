use std::collections::HashMap;

use crate::expr::Expression;
use crate::normalize::normalize_variables;

/// Evaluate an expression and return the normalized result.
///
/// This can also be used as a method on `Expression`.
/// # Examples
/// ```rust
/// use lambda::church;
/// use lambda::Expression;
///
/// let expr: Expression = "(λn.λf.λx.f (n f x)) λf.λx.x".parse().unwrap();
/// let one = lambda::evaluate(expr);
/// assert_eq!(one.to_string(), "λ1.λ2.1 2");
///
/// let three = church::add() * church::one() * church::two();
/// assert_eq!(three.evaluate(), church::three().normalize());
/// ```
pub fn evaluate(expr: Expression) -> Expression {
    evaluate_normalized(normalize_variables(expr))
}

/// Evaluate an expression that is already normalized.
///
/// The expression is assumed to be normalized, and the result is not defined
/// if this is not true. Note that it can be normalized differently to what
/// lambda::normalize::normalize_variables does, it just means there is no name
/// shadowing in the expression.
///
/// The result of the evaluation will be normalized.
pub fn evaluate_normalized(expr: Expression) -> Expression {
    normalize_variables(evaluate_no_normalization(expr))
}

/// Evaluate an expression that is already normalized, without post-normalization
///
/// See evaluate_normalized for more information. This will do what evaluate_normalized does,
/// only it will not apply normalization to the result of the evaluation.
pub fn evaluate_no_normalization(expr: Expression) -> Expression {
    _evaluate(expr, &mut HashMap::new())
}

fn _evaluate(expr: Expression, bindings: &mut HashMap<String, Expression>) -> Expression {
    match expr {
        Expression::Variable(label) => {
            // If the variable is bound, return the bound value, otherwise just return the variable.
            (*bindings.get(&label).unwrap_or(&Expression::Variable(label))).clone()
        }
        Expression::Function(param, body) => {
            Expression::new_function(param, _evaluate(*body, bindings))
        }
        Expression::Application(lhs, rhs) => {
            let rhs = _evaluate(*rhs, bindings);

            // If the lhs is a function, we want to apply it to the rhs. However, the lhs might
            // not _yet_ be a function, and will only reduce to a function after being evaluated.
            // If and only if it is an application, we evaluate the lhs first.
            let lhs = match *lhs {
                Expression::Application(_, _) => _evaluate(*lhs, bindings),
                _ => *lhs,
            };

            match lhs {
                Expression::Function(param, body) => {
                    // Perform β-reduction, i.e. apply the lhs with the rhs as the argument.
                    bindings.insert(param.clone(), rhs);
                    let result = _evaluate(*body, bindings);
                    bindings.remove(param.as_str());

                    // At this point we have performed β-reduction. It might still be that the result
                    // is an application of a function, which could be further reduced. We therefore,
                    // recursively evaluate the result once more. Note that if a function application
                    // evaluates to itself, this would become an infinite loop. Evaluating such an
                    // expression would be an infinite computation, so crashing here seems reasonable.
                    // Could possibly try to detect this and just return the expression.
                    _evaluate(result, bindings)
                }
                _ => {
                    // The application is abstract, so can't be β-reduced. In this case we just
                    // evaluate the lhs as well and return the abstract application.
                    let lhs = _evaluate(lhs, bindings);
                    Expression::new_application(lhs, rhs)
                }
            }
        }
    }
}
