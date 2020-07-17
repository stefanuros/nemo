use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn rcdata_state_transition(
  c: Option<char>,
  current_state: &mut DataState, 
  return_state: &mut DataState,
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State, c: '{:?}'", c);

  match c {
    Some('\u{0026}') => rcdata_state_transition_ampersand(c, current_state, return_state),
    Some('\u{003C}') => rcdata_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => rcdata_state_transition_null(c),
    None => rcdata_state_transition_eof(),
    _ => rcdata_state_transition_anything_else(c),
  }
}

fn rcdata_state_transition_ampersand(
  c: Option<char>,
  current_state: &mut DataState, 
  return_state: &mut DataState,
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Ampersand: '{:?}'", c);

  *return_state = DataState::RCDataState;
  *current_state = DataState::CharacterReferenceState;

  return (None, false);
}

fn rcdata_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Less Than Sign: '{:?}'", c);

  *current_state = DataState::RCDATALessThanSignState;

  return (None, false);
}

fn rcdata_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Null: '{:?}'", c);
  unexpected_null_character_parse_error::error(DataState::RCDataState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn rcdata_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn rcdata_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Anything Else: '{:?}'", c);

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rcdata_state_transition_ampersand() {
    const C: Option<char> = Some('&');
    let mut current_state: DataState = DataState::RCDataState;
    let mut return_state: DataState = DataState::default();

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rcdata_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::CharacterReferenceState, current_state);
    assert_eq!(DataState::RCDataState, return_state);
  }

  #[test]
  fn test_rcdata_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::RCDataState;
    let mut return_state: DataState = DataState::default();

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rcdata_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDATALessThanSignState, current_state);
    assert_eq!(DataState::default(), return_state);
  }

  #[test]
  fn test_rcdata_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::RCDataState;
    let mut return_state: DataState = DataState::default();

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
    let result = rcdata_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDataState, current_state);
    assert_eq!(DataState::default(), return_state);
  }

  #[test]
  fn test_rcdata_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::RCDataState;
    let mut return_state: DataState = DataState::default();

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = rcdata_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDataState, current_state);
    assert_eq!(DataState::default(), return_state);
  }

  #[test]
  fn test_rcdata_state_transition_anything_else() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::RCDataState;
    let mut return_state: DataState = DataState::default();

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = rcdata_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDataState, current_state);
    assert_eq!(DataState::default(), return_state);
  }
}
