use super::Value;

pub struct ArrayValue {
  pub values: Vec<Value>,
}

impl ArrayValue {
  pub fn new() -> Self {
    ArrayValue { values: Vec::new() }
  }
}
