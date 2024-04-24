use crate::{
  bytecode::opcode,
  context::Context,
  stack::Stack,
  tokens::{Program, Token},
  utils::STACK_LIMIT,
  values::Value,
};

pub struct Engine<'ctx> {
  ctx: &'ctx mut Context,
  code: Vec<usize>,
  constants: Vec<Token>,
  stack: &'ctx mut Stack,
  instruction_pointer: usize,
  stack_pointer: usize,
}

impl<'ctx> Engine<'ctx> {
  pub fn new(ctx: &'ctx mut Context, stack: &'ctx mut Stack) -> Self {
    //  return VM with 'ctx
    Self { ctx, code: Vec::new(), constants: Vec::new(), stack, instruction_pointer: 0, stack_pointer: 0 }
  }
  pub fn bootstrap(ctx: &'ctx mut Context, program: &Program) -> Value {
    let mut stack = Stack::new(STACK_LIMIT);
    let vm = Engine::new(ctx, &mut stack);
    vm.run(program)
  }

  fn run(mut self, program: &Program) -> Value {
    loop {
      let instruction = self.read();
      match instruction {
        opcode::OPCODE_CONST => return Value::EOL,
        // opcode::OPCODE_CONST => {
        //   let index = self.read();
        //   let constant = self.constants[index].clone();
        //   self.stack.push(Value::from_token(constant));
        // }
        _ => todo!(),
      }
    }
  }

  fn read(&mut self) -> usize {
    let instruction = self.code[self.instruction_pointer];
    self.instruction_pointer += 1;
    instruction
  }
}
