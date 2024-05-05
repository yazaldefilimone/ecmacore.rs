use std::env::args;
mod bytecode;
mod checker;
mod compiler;
mod context;
mod disassembler;
mod errors;
mod formatting;
mod parser;
mod stack;
mod utils;
mod values;
mod vm;
use vm::core;

fn main() {
  let command = args().nth(1);
  let flag = args().nth(2).unwrap_or(String::from(""));
  let source = if Some("run") != command.as_deref() {
    String::from("30 + 30")
  } else {
    let flag = args().nth(2).expect("no filename provided");
    let filename = if flag.starts_with("-") {
      args().nth(3).expect("no filename provided")
    } else {
      flag
    };
    std::fs::read_to_string(&filename).expect("could not read file")
  };
  let mut ctx = context::Context::new();
  let result = core::Engine::bootstrap(&mut ctx, &source, flag == "--debug");
  println!("{:?}", result);
}
