use std::fmt;

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Variable,
    Function,
    Application,
}

pub trait Kind {
    fn kind(&self) -> &ExpressionKind;
}
pub trait Variable: Kind {
    fn label(&self) -> &String;
}
pub trait Function: Kind {
    fn param(&self) -> &String;
    fn body(&self) -> &Box<Expression>;
}
pub trait Application: Kind {
    fn lhs(&self) -> &Box<Expression>;
    fn rhs(&self) -> &Box<Expression>;
}

#[derive(Debug, Clone)]
pub struct Expression {
    kind: ExpressionKind,

    // If this is a variable:
    label: Option<String>,

    // If this is a function:
    param: Option<String>,
    body: Option<Box<Expression>>,

    // If this is an application:
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

impl Kind for Expression {
    fn kind(&self) -> &ExpressionKind {
        &self.kind
    }
}

impl Variable for Expression {
    fn label(&self) -> &String {
        self.label.as_ref().unwrap()
    }
}

impl Function for Expression {
    fn param(&self) -> &String {
        self.param.as_ref().unwrap()
    }

    fn body(&self) -> &Box<Expression> {
        self.body.as_ref().unwrap()
    }
}

impl Application for Expression {
    fn lhs(&self) -> &Box<Expression> {
        self.lhs.as_ref().unwrap()
    }

    fn rhs(&self) -> &Box<Expression> {
        self.rhs.as_ref().unwrap()
    }
}

impl Expression {
    pub fn new_variable(label: &str) -> Self {
        Self {
            kind: ExpressionKind::Variable,
            label: Some(String::from(label)),
            param: None,
            body: None,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_function(param: String, body: Expression) -> Self {
        Self {
            kind: ExpressionKind::Function,
            label: None,
            param: Some(param),
            body: Some(Box::new(body)),
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_application(lhs: Expression, rhs: Expression) -> Self {
        Self {
            kind: ExpressionKind::Application,
            label: None,
            param: None,
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
                self.param.as_ref().unwrap(),
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
