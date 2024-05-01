use crate::{
  bytecode::opcode,
  compiler::{compile, compiler::CompilerReturn},
  context::Context,
  stack::Stack,
  utils::STACK_LIMIT,
  values::Value,
};
#[allow(dead_code)]
pub struct Engine<'ctx> {
  ctx: &'ctx mut Context,
  compiler: CompilerReturn,
  stack: &'ctx mut Stack,
  instruction_pointer: usize,
}
#[allow(dead_code)]
impl<'ctx> Engine<'ctx> {
  pub fn new(ctx: &'ctx mut Context, stack: &'ctx mut Stack, compiler: CompilerReturn) -> Self {
    //  return VM with 'ctx
    Self { ctx, compiler, stack, instruction_pointer: 0 }
  }
  pub fn bootstrap(ctx: &'ctx mut Context, source: &String) -> Value {
    let arena_allocator = oxc_allocator::Allocator::default();
    let compiler = compile(&arena_allocator, source);
    let mut stack = Stack::new(STACK_LIMIT);
    let vm = Engine::new(ctx, &mut stack, compiler);
    vm.run()
  }

  fn run(mut self) -> Value {
    loop {
      let instruction = self.read();
      match instruction {
        opcode::OPCODE_CONST => {
          let index = self.get_constant();
          self.stack.push(index);
        }
        opcode::OPCODE_ADD => self._add_operation(),
        opcode::OPCODE_SUB => self._sub_operation(),
        opcode::OPCODE_MUL => self._mul_operation(),
        opcode::OPCODE_DIV => self._div_operation(),
        opcode::OPCODE_HALF => return self.stack.pop().unwrap(),
        _ => todo!(),
      }
    }
  }

  fn read(&mut self) -> usize {
    let instruction = self.compiler.code[self.instruction_pointer];
    self.instruction_pointer += 1;
    instruction
  }

  fn get_constant(&mut self) -> Value {
    let index = self.read();
    self.compiler.constants[index].clone()
  }
  fn _add_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_number() && right.is_number() {
      let result = Value::Number(left.get_number() + right.get_number());
      self.stack.push(result);
      return;
    }

    if left.is_string() && right.is_string() {
      let result = Value::String(left.get_string() + &right.get_string());
      self.stack.push(result);
      return;
    }
    panic!("Unsupported operation, left: {:?} + right: {:?}", left, right);
  }

  fn _sub_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_number() && right.is_number() {
      let result = Value::Number(left.get_number() - right.get_number());
      self.stack.push(result);
    }
    panic!("Unsupported operation, left: {:?} - right: {:?}", left, right);
  }

  fn _mul_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if right.is_number() && right.is_number() {
      let result = Value::Number(&left.get_number() * &right.get_number());
      self.stack.push(result);
    }
    panic!("Unsupported operation, left: {:?} * right: {:?}", left, right);
  }
  fn _div_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_number() && right.is_number() {
      let result = Value::Number(left.get_number() / right.get_number());
      self.stack.push(result);
    }
    panic!("Unsupported operation, left: {:?} / right: {:?}", left, right);
  }
}
