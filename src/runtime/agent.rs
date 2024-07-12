/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>

*/

//! 9.7 Agents
//! https://tc39.es/ecma262/#sec-agents

use super::{Realm, SymbolValue};

pub struct Agent {
  pub realm: Realm,
}

impl Agent {
  pub fn new() -> Self {
    Agent { realm: Realm::new() }
  }

  pub fn create_symbol(&self, name: &str) -> SymbolValue {
    SymbolValue::new(name.to_owned())
  }
}
