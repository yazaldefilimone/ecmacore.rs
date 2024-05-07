use super::Value;

// string in js/ts language
#[derive(Debug, Clone, PartialEq)]
pub enum StringValue {
  String(String),
}

impl StringValue {
  pub fn is_truthy(&self) -> bool {
    match self {
      StringValue::String(s) => !s.is_empty(),
    }
  }
  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn is_equal(&self, other: &Value) -> bool {
    match (self, other) {
      (StringValue::String(s), Value::String(StringValue::String(s2))) => s == s2,
      _ => false,
    }
  }
}
