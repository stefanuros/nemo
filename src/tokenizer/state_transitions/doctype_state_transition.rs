use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  missing_whitespace_before_doctype_name_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token,
  token_types::DoctypeToken
};

pub fn doctype_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => doctype_state_transition_whitespace(c, current_state),
    Some('\u{003E}') => doctype_state_transition_greater_than_sign(c, current_state),
    None => doctype_state_transition_eof(),
    _ => doctype_state_transition_anything_else(c, current_state)
  }
}

fn doctype_state_transition_whitespace(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE State Whitespace: '{:?}'", c);

  *current_state = DataState::BeforeDOCTYPENameState;

  return (None, false);
}

fn doctype_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::BeforeDOCTYPENameState;

  return (None, true);
}

fn doctype_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE State EOF");

  eof_in_doctype_parse_error::error(DataState::CommentEndDashState.to_string());

  return (
    Some(vec![
      Token::DOCTYPE(          
        DoctypeToken {
          force_quirks: true,
          ..DoctypeToken::default()
        }
      ),
      Token::EOFToken
    ]), 
    false
  );
}

fn doctype_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE State Anything Else: '{:?}'", c);

  missing_whitespace_before_doctype_name_parse_error::error(DataState::DOCTYPEState.to_string(), c.unwrap());

  *current_state = DataState::BeforeDOCTYPENameState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_doctype_state_transition_whitespace() {
    const C: Option<char> = Some('\u{0009}');
    let mut current_state: DataState = DataState::DOCTYPEState;
    
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = doctype_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPENameState, current_state);
  }

  #[test]
  fn test_doctype_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::DOCTYPEState;
    
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = doctype_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPENameState, current_state);
  }

  #[test]
  fn test_doctype_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::DOCTYPEState;
    
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::DOCTYPE(
          DoctypeToken {
            force_quirks: true,
            ..DoctypeToken::default()
          }
        ),
        Token::EOFToken
      ]), 
      false
    );
    let result = doctype_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPEState, current_state);
  }

  #[test]
  fn test_doctype_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::DOCTYPEState;
    
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = doctype_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPENameState, current_state);
  }
}
