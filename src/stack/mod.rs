#![allow(dead_code)]
use crate::errors::EngineError;
use crate::values::Value;

pub struct Stack {
  stack: Vec<Value>,
  scope_counter: usize,
}

impl Stack {
  pub fn new(capacity: usize) -> Self {
    Self { stack: Vec::with_capacity(capacity), scope_counter: 0 }
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Result<Value, EngineError> {
    self.stack.pop().ok_or(EngineError::StackUnderflow)
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
  pub fn is_global_scope(&self) -> bool {
    self.scope_counter == 1
  }
  pub fn enter_scope(&mut self) {
    self.scope_counter += 1;
  }
  pub fn exit_scope(&mut self) {
    self.scope_counter -= 1;
  }
  pub fn push_in_global_scope(&mut self, value: Value, frame: usize) {
    // set in global scope only when the scope counter larger than 1
    if !self.is_global_scope() {
      self.stack.insert(frame, value);
      return;
    }
    self.push(value);
  }
}
