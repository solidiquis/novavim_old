use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    CharNotFound,
    EmptyLine,
    EndOfText,
    PatternNotFound,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CharNotFound => write!(f, "Char not found."),
            Error::EmptyLine => write!(f, "Current line is blank."),
            Error::EndOfText => write!(f, "Reach end of text."),
            Error::PatternNotFound => write!(f, "Pattern not found."),
        }
    }
}
