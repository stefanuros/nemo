use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn plaintext_state_transition(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("PLAINTEXT State, c: '{:?}'", c);

  match c {
    Some('\u{0000}') => plaintext_state_transition_null(c),
    None => plaintext_state_transition_eof(),
    _ => plaintext_state_transition_anything_else(c),
  }
}

fn plaintext_state_transition_null(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("PLAINTEXT State Null: '{:?}'", c);
  unexpected_null_character_parse_error::error(DataState::PLAINTEXTState.to_string(), c.unwrap());

  return (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
}

fn plaintext_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("PLAINTEXT State EOF");

  return (Some(vec![Token::EOFToken]), false);
}

fn plaintext_state_transition_anything_else(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("PLAINTEXT State Anything Else: '{:?}'", c);

  return (Some(vec![Token::CharacterToken(c.unwrap())]), false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_plaintext_state_transition_null() {
    const C: Option<char> = Some('\0');

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken('\u{FFFD}')]), false);
    let result = plaintext_state_transition(C);

    assert_eq!(expected, result);
  }

  #[test]
  fn test_plaintext_state_transition_eof() {
    const C: Option<char> = None;

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = plaintext_state_transition(C);

    assert_eq!(expected, result);
  }

  #[test]
  fn test_plaintext_state_transition_anything_else() {
    const C: Option<char> = Some('x');

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::CharacterToken(C.unwrap())]), false);
    let result = plaintext_state_transition(C);

    assert_eq!(expected, result);
  }
}
