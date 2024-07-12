use super::Agent;
use super::SymbolValue;

pub const MATCH_ALL_SYMBOL: &str = "Symbol.matchAll";
pub const MATCH_SYMBOL: &str = "Symbol.match";
pub const REPLACE_SYMBOL: &str = "Symbol.replace";
pub const SEARCH_SYMBOL: &str = "Symbol.search";
pub const SPLIT_SYMBOL: &str = "Symbol.split";
pub const TO_PRIMITIVE_SYMBOL: &str = "Symbol.toPrimitive";
pub const TO_STRING_TAG_SYMBOL: &str = "Symbol.toStringTag";
pub const UNSCOPABLES_SYMBOL: &str = "Symbol.unscopables";
pub const MATCH_ALL_METHOD: &str = "Symbol.prototype.matchAll";
pub const MATCH_METHOD: &str = "Symbol.prototype.match";
pub const REPLACE_METHOD: &str = "Symbol.prototype.replace";
pub const SEARCH_METHOD: &str = "Symbol.prototype.search";
pub const SPLIT_METHOD: &str = "Symbol.prototype.split";
pub const TO_PRIMITIVE_METHOD: &str = "Symbol.prototype.toPrimitive";
pub const TO_STRING_TAG_METHOD: &str = "Symbol.prototype.toStringTag";
pub const UNSCOPABLES_METHOD: &str = "Symbol.prototype.unscopables";
pub const ITERATOR_SYMBOL: &str = "Symbol.iterator";
pub const HAS_INSTANCE_SYMBOL: &str = "Symbol.hasInstance";
pub const IS_CONCAT_SPREADABLE_SYMBOL: &str = "Symbol.isConcatSpreadable";
pub const SPECIES_SYMBOL: &str = "Symbol.species";

pub struct WellKnownSymbol {
  pub asyncIterator: SymbolValue,
  pub hasInstance: SymbolValue,
  pub isConcatSpreadable: SymbolValue,
  pub iterator: SymbolValue,
  pub match_: SymbolValue,
  pub matchAll: SymbolValue,
  pub replace: SymbolValue,
  pub search: SymbolValue,
  pub species: SymbolValue,
  pub split: SymbolValue,
  pub toPrimitive: SymbolValue,
  pub toStringTag: SymbolValue,
  pub unscopables: SymbolValue,
}

impl WellKnownSymbol {
  pub fn init(agent: &Agent) -> Self {
    WellKnownSymbol {
      asyncIterator: agent.create_symbol(ITERATOR_SYMBOL),
      hasInstance: agent.create_symbol(HAS_INSTANCE_SYMBOL),
      isConcatSpreadable: agent.create_symbol(IS_CONCAT_SPREADABLE_SYMBOL),
      iterator: agent.create_symbol(ITERATOR_SYMBOL),
      match_: agent.create_symbol(MATCH_SYMBOL),
      matchAll: agent.create_symbol(MATCH_ALL_SYMBOL),
      replace: agent.create_symbol(REPLACE_SYMBOL),
      search: agent.create_symbol(SEARCH_SYMBOL),
      species: agent.create_symbol(SPECIES_SYMBOL),
      split: agent.create_symbol(SPLIT_SYMBOL),
      toPrimitive: agent.create_symbol(TO_PRIMITIVE_SYMBOL),
      toStringTag: agent.create_symbol(TO_STRING_TAG_SYMBOL),
      unscopables: agent.create_symbol(UNSCOPABLES_SYMBOL),
    }
  }
}
