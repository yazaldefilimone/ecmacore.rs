use crate::bytecode::opcode;
use crate::context::StoreKind;
use crate::{context::Context, values::Value};

use oxc_ast::ast::{self, AssignmentTarget, Program};
use oxc_syntax::NumberBase;

pub struct Compiler<'ctx> {
  code: Vec<usize>,
  name: String,
  constants: Vec<Value>,
  ctx: &'ctx mut Context,
}

#[allow(dead_code)]
pub struct CompilerReturn {
  name: String,
  pub code: Vec<usize>,
  pub constants: Vec<Value>,
}

#[allow(dead_code)]
impl<'ctx> Compiler<'ctx> {
  fn new(name: String, ctx: &'ctx mut Context) -> Self {
    Self { name, code: Vec::new(), constants: Vec::new(), ctx }
  }
  pub fn compile(program: &Program, ctx: &'ctx mut Context) -> CompilerReturn {
    let mut compiler = Compiler::new("main".to_string(), ctx);
    compiler.generate(program);
    CompilerReturn { name: compiler.name, code: compiler.code, constants: compiler.constants }
  }

  pub fn generate(&mut self, program: &Program) -> () {
    for statement in program.body.iter() {
      self.generate_statement(statement);
    }
    // end of program
    self.code.push(opcode::OPCODE_HALF);
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
        self._block_statement(stmt);
      }
      _ => {
        panic!("Unknown statement")
      }
    }
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
      ast::Expression::Identifier(identifier) => {
        self.generate_identifier(identifier);
      }
      ast::Expression::AssignmentExpression(assignment) => {
        self._assignment_expression(assignment);
      }
      _ => {
        panic!("Unknown expression")
      }
    }
  }

  pub fn _block_statement(&mut self, statement: &ast::BlockStatement) {
    self.enter_scope();
    for stmt in statement.body.iter() {
      self.generate_statement(stmt);
    }
    self.emit(opcode::OPCODE_POP);
    self.exit_scope();
  }

  pub fn _assignment_expression(&mut self, assignment: &ast::AssignmentExpression) {
    match assignment.operator.as_str() {
      "=" => {
        self.generate_assignment_target(&assignment.left, &assignment.right);
      }
      _ => {
        panic!("{} is not supported", assignment.operator.as_str())
      }
    }
  }
  pub fn generate_assignment_target(&mut self, target: &AssignmentTarget, init: &ast::Expression) {
    if target.is_identifier() {
      let variable_idx = self.get_assignment_target(&target);
      self.generate_expression(init);
      self.emit(opcode::OPCODE_SET_GLOBAL_SCOPE);
      self.emit(variable_idx);
      return;
    }
    panic!("Unknown left assignment expression")
  }
  pub fn get_assignment_target(&mut self, identifier: &ast::AssignmentTarget) -> usize {
    match identifier {
      ast::AssignmentTarget::SimpleAssignmentTarget(assign) => {
        return self._simple_assignment_target(assign);
      }
      ast::AssignmentTarget::AssignmentTargetPattern(_pattern) => {
        panic!("AssignmentTargetPattern is not supported")
        // return self._pattern_assignment_target(pattern);
      }
    }
  }
  pub fn _simple_assignment_target(&mut self, target: &ast::SimpleAssignmentTarget) -> usize {
    match target {
      ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) => {
        if let Some(kind) = self.ctx.get_kind_variable(&id.name) {
          if kind == StoreKind::Const {
            panic!("[Compiler] TypeError: '{}' is a read-only variable", id.name);
          }
        }
        self.get_variable_index(id)
      }
      _ => {
        panic!("Unknown left assignment expression")
      }
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
      ast::VariableDeclarationKind::Let => self._variable_declaration(declaration, StoreKind::Let),
      ast::VariableDeclarationKind::Const => self._variable_declaration(declaration, StoreKind::Const),
      _ => {
        panic!("Unknown variable declaration kind")
      }
    }
  }
  pub fn _variable_declaration(&mut self, declaration: &ast::VariableDeclaration, kind: StoreKind) {
    for declarator in declaration.declarations.iter() {
      self._variable_declarator(&declarator.id, &declarator.init, &kind);
    }
  }
  // !todo: we need to return the index of the variable for make more efficient to get the variable?
  pub fn _variable_declarator(
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
        let idx = self._define_variable(ident.name.as_str(), kind.clone());
        self.declarator_init(init, idx);
      }
      ast::BindingPatternKind::ArrayPattern(elem) => {
        for element in &elem.elements {
          if let Some(element) = element {
            self._variable_declarator(&element, init, &kind);
          }
        }
      }
      ast::BindingPatternKind::ObjectPattern(objects) => {
        for property in &objects.properties {
          match &property.key {
            ast::PropertyKey::Identifier(ident) => {
              let idx = self._define_variable(ident.name.as_str(), kind.clone());
              self.declarator_init(init, idx);
            }
            // ast::PropertyKey::PrivateIdentifier(ident) => {
            //   self.ctx.define_variable(ident.name.as_str().to_owned(), None);
            // }
            ast::PropertyKey::Expression(_) => {
              panic!("Expression key not supported")
            }
            _ => {
              panic!("Unknown property key")
            }
          }
        }
      }
      ast::BindingPatternKind::AssignmentPattern(_) => {
        panic!("Assignment pattern not supported")
      }
    }
  }

  pub fn declarator_init(&mut self, init: &Option<ast::Expression>, idx: usize) {
    if let Some(init) = init {
      self.generate_expression(&init);
    } else {
      // todo: panic error when is `const` declaraction
      self.constants.push(Value::new_undefined())
    }
    if self.ctx.is_global_scope() {
      self.emit(opcode::OPCODE_SET_GLOBAL_SCOPE);
    } else {
      self.emit(opcode::OPCODE_SET_LOCAL_SCOPE);
    }
    self.emit(idx);
  }
  pub fn generate_empty_statement(&mut self) {
    // We want to generate a half opcode here? huh... I don't know what to do here yet.
    self.emit(opcode::OPCODE_HALF);
  }

  pub fn generate_identifier(&mut self, identifier: &ast::IdentifierReference) {
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
  pub fn get_variable_index(&mut self, identifier: &ast::IdentifierReference) -> usize {
    if let Some(index) = self.ctx.get_variable_index(&identifier.name) {
      return index;
    }
    panic!("[Compiler] Reference Error: {} is not defined", identifier.name);
  }

  pub fn generate_numeric_literal(&mut self, literal: &ast::NumericLiteral) {
    let index = self.numerics_constants_index(literal);
    self.emit(opcode::OPCODE_CONST);
    self.emit(index as usize);
  }

  pub fn generate_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
    self.constants.push(Value::new_boolean(literal.value));
    let index = self.constants.len() - 1;
    self.emit(opcode::OPCODE_CONST);
    self.emit(index as usize);
  }

  pub fn generate_string_literal(&mut self, literal: &ast::StringLiteral) {
    let index = self.string_constants_index(literal.value.as_str());
    self.emit(opcode::OPCODE_CONST);
    self.emit(index as usize);
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
  pub fn emit(&mut self, byte: usize) {
    self.code.push(byte);
  }

  pub fn exit_scope(&mut self) {
    let len_of_variable_exit = self.ctx.deallocate_variable_in_scope();
    if len_of_variable_exit > 0 {
      self.emit(opcode::OPCODE_SCOPE_EXIT);
      self.emit(len_of_variable_exit);
    }
    // importante to exit before to deallocate all variables!
    self.ctx.exit_scope();
  }
  pub fn enter_scope(&mut self) {
    self.ctx.enter_scope();
  }

  pub fn _new_number(&mut self, value: &ast::NumericLiteral) -> Value {
    match &value.base {
      NumberBase::Decimal => Value::new_integer(value.value as i64),
      NumberBase::Float => Value::new_float(value.value),
      _ => Value::new_integer(value.value as i64),
    }
  }

  // numeric constants index
  pub fn numerics_constants_index(&mut self, value: &ast::NumericLiteral) -> usize {
    let value = self._new_number(&value);
    for (index, current_value) in self.constants.iter().enumerate() {
      // 1. check if the value is a number
      if !current_value.is_number() && !current_value.is_float() {
        continue;
      }
      // 2. check if the value is exists in the constants
      if current_value.is_equal(&value) {
        return index;
      }
    }
    // 3. if the value is not exists in the constants, push it
    self.constants.push(value);
    return self.constants.len() - 1;
  }

  // string constants index
  pub fn string_constants_index(&mut self, value: &str) -> usize {
    let value = Value::new_string(value.to_owned());

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
  pub fn _define_variable(&mut self, name: &str, kind: StoreKind) -> usize {
    match kind {
      StoreKind::Const => {
        if self.ctx.is_exist_variable(name) {
          panic!("[Compiler] SyntaxError: '{}' has already been declared.", name);
        }
        self.ctx.define_variable(name.to_owned(), None, StoreKind::Const)
      }
      StoreKind::Let => {
        if self.ctx.is_exist_variable(name) {
          panic!("[Compiler] SyntaxError: '{}' has already been declared.", name);
        }
        self.ctx.define_variable(name.to_owned(), None, StoreKind::Let)
      }
      StoreKind::Var => self.ctx.define_variable(name.to_owned(), None, StoreKind::Var),
    }
  }
}
