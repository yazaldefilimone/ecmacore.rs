/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
6.1.1 The Undefined Type
The Undefined type has exactly one value, called undefined. Any variable that has not been assigned a value has the value undefined.

@links:
- https://tc39.es/ecma262/#sec-ecmascript-language-types-undefined-type
*/

#[derive(Debug)]
pub struct UndefinedValue;

impl UndefinedValue {
  pub fn new() -> Self {
    UndefinedValue
  }
}
