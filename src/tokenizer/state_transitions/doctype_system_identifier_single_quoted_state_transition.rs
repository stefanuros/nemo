use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  unexpected_null_character_parse_error,
  abrupt_doctype_system_identifier_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn doctype_system_identifier_single_quoted_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State c: '{:?}'", c);

  return match c {
    Some('\u{0027}') => doctype_system_identifier_single_quoted_state_transition_apostrophe(c, current_state),
    Some('\u{0000}') => doctype_system_identifier_single_quoted_state_transition_null(c, current_token),
    Some('\u{003E}') => doctype_system_identifier_single_quoted_state_transition_greater_than_sign(c, current_state, current_token),
    None => doctype_system_identifier_single_quoted_state_transition_eof(current_token),
    _ => doctype_system_identifier_single_quoted_state_transition_anything_else(c, current_token)
  }
}

fn doctype_system_identifier_single_quoted_state_transition_apostrophe(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State Apostrophe: '{:?}'", c);

  *current_state = DataState::AfterDOCTYPESystemIdentifierState;

  return (None, false);
}

fn doctype_system_identifier_single_quoted_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::DOCTYPESystemIdentifierDoubleQuotedState.to_string(), c.unwrap());

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.push_to_system_identifier('\u{FFFD}');
  }

  return (None, false);
}

fn doctype_system_identifier_single_quoted_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State Greater Than Sign: '{:?}'", c);

  abrupt_doctype_system_identifier_parse_error::error(DataState::DOCTYPESystemIdentifierDoubleQuotedState.to_string(), c.unwrap());

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_force_quirks(true);
  }

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn doctype_system_identifier_single_quoted_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State EOF");

  eof_in_doctype_parse_error::error(DataState::DOCTYPESystemIdentifierDoubleQuotedState.to_string());

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

fn doctype_system_identifier_single_quoted_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("DOCTYPE System Identifier Single Quoted State Anything Else: '{:?}'", c);

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.push_to_system_identifier(c.unwrap());
  }

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::DoctypeToken;

  #[test]
  fn test_doctype_system_identifier_single_quoted_state_transition_apostrophe() {
    const C: Option<char> = Some('\'');
    let mut current_state: DataState = DataState::DOCTYPESystemIdentifierDoubleQuotedState;
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
    let result = doctype_system_identifier_single_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPESystemIdentifierState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_doctype_system_identifier_single_quoted_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::DOCTYPESystemIdentifierDoubleQuotedState;
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
          system_identifier: Some("abc�".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = doctype_system_identifier_single_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPESystemIdentifierDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
  
  #[test]
  fn test_doctype_system_identifier_single_quoted_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::DOCTYPESystemIdentifierDoubleQuotedState;
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
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = doctype_system_identifier_single_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_doctype_system_identifier_single_quoted_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::DOCTYPESystemIdentifierDoubleQuotedState;
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
    let result = doctype_system_identifier_single_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPESystemIdentifierDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_doctype_system_identifier_single_quoted_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::DOCTYPESystemIdentifierDoubleQuotedState;
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
          system_identifier: Some("abcg".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = doctype_system_identifier_single_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPESystemIdentifierDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
