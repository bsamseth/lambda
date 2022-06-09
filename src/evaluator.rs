use crate::parser::expr::Expression;
use std::collections::HashMap;

pub fn evaluate(expr: Expression) -> Expression {
    eval_with_variable_bindings(expr, &mut HashMap::new())
}

fn eval_with_variable_bindings(
    expr: Expression,
    bindings: &mut HashMap<String, Expression>,
) -> Expression {
    match expr {
        Expression::Variable(label) => {
            // If the variable is bound, return the bound value, otherwise just return the variable.
            (*bindings.get(&label).unwrap_or(&Expression::Variable(label))).clone()
        }
        Expression::Function(param, body) => {
            // Functions evaluate just need to recursively evaluate the body.
            Expression::new_function(param, eval_with_variable_bindings(*body, bindings))
        }
        Expression::Application(lhs, rhs) => {
            // Evaluate the rhs with existing bindings before anything else, irrespective of the lhs.
            // If the lhs is a function, then we _must_ do this before binding it to the parameter
            // of the lhs function. This is because the parameter might shadow an existing binding
            // that should be applied to the rhs first.
            // If the lhs is not a function, then it doesn't matter which order we do this in.
            let rhs = eval_with_variable_bindings(*rhs, bindings);

            // If the lhs is a function, we want to apply it to the rhs. However, the lhs might
            // not _yet_ be a function, and will only reduce to a function after being evaluated.
            // If and only if it is an application, we evaluate the lhs first.
            let lhs = match *lhs {
                Expression::Application(_, _) => eval_with_variable_bindings(*lhs, bindings),
                _ => *lhs,
            };

            match lhs {
                Expression::Function(param, body) => {
                    // Perform β-reduction, i.e. apply the lhs with the rhs as the argument.
                    let prev = bindings.insert(param.clone(), rhs);
                    let result = eval_with_variable_bindings(*body, bindings);

                    // Restore the previous binding if there was one, otherwise clear it out.
                    if let Some(p) = prev {
                        bindings.insert(param.to_owned(), p);
                    } else {
                        bindings.remove(param.as_str());
                    }

                    // At this point we have performed β-reduction. It might still be that the result
                    // an application of a function, which could be further reduced. We therefore,
                    // recursively evaluate the result once more. Note that if a function application
                    // evaluates to itself, this would become an infinite loop. Evaluating such an
                    // expression would be an infinite computation, so crashing here seems reasonable.
                    // Could possibly try to detect this and just return the expression.
                    eval_with_variable_bindings(result, bindings)
                }
                _ => {
                    // The application is abstract, so can't be β-reduced. In this case we just
                    // evaluate the lhs as well and return the abstract application.
                    let lhs = eval_with_variable_bindings(lhs, bindings);
                    Expression::new_application(lhs, rhs)
                }
            }
        }
    }
}
