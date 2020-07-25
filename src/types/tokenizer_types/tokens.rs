use std::fmt::{
  Display,
  Formatter,
  Result
};

use super::token_types::TagToken ;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  CharacterToken(char),
  CommentToken(String),
  DOCTYPE(),
  EndTagToken(TagToken),
  EOFToken,
  StartTagToken(TagToken),
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
