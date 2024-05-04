use crate::values::Value;
use std::fmt;
use std::fmt::Display;

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Value::String(s) => write!(f, "{}", s),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Number(n) => write!(f, "{}", n),
      // Value::Float(f) => write!(f, "{}", f),
      Value::EOL => write!(f, "End of Line"),
      // _ => write!(f, "Unknown Value"),
    }
  }
}
