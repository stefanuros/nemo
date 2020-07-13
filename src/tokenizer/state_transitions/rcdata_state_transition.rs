use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::data_states::DataState::RCDataState;

pub fn rcdata_state_transition(c: Option<char>) {
  println!("RCDATA State, c: '{}'", c.unwrap());

  match c {
    Some('\u{0026}') => rcdata_state_transition_ampersand(c),
    Some('\u{003C}') => rcdata_state_transition_less_than_sign(c),
    Some('\u{0000}') => rcdata_state_transition_null(c),
    None => rcdata_state_transition_eof(),
    _ => rcdata_state_transition_anything_else(c),
  }
}

fn rcdata_state_transition_ampersand(c: Option<char>) {
  println!("RCDATA State Ampersand: {}", c.unwrap());
}

fn rcdata_state_transition_less_than_sign(c: Option<char>) {
  println!("RCDATA State Less Than Sign: {}", c.unwrap());
}

fn rcdata_state_transition_null(c: Option<char>) {
  unexpected_null_character_parse_error::error(RCDataState.to_string(), c.unwrap());
}

fn rcdata_state_transition_eof() {
  println!("RCDATA State EOF");
}

fn rcdata_state_transition_anything_else(c: Option<char>) {
  println!("RCDATA State Anything Else: {}", c.unwrap());
}
