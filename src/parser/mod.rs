mod lexer;
use lexer::Lexer;

use crate::new_lexer;

new_lexer!(Parser);

impl<'i> Parser<'i> {
  pub fn parse(&mut self) {}
}
