use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  missing_doctype_name_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token,
  token_types::DoctypeToken
};

pub fn before_doctype_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => before_doctype_name_state_transition_whitespace(c),
    Some(x) if x.is_ascii_uppercase() => before_doctype_name_state_transition_ascii_upper(c, current_state, current_token),
    Some('\u{0000}') => before_doctype_name_state_transition_null(c, current_state, current_token),
    Some('\u{003E}') => before_doctype_name_state_transition_greater_than_sign(c, current_state),
    None => before_doctype_name_state_transition_eof(),
    _ => before_doctype_name_state_transition_anything_else(c, current_state, current_token)
  }
}

fn before_doctype_name_state_transition_whitespace(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State Whitespace: '{:?}'", c);

  return (None, false);
}

fn before_doctype_name_state_transition_ascii_upper(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State Ascii Upper: '{:?}'", c);

  *current_token = Some(Token::DOCTYPE(
    DoctypeToken::new_c(c.unwrap().to_ascii_lowercase())
  ));

  *current_state = DataState::DOCTYPENameState;

  return (None, false);
}

fn before_doctype_name_state_transition_null(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State Null: '{:?}'", c);

  *current_token = Some(Token::DOCTYPE(
    DoctypeToken::new("�")
  ));

  *current_state = DataState::DOCTYPENameState;

  return (None, false);
}

fn before_doctype_name_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State Greater Than Sign: '{:?}'", c);

  missing_doctype_name_parse_error::error(DataState::BeforeDOCTYPENameState.to_string(), c.unwrap());

  *current_state = DataState::DataState;

  return (
    Some(vec![
      Token::DOCTYPE(          
        DoctypeToken {
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    ]), 
    false
  );
}

fn before_doctype_name_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State EOF");

  eof_in_doctype_parse_error::error(DataState::BeforeDOCTYPENameState.to_string());

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

fn before_doctype_name_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before DOCTYPE Name State Anything Else: '{:?}'", c);

  *current_token = Some(Token::DOCTYPE(
    DoctypeToken::new_c(c.unwrap())
  ));

  *current_state = DataState::DOCTYPENameState;

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_before_doctype_name_state_transition_whitespace() {
    const C: Option<char> = Some('\u{000A}');
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = None;
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_name_state_transition_ascii_upper() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("a".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_name_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("�".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = None;
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::DOCTYPE(
          DoctypeToken {
            force_quirks: true,
            ..DoctypeToken::default()
          }
        )
      ]), 
      false
    );
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_name_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = None;
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
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeDOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_doctype_name_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::BeforeDOCTYPENameState;
    let mut current_token: Option<Token> = None;
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          name: Some("g".to_string()),
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
