/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
@links:
*/
#[derive(Debug)]
pub struct BigIntValue {
  // todo: check if i128 is correct in t39 spec
  value: i128,
}

impl BigIntValue {
  pub fn new(value: i128) -> Self {
    BigIntValue { value }
  }
}
