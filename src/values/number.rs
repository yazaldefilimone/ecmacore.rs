/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
*/

#[derive(Debug)]
pub struct NumberValue {
  value: i64,
}

impl NumberValue {
  pub fn new(value: i64) -> Self {
    NumberValue { value }
  }
}
