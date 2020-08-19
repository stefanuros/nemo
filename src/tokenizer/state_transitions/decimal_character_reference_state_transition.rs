use crate::errors::tokenizer_errors::missing_semicolon_after_character_reference_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn decimal_character_reference_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Decimal Character Reference State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_digit() => decimal_character_reference_state_transition_ascii_digit(c, character_reference_code),
    Some('\u{003B}') => decimal_character_reference_state_transition_semicolon(c, current_state),
    _ => decimal_character_reference_state_transition_anything_else(c, current_state)
  }
}

fn decimal_character_reference_state_transition_ascii_digit(
  c: Option<char>,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Decimal Character Reference State ASCII Digit: '{:?}'", c);

  *character_reference_code *= 10;
  *character_reference_code += (c.unwrap() as u32) - 0x30;

  return (None, false);
}

fn decimal_character_reference_state_transition_semicolon(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Decimal Character Reference State Semicolon: '{:?}'", c);

  *current_state = DataState::NumericCharacterReferenceEndState;

  return (None, false);
}


fn decimal_character_reference_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Decimal Character Reference State Anything Else: '{:?}'", c);

  missing_semicolon_after_character_reference_parse_error::error(DataState::DecimalCharacterReferenceStartState.to_string(), c);

  *current_state = DataState::NumericCharacterReferenceEndState;

  return (None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decimal_character_reference_state_transition_ascii_digit() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::DecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = decimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DecimalCharacterReferenceState, current_state);
    assert_eq!((10 * 10) + 5, character_reference_code)
  }

  #[test]
  fn test_decimal_character_reference_state_transition_semicolon() {
    const C: Option<char> = Some(';');
    let mut current_state: DataState = DataState::DecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = decimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NumericCharacterReferenceEndState, current_state);
    assert_eq!(10, character_reference_code)
  }

  #[test]
  fn test_decimal_character_reference_state_transition_anything_else() {
    const C: Option<char> = Some('X');
    let mut current_state: DataState = DataState::DecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = decimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NumericCharacterReferenceEndState, current_state);
    assert_eq!(10, character_reference_code)
  }
}
