use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  CharacterToken(char),
  CommentToken(String),
  DOCTYPE(),
  EndTagToken(String),
  EOFToken,
  StartTagToken(String),
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
