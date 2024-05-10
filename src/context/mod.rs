#![allow(dead_code)]
use std::rc;

use crate::{utils::is_internal_variable, values::Value};
#[derive(Debug, Clone, PartialEq)]
pub enum StoreKind {
  Const,
  Let,
  Var,
}
pub struct Store {
  pub name: String,
  pub value: Value,
  pub kind: StoreKind,
  pub level: usize,
}

pub struct Context {
  global: Vec<Store>,
  local: Vec<Store>,
  current_scope: usize,
}

impl Default for Context {
  fn default() -> Self {
    let global = vec![
      Store { name: "undefined".to_string(), value: Value::new_undefined(), level: 0, kind: StoreKind::Const },
      // Store { name: "undefined".to_string(), value: Value::Undefined },
      // Store { name: "NaN".to_string(), value: Value::NaN },
      // Store { name: "Infinity".to_string(), value: Value::Infinity },
      // Store { name: "Object".to_string(), value: Value::Object },
      // Store { name: "Function".to_string(), value: Value::Function },
      // Store { name: "Array".to_string(), value: Value::Array },
      // Store { name: "String".to_string(), value: Value::String("".to_string()) },
      // Store { name: "Number".to_string(), value: Value::Number(0.0) },
      // Store { name: "Boolean".to_string(), value: Value::Boolean(false) },
      // Store { name: "Math".to_string(), value: Value::Object },
      // Store { name: "Date".to_string(), value: Value::Object },
      // Store { name: "RegExp".to_string(), value: Value::Object },
      // Store { name: "Error".to_string(), value: Value::Object },
      // Store { name: "console".to_string(), value: Value::Undefined },
    ];
    Self { global, current_scope: 0, local: vec![] }
  }
}

impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_variable(&mut self, index: usize, value: Value) {
    if index >= self.local.len() && index >= self.global.len() {
      panic!("[Context]: {} is not exist", index);
    }

    if self.is_global_scope() {
      self.local[index].value = value;
      return;
    }
    self.local[index].value = value;
  }

  pub fn set_local(&mut self, index: usize, value: Value) {
    if index >= self.local.len() {
      panic!("[Context]: {} is not exist", index);
    }
    self.local[index].value = value
  }
  pub fn get_variable(&self, index: usize) -> &Store {
    if self.is_global_scope() {
      return &self.global[index];
    }
    return &self.local[index];
  }
  pub fn get_variable_name(&self, index: usize) -> &String {
    if self.is_global_scope() {
      return &self.global[index].name;
    }
    return &self.local[index].name;
  }
  pub fn get_variable_value(&self, index: usize) -> &Value {
    if self.is_global_scope() {
      return &self.global[index].value;
    }
    return &self.local[index].value;
  }
  pub fn is_exist_variable(&self, name: &str) -> bool {
    if self.is_global_scope() {
      return self.global.iter().any(|s| s.name == name);
    }
    let result = self.local.iter().any(|s| s.name == name
    if !result {
      self.global.iter().any(|s| s.name == name)
    } else {
      result
    }
  }
  pub fn get_kind_variable(&self, name: &str) -> Option<StoreKind> {
    if self.is_global_scope() {
      return self.global.iter().find(|s| s.name == name).map(|s| s.kind.clone());
    }
    let result = self.local.iter().find(|s| s.name == name).map(|s| s.kind.clone());

    if result.is_none() {
      return self.global.iter().find(|s| s.name == name).map(|s| s.kind.clone());
    }
    result
  }
  pub fn is_internal(&self, name: &str) -> bool {
    is_internal_variable(name)
  }
  pub fn get_variable_index(&self, name: &str) -> Option<usize> {
    if self.is_global_scope() {
      return self.global.iter().position(|s| s.name == name);
    }
    let result = self.local.iter().position(|s| s.name == name);
    if result.is_none() {
      return self.global.iter().position(|s| s.name == name);
    }
    result
  }

  pub fn enter_scope(&mut self) {
    self.current_scope += 1;
  }
  pub fn exit_scope(&mut self) {
    if self.current_scope == 1 {
      self.current_scope = 1;
      return;
    }
    self.current_scope -= 1;
  }
  pub fn is_global_scope(&self) -> bool {
    self.current_scope == 0
  }
  pub fn get_current_scope(&self) -> usize {
    self.current_scope
  }
  pub fn define_variable(&mut self, name: String, value: Option<Value>, kind: StoreKind) -> usize {
    if self.is_global_scope() {
      let index = match self.get_variable_index(&name) {
        Some(index) => index,
        None => {
          // register new variable by default is undefined
          self.global.push(Store {
            name,
            kind,
            level: self.current_scope,
            value: match value {
              Some(v) => v,
              None => Value::new_undefined(),
            },
          });
          self.global.len() - 1
        }
      };
      return index;
    }

    let index = match self.get_variable_index(&name) {
      Some(index) => index,
      None => {
        // register new variable by default is undefined
        self.local.push(Store {
          name,
          kind,
          level: self.current_scope,
          value: match value {
            Some(v) => v,
            None => Value::new_undefined(),
          },
        });
        self.local.len() - 1
      }
    };
    return index;
  }
}
