use crate::error;
use crate::parser::expr::{Application, Expression, ExpressionKind, Function, Kind, Variable};

pub fn evaluate(expr: Expression) -> Result<Expression, error::LambdaError> {
    eval_with_variable_bindings(expr, None, None)
}

fn eval_with_variable_bindings(
    expr: Expression,
    binding_label: Option<String>,
    binding_expr: Option<Expression>,
) -> Result<Expression, error::LambdaError> {
    match expr.kind() {
        ExpressionKind::Variable => match binding_label {
            Some(label) if &label == expr.label() => Ok(binding_expr.unwrap()),
            _ => Ok(expr),
        },
        ExpressionKind::Function => Ok(expr),
        ExpressionKind::Application => {
            let lhs = eval_with_variable_bindings(
                (*expr.lhs().as_ref()).clone(),
                binding_label.clone(),
                binding_expr.clone(),
            )?;
            let rhs = eval_with_variable_bindings(
                (*expr.rhs().as_ref()).clone(),
                binding_label.clone(),
                binding_expr.clone(),
            )?;

            match lhs.kind() {
                ExpressionKind::Function => eval_with_variable_bindings(
                    (*lhs.body().as_ref()).clone(),
                    Some(lhs.param().to_string()),
                    Some(rhs),
                ),
                _ => {
                    // Application is abstract and cannot be further evaluated.
                    Ok(Expression::new_application(lhs, rhs))
                }
            }
        }
    }
}
