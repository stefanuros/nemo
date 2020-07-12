use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;

pub fn rcdata_state_transition(c: char) {
  print!("RCDATA State: '{}'", c);

  match c {
    '\u{0026}' => rcdata_state_transition_ampersand(c),
    '\u{003C}' => rcdata_state_transition_less_than_sign(c),
    '\u{0000}' => rcdata_state_transition_null(c),
    // '\u{0026}' => rcdata_state_transition_eof(c),
    _ => rcdata_state_transition_anything_else(c),
  }
}

fn rcdata_state_transition_ampersand(c: char) {}

fn rcdata_state_transition_less_than_sign(c: char) {}

fn rcdata_state_transition_null(c: char) {
  unexpected_null_character_parse_error::error();
}

fn rcdata_state_transition_eof(c: char) {}

fn rcdata_state_transition_anything_else(c: char) {}
