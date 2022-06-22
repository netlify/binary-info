use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InfoError {
    details: String,
}

impl InfoError {
    pub fn new(msg: impl ToString) -> InfoError {
        InfoError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InfoError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<goblin::error::Error> for InfoError {
    fn from(err: goblin::error::Error) -> Self {
        InfoError::new(err)
    }
}
