/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
*/

#[derive(Debug)]
pub struct BooleanValue {
  value: bool,
}

impl BooleanValue {
  pub fn new(value: bool) -> Self {
    BooleanValue { value }
  }
}
