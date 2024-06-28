/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
*/
#[derive(Debug)]
pub struct StringValue {
  value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self {
    StringValue { value }
  }
}
