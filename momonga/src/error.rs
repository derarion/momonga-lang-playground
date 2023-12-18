use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    Parser(ParseError),
    Eval(EvalError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(err) => write!(f, "{}", err),
            Error::Eval(err) => write!(f, "{}", err),
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

impl From<EvalError> for Error {
    fn from(e: EvalError) -> Self {
        Error::Eval(e)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    PestParser,
    BuildAst,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::PestParser => write!(f, "Syntax error"),
            ParseError::BuildAst => write!(f, "Syntax error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    Argument,
    Index,
    InvalidExpression,
    Name,
    OutOfRange,
    Type,
    ZeroDivision,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::Argument => write!(f, "Argument error"),
            EvalError::Index => write!(f, "Index error"),
            EvalError::InvalidExpression => write!(f, "Invalid expression error"),
            EvalError::Name => write!(f, "Name error"),
            EvalError::OutOfRange => write!(f, "Out of range error"),
            EvalError::Type => write!(f, "Type error"),
            EvalError::ZeroDivision => write!(f, "Zero division error"),
        }
    }
}
