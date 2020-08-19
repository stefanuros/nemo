use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  missing_doctype_public_identifier_parse_error,
  missing_quote_before_doctype_public_identifier_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn before_doctype_public_identifier_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => before_doctype_public_identifier_state_transition_whitespace(c),
    Some('\u{0022}') => before_doctype_public_identifier_state_transition_quotation_mark(c, current_state, current_token),
    Some('\u{0027}') => before_doctype_public_identifier_state_transition_apostrophe(c, current_state, current_token),
    Some('\u{003E}') => before_doctype_public_identifier_state_transition_greater_than_sign(c, current_state, current_token),
    None => before_doctype_public_identifier_state_transition_eof(current_token),
    _ => before_doctype_public_identifier_state_transition_anything_else(c, current_state, current_token)
  }
}

fn before_doctype_public_identifier_state_transition_whitespace(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State Whitespace: '{:?}'", c);

  return (None, false);
}

fn before_doctype_public_identifier_state_transition_quotation_mark(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State Quotation Mark: '{:?}'", c);

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_public_identifier("");
  }

  *current_state = DataState::DOCTYPEPublicIdentifierDoubleQuotedState;

  return (None, false);
}

fn before_doctype_public_identifier_state_transition_apostrophe(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State Apostrophe: '{:?}'", c);

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_public_identifier("");
  }

  *current_state = DataState::DOCTYPEPublicIdentifierSingleQuotedState;

  return (None, false);
}

fn before_doctype_public_identifier_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State Greater Than Sign: '{:?}'", c);

  missing_doctype_public_identifier_parse_error::error(DataState::AfterDOCTYPEPublicKeywordState.to_string(), c.unwrap());

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

fn before_doctype_public_identifier_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State EOF");

  eof_in_doctype_parse_error::error(DataState::BeforeDOCTYPENameState.to_string());

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

fn before_doctype_public_identifier_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Public Identifier State Anything Else: '{:?}'", c);

  missing_quote_before_doctype_public_identifier_parse_error::error(DataState::AfterDOCTYPEPublicKeywordState.to_string(), c.unwrap());

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_force_quirks(true);
  }

  *current_state = DataState::BogusDOCTYPEState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::DoctypeToken;

  #[test]
  fn test_before_doctype_public_identifier_state_transition_whitespace() {
    const C: Option<char> = Some('\u{0009}');
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPEPublicIdentifierState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_public_identifier_state_transition_quotation_mark() {
    const C: Option<char> = Some('\"');
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          public_identifier: Some("".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPEPublicIdentifierDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_public_identifier_state_transition_apostrophe() {
    const C: Option<char> = Some('\'');
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          public_identifier: Some("".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPEPublicIdentifierSingleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_public_identifier_state_transitiongreater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
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
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_public_identifier_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
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
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPEPublicIdentifierState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_public_identifier_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::BeforeDOCTYPEPublicIdentifierState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("abc".to_string()),
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, true); 
    let result = before_doctype_public_identifier_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
