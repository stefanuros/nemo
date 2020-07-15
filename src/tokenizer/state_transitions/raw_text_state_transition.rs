use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn raw_text_state_transition(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Raw Text State, c: '{}'", c.unwrap());

  match c {
    Some('\u{003C}') => raw_text_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => raw_text_state_transition_null(c),
    None => raw_text_state_transition_eof(),
    _ => raw_text_state_transition_anything_else(c),
  }
}

fn raw_text_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Raw Text State Less Than Sign: {}", c.unwrap());

  *current_state = DataState::RAWTEXTLessThanSignState;

  return (None, false);
}

fn raw_text_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Raw Text State Null: '{}'", c.unwrap());
  unexpected_null_character_parse_error::error(DataState::RAWTEXTState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn raw_text_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Raw Text State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn raw_text_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Raw Text State Anything Else: {}", c.unwrap());

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}
