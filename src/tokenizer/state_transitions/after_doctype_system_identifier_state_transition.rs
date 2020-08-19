use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  unexpected_character_after_doctype_system_identifier_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn after_doctype_system_identifier_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE System Identifier State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => after_doctype_system_identifier_state_transition_whitespace(c),
    Some('\u{003E}') => after_doctype_system_identifier_state_transition_greater_than_sign(c, current_state, current_token),
    None => after_doctype_system_identifier_state_transition_eof(current_token),
    _ => after_doctype_system_identifier_state_transition_anything_else(c, current_state)
  }
}

fn after_doctype_system_identifier_state_transition_whitespace(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE System Identifier State Whitespace: '{:?}'", c);

  return (None, false);
}

fn after_doctype_system_identifier_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE System Identifier State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn after_doctype_system_identifier_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE System Identifier State EOF");

  eof_in_doctype_parse_error::error(DataState::AfterDOCTYPESystemIdentifierState.to_string());

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_force_quirks(true);
  }

  return (
    Some(vec![
      current_token.clone().unwrap(),
      Token::EOFToken
    ]), 
    false
  );
}

fn after_doctype_system_identifier_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE System Identifier State Anything Else: '{:?}'", c);

  unexpected_character_after_doctype_system_identifier_parse_error::error(DataState::AfterDOCTYPESystemIdentifierState.to_string(), c.unwrap());

  *current_state = DataState::BogusDOCTYPEState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::DoctypeToken;
  
  #[test]
  fn test_after_doctype_system_identifier_state_transition_whitespace() {
    const C: Option<char> = Some(' ');
    let mut current_state: DataState = DataState::AfterDOCTYPESystemIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_doctype_system_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPESystemIdentifierState, current_state);
    assert_eq!(expected_current_token, current_token);
  }  
  #[test]
  fn test_after_doctype_system_identifier_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AfterDOCTYPESystemIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = after_doctype_system_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_doctype_system_identifier_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AfterDOCTYPESystemIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap(),
        Token::EOFToken
      ]), 
      false
    );
    let result = after_doctype_system_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPESystemIdentifierState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_doctype_system_identifier_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::AfterDOCTYPESystemIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          system_identifier: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = after_doctype_system_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
