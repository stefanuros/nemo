use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug)]
pub enum Token {
  CharacterToken(char),
  CommentToken(String),
  EOFToken,
  CurrentTagToken(String)
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
