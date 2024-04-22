use std::fmt;

pub struct GrepError {
    pub message: String
}

impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GrepError: {}", self.message)
    }
}

impl fmt::Debug for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GrepError(Debug): {}", self.message)
    }
}
