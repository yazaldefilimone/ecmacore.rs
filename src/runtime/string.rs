/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

/*
*/
#[derive(Debug)]
pub struct StringValue {
  value: String,
  ascii_word_characters: String,
}

/// https://tc39.es/ecma262/#ASCII-word-characters
//  let ascii_word_characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";
// /// The definition of white space is the union of WhiteSpace and LineTerminator.
//  let whitespace = tokenizer.whitespace ++ tokenizer.line_terminators;
//  let empty: Self = fromLiteral("");
impl StringValue {
  pub fn new(value: String) -> Self {
    let ascii_word_characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_".to_owned();
    StringValue { value, ascii_word_characters }
  }
}
