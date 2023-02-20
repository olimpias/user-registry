use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    NotFound,
}

impl Error for UserError {
    
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        NotFound => write!(f, "unable to find user"),
      }
    }
  }