//  todo: improve the value types
#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
  Number(f64),
  Float(f32),
  String(String),
  Boolean(bool),
  EOL,
}
