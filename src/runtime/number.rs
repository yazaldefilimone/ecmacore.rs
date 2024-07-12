/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
*/

#[derive(Debug)]
pub struct NumberValue {
  value: f64,
}

impl NumberValue {
  pub fn new(value: f64) -> Self {
    NumberValue { value }
  }
}
