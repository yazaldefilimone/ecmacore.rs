#![allow(dead_code)]
pub mod boolean;
pub mod number;
pub mod object;
pub mod string;
use boolean::BooleanValue;
use number::NumberValue;
use object::ObjectValue;
use string::StringValue;
#[derive(Debug, Clone)]
pub enum Value {
  Number(NumberValue),
  String(StringValue),
  Boolean(BooleanValue),
  Object(ObjectValue),
  Undefined,
  EOL,
}
#[allow(dead_code)]
impl Value {
  pub fn is_number(&self) -> bool {
    match self {
      Value::Number(_) => true,
      _ => false,
    }
  }
  pub fn is_equal(&self, other: &Value) -> bool {
    match self {
      Value::Number(n) => n.is_equal(other),
      Value::String(s) => s.is_equal(other),
      Value::Boolean(b) => b.is_equal(other),
      Value::Object(o) => o.is_equal(other),
      _ => false,
    }
  }
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Number(n) => n.is_truthy(),
      Value::String(s) => s.is_truthy(),
      Value::Boolean(b) => b.is_truthy(),
      Value::Object(o) => o.is_truthy(),
      Value::Undefined => false,
      _ => false,
    }
  }

  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn get_integer(&self) -> i64 {
    match self {
      Value::Number(num) => match &num {
        NumberValue::Integer(i) => *i,
        _ => 0,
      },
      _ => panic!("Value is not a number"),
    }
  }
  pub fn is_integer(&self) -> bool {
    if let Value::Number(NumberValue::Integer(_)) = self {
      return true;
    }
    return false;
  }
  pub fn get_float(&self) -> f64 {
    if let Value::Number(NumberValue::Float(f)) = self {
      return *f;
    }
    panic!("Value is not a float")
  }

  pub fn is_float(&self) -> bool {
    if let Value::Number(NumberValue::Float(_)) = self {
      return true;
    }
    return false;
  }
  pub fn is_string(&self) -> bool {
    if let Value::String(_) = self {
      return true;
    }
    return false;
  }
  pub fn get_string(&self) -> String {
    if let Value::String(StringValue::String(s)) = self {
      return s.clone();
    }
    panic!("Value is not a string")
  }

  pub fn is_boolean(&self) -> bool {
    if let Value::Boolean(_) = self {
      return true;
    }
    return false;
  }
  pub fn get_boolean(&self) -> bool {
    if let Value::Boolean(value) = self {
      return value.is_truthy();
    }
    panic!("Value is not a boolean")
  }
  pub fn is_undefined(&self) -> bool {
    if let Value::Undefined = self {
      return true;
    }
    return false;
  }

  pub fn _is_same_type(&self, other: &Value) -> bool {
    match (self, other) {
      (Value::Number(_), Value::Number(_)) => true,
      (Value::String(_), Value::String(_)) => true,
      (Value::Boolean(_), Value::Boolean(_)) => true,
      (Value::Object(_), Value::Object(_)) => true,
      (Value::Undefined, Value::Undefined) => true,
      (Value::EOL, Value::EOL) => true,
      _ => false,
    }
  }

  // built-in functions
  pub fn new_integer(value: i64) -> Value {
    Value::Number(NumberValue::Integer(value))
  }
  pub fn new_string(value: String) -> Value {
    Value::String(StringValue::String(value))
  }
  pub fn new_boolean(value: bool) -> Value {
    if value {
      Value::Boolean(BooleanValue::True)
    } else {
      Value::Boolean(BooleanValue::False)
    }
  }
  pub fn new_undefined() -> Value {
    Value::Undefined
  }
  pub fn new_eol() -> Value {
    Value::EOL
  }
  pub fn new_float(value: f64) -> Value {
    Value::Number(NumberValue::Float(value))
  }
  pub fn new_null() -> Value {
    Value::Object(ObjectValue::Null)
  }
  pub fn new_hex(value: i64) -> Value {
    Value::Number(NumberValue::Hex(value))
  }
  pub fn new_octal(value: i64) -> Value {
    Value::Number(NumberValue::Octal(value))
  }
  pub fn new_binary(value: i64) -> Value {
    Value::Number(NumberValue::Binary(value))
  }
}
