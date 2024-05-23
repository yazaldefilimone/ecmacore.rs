#![allow(dead_code)]
use crate::bytecode::opcode;
use crate::context::{Context, StoreKind};
use crate::values::Value;
use oxc_ast::ast::{self, AssignmentTarget, Program};
use oxc_syntax::NumberBase;

pub struct Compiler<'ctx> {
  code: Vec<usize>,
  name: String,
  constants: Vec<Value>,
  ctx: &'ctx mut Context,
}

pub struct CompilerReturn {
  name: String,
  pub code: Vec<usize>,
  pub constants: Vec<Value>,
}

impl<'ctx> Compiler<'ctx> {
  pub fn new(name: String, ctx: &'ctx mut Context) -> Self {
    Self { name, code: Vec::new(), constants: Vec::new(), ctx }
  }

  pub fn compile(program: &Program, ctx: &'ctx mut Context) -> CompilerReturn {
    let mut compiler = Compiler::new("main".to_string(), ctx);
    compiler.generate_program(program);
    CompilerReturn { name: compiler.name, code: compiler.code, constants: compiler.constants }
  }

  fn generate_program(&mut self, program: &Program) {
    for statement in &program.body {
      self.generate_statement(statement);
    }
    self.emit(opcode::OPCODE_HALF);
  }

  fn generate_statement(&mut self, statement: &ast::Statement) {
    match statement {
      ast::Statement::ExpressionStatement(stmt) => self.generate_expression(&stmt.expression),
      ast::Statement::Declaration(decl) => self.generate_declaration(decl),
      ast::Statement::IfStatement(stmt) => self.generate_if_statement(stmt),
      ast::Statement::EmptyStatement(_) => self.generate_empty_statement(),
      ast::Statement::BlockStatement(stmt) => self.generate_block_statement(stmt),
      _ => panic!("Unknown statement"),
    }
  }

  fn generate_expression(&mut self, expression: &ast::Expression) {
    match expression {
      ast::Expression::NumericLiteral(value) => self.generate_numeric_literal(value),
      ast::Expression::BooleanLiteral(value) => self.generate_boolean_literal(value),
      ast::Expression::StringLiteral(literal) => self.generate_string_literal(literal),
      ast::Expression::BinaryExpression(binary) => self.generate_binary_expression(binary),
      ast::Expression::Identifier(identifier) => self.generate_identifier(identifier),
      ast::Expression::AssignmentExpression(assignment) => self.generate_assignment_expression(assignment),
      _ => panic!("Unknown expression"),
    }
  }

  fn generate_block_statement(&mut self, statement: &ast::BlockStatement) {
    self.enter_scope();
    for stmt in &statement.body {
      self.generate_statement(stmt);
    }
    self.emit(opcode::OPCODE_POP);
    self.exit_scope();
  }

  fn generate_assignment_expression(&mut self, assignment: &ast::AssignmentExpression) {
    match assignment.operator.as_str() {
      "=" => self.generate_assignment_target(&assignment.left, &assignment.right),
      _ => panic!("{} is not supported", assignment.operator.as_str()),
    }
  }

  fn generate_assignment_target(&mut self, target: &AssignmentTarget, init: &ast::Expression) {
    if target.is_identifier() {
      let variable_idx = self.get_assignment_target(target);
      self.generate_expression(init);
      self.emit(opcode::OPCODE_SET_GLOBAL_SCOPE);
      self.emit(variable_idx);
      return;
    }
    panic!("Unknown left assignment expression");
  }

  fn get_assignment_target(&mut self, identifier: &ast::AssignmentTarget) -> usize {
    match identifier {
      ast::AssignmentTarget::SimpleAssignmentTarget(assign) => self.get_simple_assignment_target(assign),
      ast::AssignmentTarget::AssignmentTargetPattern(_) => panic!("AssignmentTargetPattern is not supported"),
    }
  }

