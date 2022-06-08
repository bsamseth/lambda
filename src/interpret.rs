use crate::error;
use crate::parser::expr::{Application, Expression, ExpressionKind, Kind};

pub fn evaluate(expr: &Expression) -> Result<&Expression, error::LambdaError> {
    match expr.kind() {
        ExpressionKind::Variable => Ok(expr),
        ExpressionKind::Function => Ok(expr),
        ExpressionKind::Application => {
            let lhs = evaluate(&expr.lhs())?;
            let _rhs = evaluate(&expr.rhs())?;
            Ok(lhs)
        }
    }
}
