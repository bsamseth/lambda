pub mod church {
    use crate::parser::expr::Expression;

    pub const ZERO: &str = "λf.λx.x";
    pub const ONE: &str = "λf.λx.f x";
    pub const TWO: &str = "λf.λx.f (f x)";
    pub const THREE: &str = "λf.λx.f (f (f x))";
    pub const SUCC: &str = "λn.λf.λx.f (n f x)";
    pub const ADD: &str = "λm.λn.λf.λx.m f (n f x)";
    pub const MUL: &str = "λm.λn.λf.m (n f)";

    pub fn zero() -> Expression {
        ZERO.parse().unwrap()
    }
    pub fn one() -> Expression {
        ONE.parse().unwrap()
    }
    pub fn two() -> Expression {
        TWO.parse().unwrap()
    }
    pub fn three() -> Expression {
        THREE.parse().unwrap()
    }
    pub fn succ() -> Expression {
        SUCC.parse().unwrap()
    }
    pub fn add() -> Expression {
        ADD.parse().unwrap()
    }
    pub fn mul() -> Expression {
        MUL.parse().unwrap()
    }
}
