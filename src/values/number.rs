use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
  Infinity,     // Infinity
  NaN,          // NaN
  Float(f64),   // 1.0;
  Integer(i64), // 1;
  Binary(i64),  // 0b1;
  Octal(i64),   // 0o1;
  Hex(i64),     // 0x1;
}

impl NumberValue {
  pub fn is_truthy(&self) -> bool {
    match self {
      NumberValue::Infinity => true,
      NumberValue::NaN => false,
      NumberValue::Float(f) => *f != 0.0,
      NumberValue::Integer(i) => *i != 0,
      NumberValue::Binary(i) => *i != 0,
      NumberValue::Octal(i) => *i != 0,
      NumberValue::Hex(i) => *i != 0,
    }
  }
  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn is_equal(&self, other: &Value) -> bool {
    match (self, other) {
      (NumberValue::Infinity, Value::Number(NumberValue::Infinity)) => true,
      (NumberValue::NaN, Value::Number(NumberValue::NaN)) => true,
      (NumberValue::Float(f), Value::Number(NumberValue::Float(f2))) => f == f2,
      (NumberValue::Integer(i), Value::Number(NumberValue::Integer(i2))) => i == i2,
      (NumberValue::Integer(i), Value::Number(NumberValue::Float(f))) => *i as f64 == *f,
      (NumberValue::Float(f), Value::Number(NumberValue::Integer(i))) => *f == *i as f64,
      (NumberValue::Binary(i), Value::Number(NumberValue::Binary(i2))) => i == i2,
      (NumberValue::Octal(i), Value::Number(NumberValue::Octal(i2))) => i == i2,
      (NumberValue::Hex(i), Value::Number(NumberValue::Hex(i2))) => i == i2,
      _ => false,
    }
  }
}
