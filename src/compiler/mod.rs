use oxc_allocator::Allocator;
use oxc_span::SourceType;
pub mod compiler;
use compiler::Compiler;

use self::compiler::CompilerReturn;

pub fn compile(arena_allocator: &Allocator, source: &String) -> CompilerReturn {
  let parser = oxc_parser::Parser::new(&arena_allocator, source, SourceType::default());
  let result = parser.parse();
  Compiler::compile(&result.program)
}
