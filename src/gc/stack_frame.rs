/*

Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>

*/

use crate::gc::GCValue;
use std::collections::HashSet;

#[derive(Debug)]
pub struct StackFrame {
  locals: Vec<GCValue>,
}

impl StackFrame {
  pub fn new() -> Self {
    StackFrame { locals: Vec::new() }
  }

  pub fn add_local(&mut self, value: GCValue) {
    self.locals.push(value);
  }

  pub fn get_roots(&self) -> HashSet<usize> {
    let mut roots = HashSet::new();
    for local in &self.locals {
      if let GCValue::Reference(id) = local {
        roots.insert(*id);
      }
    }
    roots
  }
}
