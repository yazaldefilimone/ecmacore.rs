use oxc_allocator::Allocator;
use oxc_span::SourceType;
pub mod compiler;
use compiler::Compiler;

use crate::context::Context;

use self::compiler::CompilerReturn;

pub fn compile(arena_allocator: &Allocator, source: &String, ctx: &mut Context) -> CompilerReturn {
  let source_type = SourceType::default().with_module(true).with_typescript(true);
  let parser = oxc_parser::Parser::new(&arena_allocator, source, source_type);
  let result = parser.parse();
  Compiler::compile(&result.program, ctx)
}
