#![allow(dead_code)]
use crate::bytecode::opcode;
use crate::utils::opcode_to_string;
use crate::values::Value;

pub struct Disassembler<'ctx> {
  constants: &'ctx Vec<Value>,
  code: &'ctx Vec<usize>,
  string: String,
}

impl<'ctx> Disassembler<'ctx> {
  pub fn new(code: &'ctx Vec<usize>, constants: &'ctx Vec<Value>) -> Self {
    Self { code, constants, string: String::new() }
  }
  pub fn disassemble(&mut self) -> () {
    println!("~~~~~~~~~~~~~~~~~ Disasseble ~~~~~~~~~~~~~~~");
    println!("Offset    Bytes     Opcode    Operand");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let mut ip = 0;
    while ip < self.code.len() {
      if ip != 0 {
        println!("{}", self.string);
      }
      ip = self.disassemble_instruction(ip);
    }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  }

  fn disassemble_instruction(&mut self, ip: usize) -> usize {
    self.string.clear();
    self.string += format!("{:04}      ", ip).as_str();
    let opcode = self.code[ip];
    match opcode {
      opcode::OPCODE_HALF => {
        return self.disassemble_simple(opcode, ip);
      }
      opcode::OPCODE_CONST => {
        return self.disassemble_const(ip, opcode);
      }
      opcode::OPCODE_EQ => {
        return self.disassemble_simple(opcode, ip);
      }
      _ => {
        print!("Disassemble: Unknown opcode: {}", opcode_to_string(opcode));
        return ip + 1;
      }
    }
  }
  pub fn disassemble_const(&mut self, offset: usize, opcode: usize) -> usize {
    self.dumb_bytecode(offset, 2);
    self.print_opcode(opcode);
    let index = self.code[offset + 1];
    self.string += format!("    ({})", self.constants[index]).as_str();
    return offset + 2;
  }

  pub fn disassemble_simple(&mut self, opcode: usize, offset: usize) -> usize {
    self.dumb_bytecode(offset, 1);
    self.print_opcode(opcode);
    return offset + 1;
  }

  pub fn dumb_bytecode(&mut self, offset: usize, count: usize) -> () {
    for i in 0..count {
      self.string += format!("{:02x}  ", self.code[offset + i]).as_str();
    }
    self.string += "  ";
  }

  pub fn print_opcode(&mut self, opcode: usize) -> () {
    self.string += format!("{}", opcode_to_string(opcode)).as_str()
  }

  pub fn disassemble_hex(&self, index: usize) -> String {
    format!("{:x}", index)
  }
}
