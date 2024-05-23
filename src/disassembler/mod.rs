#![allow(dead_code)]
use crate::bytecode::opcode;
use crate::context::Context;
use crate::utils::opcode_to_string;
use crate::values::Value;

pub struct Disassembler<'ctx> {
  constants: &'ctx Vec<Value>,
  code: &'ctx Vec<usize>,
  instructions: Vec<Vec<String>>,
  name: String,
  line: Vec<String>,
  ctx: &'ctx mut Context,
}

impl<'ctx> Disassembler<'ctx> {
  pub fn new(code: &'ctx Vec<usize>, name: &str, constants: &'ctx Vec<Value>, ctx: &'ctx mut Context) -> Self {
    let instructions = vec![];
    let line = vec![];
    Self { code, constants, instructions, ctx, line, name: name.to_owned() }
  }
  pub fn disassemble(&mut self) -> () {
    let header = format!("{:<10} {:<12} {:<14} {}", "Offset", "Bytes", "Opcode", "Operand");
    println!("----------------- Disassembler -----------------");
    println!("{}", header);
    println!("------------------------------------------------");
    // ------------------------------------------------
    let mut offset = 0;
    while offset < self.code.len() {
      offset = self.disassemble_instruction(offset);
      self.print_line();
    }
  }

  fn disassemble_instruction(&mut self, offset: usize) -> usize {
    self.print_offset(offset);
    let opcode = self.code[offset];
    match opcode {
      opcode::OPCODE_HALF
      | opcode::OPCODE_SUB
      | opcode::OPCODE_MUL
      | opcode::OPCODE_DIV
      | opcode::OPCODE_ADD
      | opcode::OPCODE_POP
      | opcode::OPCODE_EQ => {
        return self.disassemble_simple(opcode, offset);
      }
      opcode::OPCODE_CONST => {
        return self.disassemble_const(offset, opcode);
      }
      opcode::OPCODE_SET_GLOBAL_SCOPE | opcode::OPCODE_LOAD_GLOBAL_SCOPE => {
        return self.disassemble_global(offset, opcode);
      }
      opcode::OPCODE_SET_LOCAL_SCOPE | opcode::OPCODE_LOAD_LOCAL_SCOPE => {
        return self.disassemble_local(offset, opcode);
      }
      opcode::OPCODE_JUMP_IF_FALSE | opcode::OPCODE_JUMP => {
        return self.disassemble_jump(offset, opcode);
      }
      _ => {
        print!("[Disassemble] Unknown opcode: {}", opcode_to_string(opcode));
        return offset + 1;
      }
    }
  }

  pub fn disassemble_jump(&mut self, offset: usize, opcode: usize) -> usize {
    self.dumb_bytecode(offset, 3);
    self.print_opcode(opcode);
    let index = self.code[offset + 1];
    let jump = self.code[offset + 2];
    self.line.push(format!("{:<8}", index));
    self.line.push(format!("{:<8}", jump));
    return offset + 3;
  }
  pub fn disassemble_global(&mut self, offset: usize, opcode: usize) -> usize {
    self.dumb_bytecode(offset, 2);
    self.print_opcode(opcode);
    let index = self.code[offset + 1];
    let var = &self.ctx.get_global_variable(index).unwrap().name;
    self.print_operand(var.to_owned());
    return offset + 2;
  }
  pub fn disassemble_local(&mut self, offset: usize, opcode: usize) -> usize {
    self.dumb_bytecode(offset, 2);
    self.print_opcode(opcode);
    let index = self.code[offset + 1];
    let var = &self.ctx.get_local_variable(index).unwrap().name;
    self.print_operand(var.to_owned());
    return offset + 2;
  }
  pub fn disassemble_const(&mut self, offset: usize, opcode: usize) -> usize {
    self.dumb_bytecode(offset, 2);
    self.print_opcode(opcode);
    let index = self.code[offset + 1];
    self.print_operand(self.constants[index].to_string());
    return offset + 2;
  }

  pub fn disassemble_simple(&mut self, opcode: usize, offset: usize) -> usize {
    self.dumb_bytecode(offset, 1);
    self.print_opcode(opcode);
    return offset + 1;
  }

  pub fn dumb_bytecode(&mut self, offset: usize, count: usize) -> () {
    let mut output = String::new();
    for i in 0..count {
      output += format!("{:02X} ", self.code[offset + i] & 0xFF).as_str();
    }
    self.line.push(format!("{:<12} ", output.trim()));
  }
  pub fn print_opcode(&mut self, opcode: usize) -> () {
    self.line.push(format!("{:<14} ", opcode_to_string(opcode).trim()));
  }
  pub fn print_operand(&mut self, operand: String) -> () {
    self.line.push(format!("({})", operand));
  }

  pub fn print_offset(&mut self, offset: usize) -> () {
    let formatted = format!("{:08X} ", offset);
    self.line.push(format!("{:<10} ", formatted.trim()));
  }

  pub fn print_line(&mut self) -> () {
    println!("{}", format!("{}", self.line.join("")));
    self.line.clear();
  }
}
