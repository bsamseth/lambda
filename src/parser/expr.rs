use std::fmt;

#[derive(Debug)]
pub enum ExpressionKind {
    Variable,
    Function,
    Application,
}

#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,

    // If this is a variable:
    label: Option<String>,

    // If this is a function:
    params: Option<Vec<String>>,
    body: Option<Box<Expression>>,

    // If this is an application:
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

impl Expression {
    pub fn new_variable(label: &str) -> Self {
        Self {
            kind: ExpressionKind::Variable,
            label: Some(String::from(label)),
            params: None,
            body: None,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_function(params: Vec<String>, body: Expression) -> Self {
        Self {
            kind: ExpressionKind::Function,
            label: None,
            params: Some(params),
            body: Some(Box::new(body)),
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_application(lhs: Expression, rhs: Expression) -> Self {
        Self {
            kind: ExpressionKind::Application,
            label: None,
            params: None,
            body: None,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ExpressionKind::Variable => write!(f, "{}", self.label.as_ref().unwrap()),
            ExpressionKind::Function => write!(
                f,
                "λ{}.{}",
                self.params.as_ref().unwrap().join(".λ"),
                self.body.as_ref().unwrap()
            ),
            ExpressionKind::Application => {
                let &lhs = &self.lhs.as_ref().unwrap();
                let &rhs = &self.rhs.as_ref().unwrap();
                match lhs.kind {
                    ExpressionKind::Variable => match rhs.kind {
                        ExpressionKind::Variable => write!(f, "{} {}", lhs, rhs),
                        _ => write!(f, "{} ({})", lhs, rhs),
                    },
                    _ => match rhs.kind {
                        ExpressionKind::Variable => write!(f, "({}) {}", lhs, rhs),
                        _ => write!(f, "({}) ({})", lhs, rhs),
                    },
                }
            }
        }
    }
}

// impl fmt::Debug for Expression {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", &self)
//     }
// }
