//  todo: improve the value types
#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub enum Value {
  Number(f64),
  // Float(f64),
  String(String),
  Boolean(bool),
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

  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Boolean(b) => *b,
      Value::Number(n) => *n != 0.0,
      Value::String(s) => !s.is_empty(),
      Value::EOL => false,
      // Value::Float(f) => *f != 0.0,
      // _ => false,
    }
  }
  pub fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
  pub fn get_number(&self) -> f64 {
    match self {
      Value::Number(n) => *n,
      _ => 0.0,
    }
  }
  // pub fn is_float(&self) -> bool {
  //   match self {
  //     Value::Float(_) => true,
  //     _ => false,
  //   }
  // }
  // pub fn get_float(&self) -> f32 {
  //   match self {
  //     Value::Float(f) => *f,
  //     _ => 0.0,
  //   }
  // }

  pub fn is_string(&self) -> bool {
    match self {
      Value::String(_) => true,
      _ => false,
    }
  }
  pub fn get_string(&self) -> String {
    match self {
      Value::String(s) => s.clone(),
      _ => String::new(),
    }
  }

  pub fn is_boolean(&self) -> bool {
    match self {
      Value::Boolean(_) => true,
      _ => false,
    }
  }
  pub fn get_boolean(&self) -> bool {
    match self {
      Value::Boolean(b) => *b,
      _ => false,
    }
  }
}