  fn get_simple_assignment_target(&mut self, target: &ast::SimpleAssignmentTarget) -> usize {
    match target {
      ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) => {
        if let Some(kind) = self.ctx.get_kind_variable(&id.name) {
          if kind == StoreKind::Const {
            panic!("[Compiler] TypeError: '{}' is a read-only variable", id.name);
          }
        }
        self.get_variable_index(id)
      }
      _ => panic!("Unknown left assignment expression"),
    }
  }

  fn generate_declaration(&mut self, declaration: &ast::Declaration) {
    match declaration {
      ast::Declaration::VariableDeclaration(decl) => self.generate_variable_declaration(decl),
      _ => panic!("Unknown declaration"),
    }
  }

  fn generate_if_statement(&mut self, statement: &ast::IfStatement) {
    self.generate_expression(&statement.test);
    self.emit(opcode::OPCODE_JUMP_IF_FALSE);
    let jump_if_false_address = self.code.len();
    self.emit(0);
    self.generate_statement(&statement.consequent);
    self.emit(opcode::OPCODE_JUMP);
    let jump_address = self.code.len();
    self.emit(0);
    self.code[jump_if_false_address] = self.code.len();
    if let Some(alternate) = &statement.alternate {
      self.generate_statement(alternate);
    }
    self.code[jump_address] = self.code.len();
  }

  fn generate_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) {
    match declaration.kind {
      ast::VariableDeclarationKind::Let => self.handle_variable_declaration(declaration, StoreKind::Let),
      ast::VariableDeclarationKind::Const => self.handle_variable_declaration(declaration, StoreKind::Const),
      _ => panic!("Unknown variable declaration kind"),
    }
  }

  fn handle_variable_declaration(&mut self, declaration: &ast::VariableDeclaration, kind: StoreKind) {
    for declarator in &declaration.declarations {
      self.handle_variable_declarator(&declarator.id, &declarator.init, &kind);
    }
  }

  fn handle_variable_declarator(
    &mut self,
    pattern: &ast::BindingPattern,
    init: &Option<ast::Expression>,
    kind: &StoreKind,
  ) {
    match &pattern.kind {
      ast::BindingPatternKind::BindingIdentifier(ident) => {
        if kind == &StoreKind::Const && init.is_none() {
          panic!(
            "[Compiler] SyntaxError: 'const' declarations must be initialized at '{}'",
            ident.name
          );
        }
        let idx = self.define_variable(ident.name.as_str(), kind.clone());
        self.initialize_declarator(init, idx);
      }
      ast::BindingPatternKind::ArrayPattern(elem) => {
        for element in &elem.elements {
          if let Some(element) = element {
            self.handle_variable_declarator(element, init, kind);
          }
        }
      }
      ast::BindingPatternKind::ObjectPattern(objects) => {
        for property in &objects.properties {
          match &property.key {
            ast::PropertyKey::Identifier(ident) => {
              let idx = self.define_variable(ident.name.as_str(), kind.clone());
              self.initialize_declarator(init, idx);
            }
            ast::PropertyKey::Expression(_) => panic!("Expression key not supported"),
            _ => panic!("Unknown property key"),
          }
        }
      }
      ast::BindingPatternKind::AssignmentPattern(_) => panic!("Assignment pattern not supported"),
    }
  }

  fn initialize_declarator(&mut self, init: &Option<ast::Expression>, idx: usize) {
    if let Some(init) = init {
      self.generate_expression(init);
    } else {
      self.constants.push(Value::new_undefined())
    }
    if self.ctx.is_global_scope() {
      self.emit(opcode::OPCODE_SET_GLOBAL_SCOPE);
    } else {
      self.emit(opcode::OPCODE_SET_LOCAL_SCOPE);
    }
    self.emit(idx);
  }

  fn generate_empty_statement(&mut self) {
    self.emit(opcode::OPCODE_HALF);
  }

  fn generate_identifier(&mut self, identifier: &ast::IdentifierReference) {
    if let Some(index) = self.ctx.get_variable_index(&identifier.name) {
      self.emit(opcode::OPCODE_LOAD_GLOBAL_SCOPE);
      self.emit(index);
      return;
    }
    if !self.ctx.is_internal(&identifier.name) {
      panic!("[Compiler] {} is not implemented yet", identifier.name);
    }
    panic!("[Compiler] Reference Error: {} is not defined", identifier.name);
  }

  fn get_variable_index(&mut self, identifier: &ast::IdentifierReference) -> usize {
    if let Some(index) = self.ctx.get_variable_index(&identifier.name) {
      return index;
    }
    panic!("[Compiler] Reference Error: {} is not defined", identifier.name);
  }

  fn generate_numeric_literal(&mut self, literal: &ast::NumericLiteral) {
    let index = self.get_numeric_constant_index(literal);
    self.emit(opcode::OPCODE_CONST);
    self.emit(index);
  }

  // fn generate_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
  //     self.constants.push(Value::new_boolean(literal.value));
  //     let index = self.constants.len() - 1;
  //     self.emit(op
  fn generate_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
    self.constants.push(Value::new_boolean(literal.value));
    let index = self.constants.len() - 1;
    self.emit(opcode::OPCODE_CONST);
    self.emit(index);
  }

  fn generate_string_literal(&mut self, literal: &ast::StringLiteral) {
    let index = self.get_string_constant_index(&literal.value);
    self.emit(opcode::OPCODE_CONST);
    self.emit(index);
  }

  fn generate_binary_expression(&mut self, binary: &ast::BinaryExpression) {
    self.generate_expression(&binary.left);
    self.generate_expression(&binary.right);
    match binary.operator.as_str() {
      "+" => self.emit(opcode::OPCODE_ADD),
      "-" => self.emit(opcode::OPCODE_SUB),
      "*" => self.emit(opcode::OPCODE_MUL),
      "/" => self.emit(opcode::OPCODE_DIV),
      "===" => self.emit(opcode::OPCODE_EQ),
      _ => panic!("Unknown binary operator"),
    }
  }

  fn emit(&mut self, byte: usize) {
    self.code.push(byte);
  }

  fn exit_scope(&mut self) {
    let len_of_variable_exit = self.ctx.deallocate_variable_in_scope();
    if len_of_variable_exit > 0 {
      self.emit(opcode::OPCODE_SCOPE_EXIT);
      self.emit(len_of_variable_exit);
    }
    self.ctx.exit_scope();
  }

  fn enter_scope(&mut self) {
    self.ctx.enter_scope();
  }

  fn create_value(&mut self, value: &ast::NumericLiteral) -> Value {
    match value.base {
      NumberBase::Decimal => Value::new_integer(value.value as i64),
      NumberBase::Float => Value::new_float(value.value),
      _ => Value::new_integer(value.value as i64),
    }
  }

  fn get_numeric_constant_index(&mut self, value: &ast::NumericLiteral) -> usize {
    let new_value = self.create_value(value);
    for (index, current_value) in self.constants.iter().enumerate() {
      if current_value.is_number() && current_value.is_equal(&new_value) {
        return index;
      }
    }
    self.constants.push(new_value);
    self.constants.len() - 1
  }

  fn get_string_constant_index(&mut self, value: &str) -> usize {
    let new_value = Value::new_string(value.to_owned());
    for (index, current_value) in self.constants.iter().enumerate() {
      if current_value.is_string() && current_value.get_string() == new_value.get_string() {
        return index;
      }
    }
    self.constants.push(new_value);
    self.constants.len() - 1
  }

  fn define_variable(&mut self, name: &str, kind: StoreKind) -> usize {
    if self.ctx.is_exist_variable(name) {
      panic!("[Compiler] SyntaxError: '{}' has already been declared.", name);
    }
    self.ctx.define_variable(name.to_owned(), None, kind)
  }
}
