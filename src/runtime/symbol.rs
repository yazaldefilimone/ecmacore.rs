/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*

@links:
-
*/
#[derive(Debug)]
pub struct SymbolValue {
  // todo: consider using a better type for Symbol (tips. check in t39 spec or jscore ...)
  value: String,
}

impl SymbolValue {
  pub fn new(value: String) -> Self {
    SymbolValue { value }
  }
}
