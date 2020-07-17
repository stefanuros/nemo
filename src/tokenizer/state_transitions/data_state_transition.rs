use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn data_state_transition(
  c: Option<char>, 
  current_state: &mut DataState, 
  return_state: &mut DataState
  // TODO This can be removed cause its not used in this function
  // create_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Data State, c: '{:?}'", c);

  match c {
    Some('\u{0026}') => data_state_transition_ampersand(c, current_state, return_state),
    Some('\u{003C}') => data_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => data_state_transition_null(c),
    None => data_state_transition_eof(),
    _ => data_state_transition_anything_else(c),
  }
}

fn data_state_transition_ampersand(
  c: Option<char>, 
  current_state: &mut DataState, 
  return_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Data State Ampersand: '{:?}'", c);

  *return_state = DataState::DataState;
  *current_state = DataState::CharacterReferenceState;

  return (None, false);
}

fn data_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Data State Less Than Sign: '{:?}'", c);
  *current_state = DataState::TagOpenState;

  return (None, false);
}

fn data_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Data State Null: '{:?}'", c);
  unexpected_null_character_parse_error::error(DataState::DataState.to_string(), c.unwrap());
  
  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

fn data_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Data State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn data_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Data State Anything Else: '{:?}'", c);

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_data_state_transition_ampersand() {
    const C: Option<char> = Some('&');
    let mut current_state: DataState = DataState::DataState;
    let mut return_state: DataState = DataState::DataState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = data_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::CharacterReferenceState, current_state);
    assert_eq!(DataState::DataState, return_state);
  }

  #[test]
  fn test_data_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::DataState;
    let mut return_state: DataState = DataState::DataState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = data_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagOpenState, current_state);
    assert_eq!(DataState::DataState, return_state);
  }

  #[test]
  fn test_data_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::DataState;
    let mut return_state: DataState = DataState::DataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = data_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
  }

  #[test]
  fn test_data_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::DataState;
    let mut return_state: DataState = DataState::DataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = data_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
  }

  #[test]
  fn test_data_state_transition_anything_else() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::DataState;
    let mut return_state: DataState = DataState::DataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = data_state_transition(C, &mut current_state, &mut return_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
  }
}
