#![allow(dead_code)]
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
  #[inline(always)]
  pub fn pop_values(&mut self, count: usize) {
    for _ in 0..count {
      self.pop().unwrap();
    }
  }

  // peek(0) returns the top of the stack
  #[inline(always)]
  pub fn peek(&self, index: usize) -> Result<&Value, EngineError> {
    let index = self.stack.len() - index - 1;
    self.stack.get(index).ok_or(EngineError::StackUnderflow)
  }
  pub fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }
  pub fn push_in_global_scope(&mut self, value: Value, frame: usize) {
    self.stack.insert(frame, value);
  }
}
