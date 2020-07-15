use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn rcdata_state_transition(
  c: Option<char>,
  current_state: &mut DataState, 
  return_state: &mut DataState,
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State, c: '{}'", c.unwrap());

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
  println!("RCDATA State Ampersand: {}", c.unwrap());

  *return_state = DataState::RCDataState;
  *current_state = DataState::CharacterReferenceState;

  return (None, false);
}

fn rcdata_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Less Than Sign: {}", c.unwrap());

  *current_state = DataState::RCDATALessThanSignState;

  return (None, false);
}

fn rcdata_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Null: '{}'", c.unwrap());
  unexpected_null_character_parse_error::error(DataState::RCDataState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn rcdata_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn rcdata_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA State Anything Else: {}", c.unwrap());

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}
