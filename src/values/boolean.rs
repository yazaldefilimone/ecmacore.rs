use super::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum BooleanValue {
  True,
  False,
}

impl BooleanValue {
  pub fn is_truthy(&self) -> bool {
    match self {
      BooleanValue::True => true,
      BooleanValue::False => false,
    }
  }
  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn is_equal(&self, other: &Value) -> bool {
    match (self, other) {
      (BooleanValue::True, Value::Boolean(BooleanValue::True)) => true,
      (BooleanValue::False, Value::Boolean(BooleanValue::False)) => true,
      _ => false,
    }
  }
}
