use crate::context::EngineTypes;
use crate::values::boolean::BooleanValue;
use crate::values::number::NumberValue;
use crate::values::object::ObjectValue;
use crate::values::string::StringValue;
use crate::values::Value;
use std::fmt;
use std::fmt::Display;

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Value::String(s) => write!(f, "{}", s),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Number(n) => write!(f, "{}", n),
      Value::Undefined => write!(f, "undefined"),
      Value::Object(obj) => write!(f, "{}", obj),
      Value::EOL => write!(f, "End of Line"),
      // _ => write!(f, "Unknown Value"),
    }
  }
}
impl Display for EngineTypes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      EngineTypes::String => write!(f, "string"),
      EngineTypes::Number => write!(f, "number"),
      EngineTypes::Undefined => write!(f, "undefined"),
      _ => write!(f, "Unknown Value"),
    }
  }
}
impl Display for NumberValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      NumberValue::Integer(i) => write!(f, "{}", i),
      NumberValue::Float(fl) => write!(f, "{}", fl),
      _ => write!(f, "Unknown Number"),
    }
  }
}
impl Display for BooleanValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      BooleanValue::True => write!(f, "Boolean(true)"),
      BooleanValue::False => write!(f, "Boolean(false)"),
    }
  }
}

impl Display for ObjectValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      // ObjectValue::Object(obj) => write!(f, "{}", obj),
      // ObjectValue::Array(arr) => write!(f, "{}", arr),
      // ObjectValue::Function(func) => write!(f, "{}", func),
      ObjectValue::Null => write!(f, "undefined"),
    }
  }
}

impl Display for StringValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      StringValue::String(s) => write!(f, "{}", s),
    }
  }
}
