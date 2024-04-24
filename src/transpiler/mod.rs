use ast::Program;
use oxc_ast::ast::{self};

use crate::tokens::{Expression, Statement, Token};

pub struct Transpiler {}

impl<'input> Transpiler {
  pub fn transpile(program: &'input Program) {
    let mut statements = Vec::new();
    for statement in &program.body.iter() {
      statements.push(Transpiler::transpile_statement(statement));
    }
  }

  pub fn transpile_statement(statement: &'input ast::Statement) -> Statement {
    match statement {
      ast::Statement::Expression(expression) => {
        let expression = Transpiler::transpile_expression(expression);
        Statement::Expression(expression)
      }
      _ => {
        panic!("Invalid statement");
      }
    }
  }

  pub fn transpile_expression(expression: &'input ast::Expression) -> Expression {
    match expression {
      ast::Expression::NumericLiteral(token) => Expression::Literal(token.clone()),
      ast::Expression::BinaryExpression(binary_expression) => {
        Transpiler::transpile_binary_expression(binary_expression)
      }
      ast::Expression::UnaryExpression(operator, right) => {
        let right = Box::new(Transpiler::transpile_expression(right));
        Expression::Unary(operator.clone(), right)
      }
      ast::Expression::StringLiteral(token) => Expression::Literal(token.clone()),
      _ => {
        panic!("Invalid expression");
      }
    }
  }

  pub fn transpile_binary_expression(expression: &'input ast::BinaryExpression) -> Expression {
    let left = Box::new(Transpiler::transpile_expression(&expression.left));
    let right = Box::new(Transpiler::transpile_expression(&expression.right));
    let operator = Token::Operator(expression.operator.as_str().to_owned());
    Expression::Binary(left, operator, right)
  }

  pub fn transpile_unary_expression(expression: &'input ast::UnaryExpression) -> Expression {
    let right = Box::new(Transpiler::transpile_expression(right));
    let 
    Expression::Unary(&expression.operator.as_str(), right)
  }
}
