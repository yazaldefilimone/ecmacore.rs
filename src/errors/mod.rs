#![allow(dead_code)]
use core::fmt;
use std::error::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum EngineError {
  StackUnderflow,
  Other(Box<dyn Error + 'static>),
}

pub fn other<E: Error + 'static>(e: E) -> EngineError {
  EngineError::Other(Box::new(e))
}

pub type Result<T> = std::result::Result<T, EngineError>;

impl fmt::Display for EngineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      EngineError::StackUnderflow => write!(f, "Stack Underflow"),
      EngineError::Other(e) => write!(f, "{e}"),
    }
  }
}
