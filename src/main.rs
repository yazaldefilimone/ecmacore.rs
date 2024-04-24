use std::env::args;

use compiler::compiler;

mod bytecode;
mod compiler;
mod context;
mod errors;
mod formatting;
mod parser;
mod stack;
mod tokens;
mod transpiler;
mod utils;
mod values;
mod vm;
use vm::core;
// mod transpiler;

// use std::env::args;

// use transpiler::parse;
fn main() {
  let filename = args().nth(1).expect("no filename given");
  let source = std::fs::read_to_string(&filename).expect("could not read file");
  let arena_allocator = oxc_allocator::Allocator::default();
  let program = compiler(&arena_allocator, &source);

  let mut ctx = context::Context::new();
  let result = core::Engine::bootstrap(&mut ctx, &program);
  println!("{:?}", result);
}
