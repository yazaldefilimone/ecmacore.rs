#![allow(dead_code)]

use super::Value;
#[derive(Debug, Clone)]
pub enum ObjectValue {
  Null,
}

impl ObjectValue {
  pub fn is_truthy(&self) -> bool {
    match self {
      ObjectValue::Null => false,
    }
  }
  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn is_equal(&self, other: &Value) -> bool {
    match (self, other) {
      (ObjectValue::Null, Value::Object(ObjectValue::Null)) => true,
      _ => false,
    }
  }
}
