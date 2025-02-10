use std::fmt;

#[derive(Debug, Clone)]
pub struct PostError;

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while creating post")
    }
}
