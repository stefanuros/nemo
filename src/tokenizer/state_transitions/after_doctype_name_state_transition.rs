use crate::errors::tokenizer_errors::{
  eof_in_doctype_parse_error,
  invalid_character_sequence_after_doctype_name_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn after_doctype_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => after_doctype_name_state_transition_whitespace(c),
    Some('\u{003E}') => after_doctype_name_state_transition_greater_than_sign(c, current_state, current_token),
    None => after_doctype_name_state_transition_eof(current_token),
    _ => after_doctype_name_state_transition_anything_else(c, current_state, current_token, iter)
  }
}

fn after_doctype_name_state_transition_whitespace(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Whitespace: '{:?}'", c);

  return (None, false);
}

fn after_doctype_name_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn after_doctype_name_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State EOF");

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

fn after_doctype_name_state_transition_anything_else(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Anything Else: '{:?}'", c);

  // If c is none, it won't match anything
  if let None = c {
    return after_doctype_name_state_transition_anything_else_other(c, current_state, current_token);
  }
  
  let mut peek = c.unwrap().to_string();

  // Loop to 5 because we want to loop 6 times  and c counts as the first one
  for _ in 0..5 {
    match iter.peek() {
      Some(x) => peek.push(x.clone()),
      None => break 
    };

    println!("After DOCTYPE Name State c: '{:?}'", peek);

    if peek.to_ascii_uppercase() == "PUBLIC" {
      return after_doctype_name_state_transition_anything_else_public(c, current_state, iter);
    }
    else if peek.to_ascii_uppercase() == "SYSTEM" {
      return after_doctype_name_state_transition_anything_else_system(c, current_state, iter);
    }
  }
  
  return after_doctype_name_state_transition_anything_else_other(c, current_state, current_token);
}

fn after_doctype_name_state_transition_anything_else_public(
  c: Option<char>, 
  current_state: &mut DataState,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Anything Else PUBLIC: '{:?}'", c);

  // Skip the next 5 (5 + c is 6 characters to consume)
  iter.nth(4);

  *current_state = DataState::AfterDOCTYPEPublicKeywordState;

  return (None, false);
}

fn after_doctype_name_state_transition_anything_else_system(
  c: Option<char>, 
  current_state: &mut DataState,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Anything Else SYSTEM: '{:?}'", c);

  // Skip the next 5 (5 + c is 6 characters to consume)
  iter.nth(4);

  *current_state = DataState::AfterDOCTYPESystemKeywordState;

  return (None, false);
}

fn after_doctype_name_state_transition_anything_else_other(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After DOCTYPE Name State Anything Else Other: '{:?}'", c);

  invalid_character_sequence_after_doctype_name_parse_error::error(DataState::AfterDOCTYPENameState.to_string(), c.unwrap());

  if let Some(Token::DOCTYPE(ref mut doctype_token)) = current_token {
    doctype_token.set_force_quirks(true);
  }

  *current_state = DataState::BogusDOCTYPEState;

  return (None, true);
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::DoctypeToken;

  #[test]
  fn test_after_doctype_name_state_transition_whitespace() {
    const C: Option<char> = Some('\u{0020}');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("A".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("A".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
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
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("A".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
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
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPENameState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_anything_else_other() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("A".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_anything_else_public() {
    const C: Option<char> = Some('p');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("UbLiCX".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPEPublicKeywordState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('X'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_anything_else_system() {
    const C: Option<char> = Some('s');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("YsTeMX".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterDOCTYPESystemKeywordState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('X'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_anything_else_not_system() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("sYsTeMX".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('s'));
  }

  #[test]
  fn test_after_doctype_name_state_transition_anything_else_full_match_system() {
    const C: Option<char> = Some('s');
    let mut current_state: DataState = DataState::AfterDOCTYPENameState;
    let mut current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          ..DoctypeToken::default()
        }
      )
    );
    let mut iter = itertools::multipeek("ysteX".chars());
    
    let expected_current_token: Option<Token> = Some(
      Token::DOCTYPE(
        DoctypeToken {
          force_quirks: true,
          ..DoctypeToken::default()
        }
      )
    );
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = after_doctype_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusDOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('y'));
  }
}
