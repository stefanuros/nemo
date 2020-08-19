use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn bogus_doctype_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus DOCTYPE State c: '{:?}'", c);

  return match c {
    Some('\u{003E}') => bogus_doctype_state_transition_greater_than_sign(c, current_state, current_token),
    Some('\u{0000}') => bogus_doctype_state_transition_null(c),
    None => bogus_doctype_state_transition_eof(current_token),
    _ => bogus_doctype_state_transition_anything_else(c)
  }
}

fn bogus_doctype_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus DOCTYPE State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn bogus_doctype_state_transition_null(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus DOCTYPE State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::BogusDOCTYPEState.to_string(), c.unwrap());

  return (None, false);
}

fn bogus_doctype_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus DOCTYPE State EOF");

  return (
    Some(vec![
      current_token.clone().unwrap(),
      Token::EOFToken
    ]), 
    false
  );
}

fn bogus_doctype_state_transition_anything_else(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus DOCTYPE State Anything Else: '{:?}'", c);

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::DoctypeToken;
  
  #[test]
  fn test_bogus_doctype_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BogusDOCTYPEState;
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
    let result = bogus_doctype_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_doctype_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::BogusDOCTYPEState;
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
    let result = bogus_doctype_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_doctype_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BogusDOCTYPEState;
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
        expected_current_token.clone().unwrap(),
        Token::EOFToken
      ]), 
      false
    );
    let result = bogus_doctype_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_doctype_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::BogusDOCTYPEState;
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
    let result = bogus_doctype_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
