use crate::{
  bytecode::opcode,
  compiler::{compile, compiler::CompilerReturn},
  context::Context,
  disassembler::Disassembler,
  stack::Stack,
  utils::STACK_LIMIT,
  values::Value,
};
#[allow(dead_code)]
pub struct Engine<'ctx> {
  ctx: &'ctx mut Context,
  compiler: &'ctx CompilerReturn,
  stack: &'ctx mut Stack,
  frame_pointer: usize,
  instruction_pointer: usize,
}
#[allow(dead_code)]
impl<'ctx> Engine<'ctx> {
  pub fn new(ctx: &'ctx mut Context, stack: &'ctx mut Stack, compiler: &'ctx CompilerReturn) -> Self {
    //  return VM with 'ctx
    Self { ctx, compiler, stack, instruction_pointer: 0, frame_pointer: 0 }
  }
  pub fn bootstrap(ctx: &'ctx mut Context, source: &String, _debug: bool) -> Value {
    let arena_allocator = oxc_allocator::Allocator::default();
    let compiler = compile(&arena_allocator, source, ctx);
    let mut stack = Stack::new(STACK_LIMIT);
    let vm = Engine::new(ctx, &mut stack, &compiler);
    // debug
    if _debug {
      let mut disassembler = Disassembler::new(&compiler.code, "main.ts", &compiler.constants, vm.ctx);
      disassembler.disassemble();
    }
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
        opcode::OPCODE_ADD => self._addition_operation(),
        opcode::OPCODE_SUB => self._subtraction_operation(),
        opcode::OPCODE_MUL => self._multplication_operation(),
        opcode::OPCODE_DIV => self._division_operation(),
        opcode::OPCODE_EQ => self._eq_operation(),
        opcode::OPCODE_JUMP => self._jump_operation(),
        opcode::OPCODE_JUMP_IF_FALSE => self._jump_if_false_operation(),
        opcode::OPCODE_LOAD_GLOBAL_SCOPE => self.load_global_scope_operation(),
        opcode::OPCODE_SET_GLOBAL_SCOPE => self.set_global_scope_operation(),
        opcode::OPCODE_POP => return self.stack.peek(0).unwrap().to_owned(),
        opcode::OPCODE_SET_LOCAL_SCOPE => self.set_local_scope_operation(),
        opcode::OPCODE_LOAD_LOCAL_SCOPE => self.load_local_scope_operation(),
        opcode::OPCODE_HALF => {
          if !self.stack.is_empty() {
            let value = self.stack.pop().unwrap();
            return value;
          } else {
            return Value::Undefined;
          }
        }
        _ => todo!("opcode not implemented"),
      }
    }
  }

  fn set_local_scope_operation(&mut self) {}
  fn load_local_scope_operation(&mut self) {
    // let index = self.read();
  }

  fn set_global_scope_operation(&mut self) {
    let index = self.read();
    //  get last value from stack
    let value = self.stack.peek(0).unwrap();
    self.ctx.set_variable(index, value.to_owned());
  }

  fn load_global_scope_operation(&mut self) {
    let index = self.read();
    let value = self.ctx.get_variable_value(index);
    self.stack.push(value.to_owned());
  }
  fn _jump_operation(&mut self) {
    let index = self.read();
    self.instruction_pointer = index;
  }
  fn _jump_if_false_operation(&mut self) {
    let index = self.read();
    let condition = self.stack.pop().unwrap();
    if !condition.is_truthy() {
      self.instruction_pointer = index;
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
  fn _eq_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let value = Value::new_boolean(right.is_equal(&left));
    self.stack.push(value);
  }

  fn _multplication_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_float() && right.is_float() {
      let result = Value::new_float(left.get_float() - right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_integer() {
      let result = Value::new_integer(left.get_integer() - right.get_integer());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_float() {
      let result = Value::new_float(left.get_integer() as f64 - right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_float() && right.is_integer() {
      let result = Value::new_float(left.get_float() - right.get_integer() as f64);
      self.stack.push(result);
      return;
    }
    panic!("Unsupported operation, left: {:?} - right: {:?}", left, right);
  }
  pub fn _addition_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_number() && right.is_float() {
      let result = Value::new_float(left.get_integer() as f64 + right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_float() && right.is_integer() {
      let result = Value::new_float(left.get_float() + right.get_integer() as f64);
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_integer() {
      let result = Value::new_integer(left.get_integer() + right.get_integer());
      self.stack.push(result);
      return;
    }

    if left.is_string() && right.is_string() {
      let result = Value::new_string(left.get_string() + &right.get_string());
      self.stack.push(result);
      return;
    }
    panic!("Unsupported operation, left: {:?} + right: {:?}", left, right);
  }

  pub fn _subtraction_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_float() && right.is_float() {
      let result = Value::new_float(left.get_float() - right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_integer() {
      let result = Value::new_integer(left.get_integer() - right.get_integer());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_float() {
      let result = Value::new_float(left.get_integer() as f64 - right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_float() && right.is_integer() {
      let result = Value::new_float(left.get_float() - right.get_integer() as f64);
      self.stack.push(result);
      return;
    }
    panic!("Unsupported operation, left: {:?} - right: {:?}", left, right);
  }

  fn _division_operation(&mut self) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    if left.is_float() && right.is_float() {
      let result = Value::new_float(left.get_float() / right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_integer() {
      let result = Value::new_integer(left.get_integer() / right.get_integer());
      self.stack.push(result);
      return;
    }
    if left.is_integer() && right.is_float() {
      let result = Value::new_float(left.get_integer() as f64 / right.get_float());
      self.stack.push(result);
      return;
    }
    if left.is_float() && right.is_integer() {
      let result = Value::new_float(left.get_float() / right.get_integer() as f64);
      self.stack.push(result);
      return;
    }
    panic!("Unsupported operation, left: {:?} / right: {:?}", left, right);
  }

  fn binary_operation(&mut self, op: fn(Value, Value) -> Value) {
    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let result = op(left, right);
    self.stack.push(result);
  }
}
