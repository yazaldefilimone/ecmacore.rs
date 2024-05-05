#![allow(dead_code)]
use crate::{utils::is_global_variable, values::Value};
pub struct Store {
  name: String,
  value: Value,
}
pub struct Context {
  store: Vec<Store>,
}

impl Default for Context {
  fn default() -> Self {
    let store = vec![
      // Store { name: "globalThis".to_string(), value: Value::Undefined },
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
    Self { store }
  }
}

impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_variable(&mut self, index: usize, value: Value) {
    if index >= self.store.len() {
      panic!("[Context]: {} is not exist", index);
    }
    self.store[index].value = value;
  }

  pub fn get_variable(&self, index: usize) -> &Store {
    &self.store[index]
  }
  pub fn get_variable_name(&self, index: usize) -> &String {
    &self.store[index].name
  }
  pub fn get_variable_value(&self, index: usize) -> &Value {
    &self.store[index].value
  }
  pub fn is_exist_variable(&self, name: &str) -> bool {
    self.store.iter().any(|s| s.name == name)
  }
  pub fn is_global_variable(&self, name: &str) -> bool {
    is_global_variable(name)
  }
  pub fn get_variable_index(&self, name: &str) -> Option<usize> {
    self.store.iter().position(|s| s.name == name)
  }

  pub fn define_variable(&mut self, name: String, value: Option<Value>) -> usize {
    match self.get_variable_index(&name) {
      Some(index) => index,
      None => {
        // register new variable by default is undefined
        self.store.push(Store {
          name,
          value: match value {
            Some(v) => v,
            None => Value::Undefined,
          },
        });
        self.store.len() - 1
      }
    }
  }
}
