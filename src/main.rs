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
mod cli;
use cli::command_line;

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

fn run(source: String, is_debug: bool) {
  let mut ctx = context::Context::new();
  let result = core::Engine::bootstrap(&mut ctx, &source, is_debug);
  println!("{:?}", result);
}
