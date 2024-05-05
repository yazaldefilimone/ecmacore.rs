use crate::errors::EngineError;
use crate::values::Value;

pub struct Stack {
  stack: Vec<Value>,
}

impl Stack {
  pub fn new(capacity: usize) -> Self {
    Self { stack: Vec::with_capacity(capacity) }
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Result<Value, EngineError> {
    self.stack.pop().ok_or(EngineError::StackUnderflow)
  }

  pub fn get_last(&self) -> Result<&Value, EngineError> {
    self.stack.last().ok_or(EngineError::StackUnderflow)
  }
}
