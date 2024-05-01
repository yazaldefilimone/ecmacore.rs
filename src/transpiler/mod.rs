use crate::tokens;
use ast::Program;
use oxc_ast::ast::{self};

use crate::tokens::{Expression, Statement, Token};

pub struct Transpiler {}
#[allow(dead_code)]
impl<'input> Transpiler {
  pub fn transpile(program: &'input Program) -> tokens::Program {
    let mut statements = Vec::new();
    for statement in program.body.iter() {
      statements.push(Transpiler::transpile_statement(statement));
    }
    let program = crate::tokens::Program { statements };
    return program;
  }

  pub fn transpile_statement(statement: &'input ast::Statement) -> Statement {
    match &statement {
      ast::Statement::ExpressionStatement(expression) => {
        let expression = Transpiler::transpile_expression(expression);
        Statement::Expression(expression)
      }
      ast::Statement::EmptyStatement(_) => Statement::Expression(Expression::Empty),
      _ => {
        panic!("Invalid statement");
      }
    }
  }
  pub fn transpile_expression(expression: &'input ast::ExpressionStatement) -> Expression {
    match &expression.expression {
      // ast::Expression::BinaryExpression(expression) => {
      //   panic!("Binary expression not implemented")
      //   // return Transpiler::transpile_binary_expression(expression);
      // }
      // ast::Expression::UnaryExpression(expression) => {
      //   panic!("Unary expression not implemented")
      //   return Transpiler::transpile_unary_expression(expression);

      // }
      ast::Expression::NumericLiteral(expression) => {
        return Transpiler::transpile_number(expression);
      }
      ast::Expression::StringLiteral(expression) => {
        return Transpiler::transpile_string(expression);
      }
      _ => {
        panic!("Invalid expression");
      }
    }
  }
  pub fn transpile_number(expression: &'input ast::NumericLiteral) -> Expression {
    Expression::Literal(Token::Number(expression.value))
  }
  pub fn transpile_string(expression: &'input ast::StringLiteral) -> Expression {
    Expression::Literal(Token::_String(expression.value.as_str().to_string()))
  }
}
