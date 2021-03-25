use crate::model;
use std::error;
use std::fmt;

impl fmt::Display for model::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Error")
    }
}

impl error::Error for model::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
