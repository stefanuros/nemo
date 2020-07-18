use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq)]
pub enum Token {
  CharacterToken(char),
  CommentToken(String),
  CurrentTagToken(String),
  EndTagToken(String),
  EOFToken,
  StartTagToken(String)
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
