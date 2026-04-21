use std::fmt;

#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    #[allow(dead_code)]
    Overflow,
    InvalidArgument,
    #[allow(dead_code)]
    Interrupted,
    InvalidToken,
    InvalidParse,
    InvalidAssignment,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::Overflow => write!(f, "arithmetic overflow"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::Interrupted => write!(f, "computation interrupted"),
            Self::InvalidToken => write!(f, "invalid token"),
            Self::InvalidParse => write!(f, "invalid parse"),
            Self::InvalidAssignment => write!(f, "invalid assignment"),
        }
    }
}

impl std::error::Error for CalcError {}
