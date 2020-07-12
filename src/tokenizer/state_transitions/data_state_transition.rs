use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;

pub fn data_state_transition(c: char) {
  print!("Data State: '{}'", c);

  match c {
    '\u{0026}' => data_state_transition_ampersand(c),
    '\u{003C}' => data_state_transition_less_than_sign(c),
    '\u{0000}' => data_state_transition_null(c),
    // '\u{0026}' => data_state_transition_eof(c),
    _ => data_state_transition_anything_else(c),
  }
}

fn data_state_transition_ampersand(c: char) {}

fn data_state_transition_less_than_sign(c: char) {}

fn data_state_transition_null(c: char) {
  unexpected_null_character_parse_error::error();
}

fn data_state_transition_eof(c: char) {}

fn data_state_transition_anything_else(c: char) {}
