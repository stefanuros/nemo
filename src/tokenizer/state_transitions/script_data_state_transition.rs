use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_state_transition(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data State, c: '{:?}'", c);

  return match c {
    Some('\u{003C}') => script_data_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => script_data_state_transition_null(c),
    None => script_data_state_transition_eof(),
    _ => script_data_state_transition_anything_else(c),
  }
}

fn script_data_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data State Less Than Sign: '{:?}'", c);

  *current_state = DataState::ScriptDataLessThanSignState;

  return (None, false);
}

fn script_data_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Script Data State Null: '{:?}'", c);
  unexpected_null_character_parse_error::error(DataState::ScriptDataState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn script_data_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Script Data State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn script_data_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Script Data State Anything Else: '{:?}'", c);

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::ScriptDataState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = script_data_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataLessThanSignState, current_state);
  }

  #[test]
  fn test_script_data_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::ScriptDataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
    let result = script_data_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataState, current_state);
  }

  #[test]
  fn test_script_data_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::ScriptDataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = script_data_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataState, current_state);
  }

  #[test]
  fn test_script_data_state_transition_anything_else() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::ScriptDataState;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = script_data_state_transition(C, &mut current_state);

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataState, current_state);
  }
}
