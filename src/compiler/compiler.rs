use crate::values::Value;

use crate::bytecode::opcode;

use oxc_ast::ast::{self, Program};

pub struct Compiler {
  code: Vec<usize>,
  name: String,
  constants: Vec<Value>,
}

#[allow(dead_code)]
pub struct CompilerReturn {
  name: String,
  pub code: Vec<usize>,
  pub constants: Vec<Value>,
}

#[allow(dead_code)]
impl Compiler {
  fn new(name: String) -> Self {
    Self { name, code: Vec::new(), constants: Vec::new() }
  }
  pub fn compile(program: &Program) -> CompilerReturn {
    let mut compiler = Compiler::new("main".to_string());
    compiler.generate(program);
    compiler.code.push(opcode::OPCODE_HALF);
    CompilerReturn { name: compiler.name, code: compiler.code, constants: compiler.constants }
  }

  pub fn generate(&mut self, program: &Program) -> () {
    for statement in program.body.iter() {
      self.generate_statement(statement);
    }
  }

  pub fn generate_statement(&mut self, statement: &ast::Statement) {
    match statement {
      ast::Statement::ExpressionStatement(stmt) => {
        self.generate_expression(&stmt.expression);
      }
      ast::Statement::EmptyStatement(_) => {
        self.generate_empty_statement();
      }
      _ => {
        panic!("Unknown statement")
      }
    }
  }
  pub fn generate_empty_statement(&mut self) {
    // We want to generate a half opcode here? huh... I don't know what to do here yet.
    self.emit(opcode::OPCODE_HALF);
  }
  pub fn generate_expression(&mut self, expression: &ast::Expression) {
    match &expression {
      ast::Expression::NumericLiteral(value) => {
        self.generate_numeric_literal(value);
      }
      // ast::ExpressionStatement::StringLiteral(value) => {
      //   let index = self.string_constants_index(value.clone());
      //   self.emit(0x02);
      //   self.emit(index as u8);
      // }
      ast::Expression::BinaryExpression(binary) => {
        self.generate_binary_expression(binary);
      }
      _ => {
        panic!("Unknown expression")
      }
    }
  }

  pub fn generate_numeric_literal(&mut self, literal: &ast::NumericLiteral) {
    let index = self.numerics_constants_index(literal.value);
    self.emit(opcode::OPCODE_CONST);
    self.emit(index as usize);
  }

  pub fn emit(&mut self, byte: usize) {
    self.code.push(byte);
  }

  // numeric constants index
  pub fn numerics_constants_index(&mut self, value: f64) -> usize {
    let value = Value::Number(value);
    for (index, current_value) in self.constants.iter().enumerate() {
      // 1. check if the value is a number
      if !current_value.is_number() {
        continue;
      }
      // 2. check if the value is exists in the constants
      if current_value.get_number() == value.get_number() {
        return index;
      }
    }
    // 3. if the value is not exists in the constants, push it
    self.constants.push(value);
    return self.constants.len() - 1;
  }

  // string constants index
  pub fn string_constants_index(&mut self, value: String) -> usize {
    let value = Value::String(value);
    for (index, current_value) in self.constants.iter().enumerate() {
      // 1. check if the value is a string
      if !current_value.is_string() {
        continue;
      }
      // 2. check if the value is exists in the constants
      if current_value.get_string() == value.get_string() {
        return index;
      }
    }
    // 3. if the value is not exists in the constants, push it
    self.constants.push(value);
    return self.constants.len() - 1;
  }

  pub fn generate_binary_expression(&mut self, binary: &ast::BinaryExpression) {
    self.generate_expression(&binary.left);
    self.generate_expression(&binary.right);
    match binary.operator.as_str() {
      "+" => {
        self.emit(opcode::OPCODE_ADD);
      }
      "-" => {
        self.emit(opcode::OPCODE_SUB);
      }
      "*" => {
        self.emit(opcode::OPCODE_MUL);
      }
      "/" => {
        self.emit(opcode::OPCODE_DIV);
      }
      _ => {
        // **, %, <<, >>, >>>, &, |, ^, ==, !=, ===, !==, <, <=, >, >=, in, instanceof
        panic!("Unknown binary operator")
      }
    }
  }
}
