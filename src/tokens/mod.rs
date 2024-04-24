// tokens

#[derive(Debug)]
pub enum Token {
  Identifier(String), // name
  _String(String),    // "string", 'a'
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
  Empty,                                           // empty
}

// program
pub struct Program {
  pub statements: Vec<Statement>,
}
