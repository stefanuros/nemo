use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  missing_quote_before_doctype_system_identifier_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn between_doctype_public_and_system_identifiers_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => between_doctype_public_and_system_identifiers_state_transition_whitespace(c),
    Some('\u{003E}') => between_doctype_public_and_system_identifiers_state_transition_greater_than_sign(c, current_state, current_token),
    Some('\u{0022}') => between_doctype_public_and_system_identifiers_state_transition_quotation_mark(c, current_state, current_token),
    Some('\u{0027}') => between_doctype_public_and_system_identifiers_state_transition_apostrophe(c, current_state, current_token),
    None => between_doctype_public_and_system_identifiers_state_transition_eof(current_token),
    _ => between_doctype_public_and_system_identifiers_state_transition_anything_else(c, current_state, current_token)
  }
}

fn between_doctype_public_and_system_identifiers_state_transition_whitespace(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State Whitespace: '{:?}'", c);

  return (None, false);
}

fn between_doctype_public_and_system_identifiers_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn between_doctype_public_and_system_identifiers_state_transition_quotation_mark(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State Quotation Mark: '{:?}'", c);

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_system_identifier("");
  }

  *current_state = DataState::DOCTYPESystemIdentifierDoubleQuotedState;

  return (None, false);
}

fn between_doctype_public_and_system_identifiers_state_transition_apostrophe(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State Apostrophe: '{:?}'", c);

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_system_identifier("");
  }

  *current_state = DataState::DOCTYPESystemIdentifierSingleQuotedState;

  return (None, false);
}

fn between_doctype_public_and_system_identifiers_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State EOF");

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

fn between_doctype_public_and_system_identifiers_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Between DOCTYPE Public And System Identifiers State Anything Else: '{:?}'", c);

  missing_quote_before_doctype_system_identifier_parse_error::error(DataState::AfterDOCTYPEPublicIdentifierState.to_string(), c.unwrap());

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
  fn test_between_doctype_public_and_system_identifiers_state_transition_whitespace() {
    const C: Option<char> = Some(' ');
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
  
  #[test]
  fn test_between_doctype_public_and_system_identifiers_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_between_doctype_public_and_system_identifiers_state_transition_quotation_mark() {
    const C: Option<char> = Some('\"');
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
          system_identifier: Some("".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPESystemIdentifierDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_between_doctype_public_and_system_identifiers_state_transition_apostrophe() {
    const C: Option<char> = Some('\'');
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
          system_identifier: Some("".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPESystemIdentifierSingleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_between_doctype_public_and_system_identifiers_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_between_doctype_public_and_system_identifiers_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::BetweenDOCTYPEPublicAndSystemIdentifiersState;
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
    let result = between_doctype_public_and_system_identifiers_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
