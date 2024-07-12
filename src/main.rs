/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>

*/

// modules
mod assembler;
mod bytecode;
mod cli;
mod compiler;
mod context;
mod diagnostics;
mod disassembler;
mod errors;
mod formatting;
mod gc;
mod parser;
mod regex;
mod runtime;
mod stack;
mod utils;
mod vm;
//  uses
use cli::command_line;
use vm::core;

fn run(source: String, is_debug: bool) {
  let mut ctx = context::Context::new();
  let result = core::Engine::bootstrap(&mut ctx, &source, is_debug);
  println!("{:?}", result);
}

fn main() {
  let matches = command_line();

  match matches.subcommand() {
    Some(("run", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let debug = matches.get_flag("debug");
      let source = std::fs::read_to_string(file).expect("could not read file");
      run(source, debug);
    }
    Some(("compile", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let debug = matches.get_flag("debug");
      let source = std::fs::read_to_string(file).expect("could not read file");
      run(source, debug);
    }
    _ => {
      panic!("Unknown command");
    }
  }
}
