use oxc_allocator::Allocator;
use oxc_span::SourceType;

use crate::{tokens::Program, transpiler::Transpiler};

pub fn compiler(arena_allocator: &Allocator, source: &String) -> Program {
  let parser = oxc_parser::Parser::new(&arena_allocator, source, SourceType::default());
  let result = parser.parse();
  Transpiler::transpile(&result.program)
}
