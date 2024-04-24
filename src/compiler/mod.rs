use oxc_allocator::Allocator;
use oxc_span::SourceType;

use crate::{tokens::Program, transpiler::Transpiler};

pub fn compiler(arena_allocator: &Allocator, source: &String) -> () {
  let parser = oxc_parser::Parser::new(&arena_allocator, source, SourceType::default());
  let result = parser.parse();
  let program = Transpiler::transpile(&result.program);
  program
}
