// tokens
pub enum Token {
  Identifier(String), // name
  Number(f64),        // value
  Operator(String),   // operator
  LParen,             // (
  RParen,             // )
}

// statements
pub enum Statement {
  Expression(Expression), // expression
}

// expressions
pub enum Expression {
  Binary(Box<Expression>, Token, Box<Expression>), // left, operator, right
  Unary(Token, Box<Expression>),                   // operator, right
  Literal(Token),                                  // token
  Grouping(Box<Expression>),                       // expression
}

// program
pub struct Program {
  pub statements: Vec<Statement>,
}
