use super::Agent;

pub struct Realm {
  pub agent: Agent,
}

impl Realm {
  pub fn new() -> Self {
    Realm { agent: Agent::new() }
  }
}
