#![allow(dead_code)]
use crate::{utils::is_internal_variable, values::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  Const,
  Let,
  Var,
}

pub struct Store {
  pub name: String,
  pub value: Value,
  pub kind: Kind,
  pub level: usize,
}

pub struct Context {
  global: Vec<Store>,
  local: Vec<Store>,
  current_scope: usize,
}

impl Default for Context {
  fn default() -> Self {
    let global =
      vec![Store { name: "undefined".to_string(), value: Value::new_undefined(), level: 0, kind: Kind::Const }];
    Self { global, current_scope: 0, local: vec![] }
  }
}

impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_variable(&mut self, index: usize, value: Value) {
    if self.is_global_scope() && index >= self.global.len() {
      panic!("[Context]: {} does not exist in global scope.", index);
    }
    if self.is_global_scope() {
      return self.set_global_variable(index, value);
    }

    if index >= self.local.len() {
      panic!("[Context]: {} does not exist in local scope.", index);
    }
    return self.set_local_variable(index, value);
  }

  fn set_global_variable(&mut self, index: usize, value: Value) {
    self.global[index].value = value;
  }
  fn set_local_variable(&mut self, index: usize, value: Value) {
    // todo: validate index in local  scope
    if index >= self.local.len() && self.local[index].level != self.get_current_scope() {
      self.set_global_variable(index, value);
      return;
    }

    self.local[index].value = value;
  }

  pub fn set_local(&mut self, index: usize, value: Value) {
    if index >= self.local.len() {
      panic!("[Context]: {} does not exist", index);
    }
    self.local[index].value = value;
  }

  pub fn deallocate_variable_in_scope(&mut self) -> usize {
    let mut count = 0;
    let total = self.local.len();
    for index in (0..total).rev() {
      if self.local[index].level == self.current_scope {
        self.local.pop();
        count += 1;
      }
    }
    count
  }

  pub fn get_variable_name(&self, index: usize) -> &String {
    &self.get_variable(index).name
  }

  pub fn get_variable_value(&self, index: usize) -> &Value {
    &self.get_variable(index).value
  }

  pub fn get_global_variable(&self, index: usize) -> Option<&Store> {
    if index > self.global.len() {
      return None;
    }
    return Some(&self.global[index]);
  }
  pub fn get_local_variable(&self, index: usize) -> Option<&Store> {
    if index > self.local.len() {
      return None;
    }
    return Some(&self.local[index]);
  }

  pub fn get_variable(&self, index: usize) -> &Store {
    if self.is_global_scope() {
      return self.get_global_variable(index).unwrap();
    }
    let local = self.get_local_variable(index);
    if let Some(var) = local {
      return var;
    }
    return self.get_global_variable(index).unwrap();
  }

  pub fn is_exist_variable(&self, name: &str) -> bool {
    if self.is_global_scope() {
      self.global.iter().any(|s| s.name == name)
    } else {
      self.local.iter().any(|s| s.name == name)
    }
  }

  pub fn get_kind_variable(&self, name: &str) -> Option<Kind> {
    if self.is_global_scope() {
      self.global.iter().find(|s| s.name == name).map(|s| s.kind.clone())
    } else {
      self.local.iter().find(|s| s.name == name).map(|s| s.kind.clone())
    }
  }

  pub fn is_internal(&self, name: &str) -> bool {
    is_internal_variable(name)
  }

  pub fn get_current_scope(&self) -> usize {
    self.current_scope
  }

  pub fn get_variable_index(&self, name: &str) -> Option<usize> {
    if self.is_global_scope() {
      self.global.iter().position(|s| s.name == name)
    } else {
      self
        .local
        .iter()
        .position(|s| s.name == name && s.level == self.current_scope)
        .or_else(|| self.global.iter().position(|s| s.name == name))
    }
  }

  pub fn enter_scope(&mut self) {
    self.current_scope += 1;
  }

  pub fn exit_scope(&mut self) {
    if self.current_scope > 0 {
      self.current_scope -= 1;
    }
  }

  pub fn is_global_scope(&self) -> bool {
    self.current_scope == 0
  }

  pub fn define_variable(&mut self, name: String, value: Option<Value>, kind: Kind) -> usize {
    if self.is_global_scope() {
      self.define_global_variable(name, value, kind)
    } else {
      self.define_local_variable(name, value, kind)
    }
  }

  fn define_global_variable(&mut self, name: String, value: Option<Value>, kind: Kind) -> usize {
    if let Some(index) = self.get_variable_index(&name) {
      index
    } else {
      self.global.push(Store {
        name,
        kind,
        level: self.get_current_scope(),
        value: value.unwrap_or_else(Value::new_undefined),
      });
      self.global.len() - 1
    }
  }

  fn define_local_variable(&mut self, name: String, value: Option<Value>, kind: Kind) -> usize {
    if let Some(index) = self.get_variable_index(&name) {
      index
    } else {
      self.local.push(Store {
        name,
        kind,
        level: self.get_current_scope(),
        value: value.unwrap_or_else(Value::new_undefined),
      });
      self.local.len() - 1
    }
  }
}
