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
      ast::Statement::Declaration(decl) => {
        self.generate_declaration(decl);
      }
      ast::Statement::IfStatement(stmt) => {
        self.generate_if_statement(stmt);
      }
      ast::Statement::EmptyStatement(_) => {
        self.generate_empty_statement();
      }
      ast::Statement::BlockStatement(stmt) => {
        self.generate_block_statement(stmt);
      }
      _ => {
        print!("{:?}", statement);
        panic!("Unknown statement")
      }
    }
  }
  pub fn generate_block_statement(&mut self, statement: &ast::BlockStatement) {
    for stmt in statement.body.iter() {
      self.generate_statement(stmt);
    }
  }
  pub fn generate_declaration(&mut self, declaration: &ast::Declaration) {
    match declaration {
      ast::Declaration::VariableDeclaration(decl) => {
        self.generate_variable_declaration(decl);
      }
      _ => {
        panic!("Unknown declaration")
      }
    }
  }

  pub fn generate_if_statement(&mut self, statement: &ast::IfStatement) {
    // 1. check the condition
    self.generate_expression(&statement.test);
    // 2. jump if false
    self.emit(opcode::OPCODE_JUMP_IF_FALSE);
    // 3. jump address to the consequent
    let jump_if_false_address = self.code.len();
    // 4. emit 0, we will fill this later
    self.emit(0);
    // 5. generate the consequent
    self.generate_statement(&statement.consequent);
    // 6. jump to the end of the if statement
    self.emit(opcode::OPCODE_JUMP);
    // 7. jump address to the end of the if statement
    let jump_address = self.code.len();
    // 8. emit 0, we will fill this later
    self.emit(0);
    // 9. fill the jump if false address
    self.code[jump_if_false_address] = self.code.len();
    // 10. generate the alternate if it exists
    if let Some(alternate) = &statement.alternate {
      // 11. generate the alternate
      self.generate_statement(alternate);
    }
    // 12. fill the jump address
    self.code[jump_address] = self.code.len();
  }

  pub fn generate_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) {
    match declaration.kind {
      ast::VariableDeclarationKind::Let => {
        // !todo: allocate or store the variable in the stack?
        // maybe we should alloc  using arena allocator, and finally we free all the memory
        // self.generate_expression(&declaration.init);
        todo!("Implement variable declaration")
      }
      _ => {
        panic!("Unknown variable declaration kind")
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
      ast::Expression::BooleanLiteral(value) => {
        self.generate_boolean_literal(value);
      }
      ast::Expression::StringLiteral(literal) => {
        self.generate_string_literal(literal);
      }
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

  pub fn generate_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
    self.constants.push(Value::Boolean(literal.value));
    let index = self.constants.len() - 1;
    self.emit(opcode::OPCODE_CONST);
    self.emit(index as usize);
  }

  pub fn generate_string_literal(&mut self, literal: &ast::StringLiteral) {
    let index = self.string_constants_index(literal.value.as_str());
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
  pub fn string_constants_index(&mut self, value: &str) -> usize {
    let value = Value::String(value.to_string());

    // maybe it's not the best way to do this, dont reuse the same constants
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
      "===" => {
        self.emit(opcode::OPCODE_EQ);
      }
      _ => {
        // **, %, <<, >>, >>>, &, |, ^, ==, !=, ===, !==, <, <=, >, >=, in, instanceof
        panic!("Unknown binary operator")
      }
    }
  }

  // debug disassemble
}
