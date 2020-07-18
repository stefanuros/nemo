use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn rawtext_state_transition(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT State, c: '{:?}'", c);

  match c {
    Some('\u{003C}') => rawtext_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => rawtext_state_transition_null(c),
    None => rawtext_state_transition_eof(),
    _ => rawtext_state_transition_anything_else(c),
  }
}

fn rawtext_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT State Less Than Sign: '{:?}'", c);

  *current_state = DataState::RAWTEXTLessThanSignState;

  return (None, false);
}

fn rawtext_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT State Null: '{:?}'", c);
  unexpected_null_character_parse_error::error(DataState::RAWTEXTState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn rawtext_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn rawtext_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT State Anything Else: '{:?}'", c);

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rawtext_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::RAWTEXTState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rawtext_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTLessThanSignState, current_state);
  }

  #[test]
  fn test_rawtext_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::RAWTEXTState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
    let result = rawtext_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
  }

  #[test]
  fn test_rawtext_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::RAWTEXTState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = rawtext_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
  }

  #[test]
  fn test_rawtext_state_transition_anything_else() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::RAWTEXTState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = rawtext_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
  }
}
