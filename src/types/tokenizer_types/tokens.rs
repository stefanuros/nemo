use std::fmt::{
  Display,
  Formatter,
  Result
};

use super::token_types::{
  TagToken,
  DoctypeToken
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  CharacterToken(char),
  CommentToken(String),
  DOCTYPE(DoctypeToken),
  EndTagToken(TagToken),
  EOFToken,
  StartTagToken(TagToken),
}

impl Token {
  pub fn empty_comment() -> Token {
    return Token::CommentToken("".to_string());
  }

  pub fn new_comment(comment: &str) -> Token {
    return Token::CommentToken(comment.to_string());
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
