use crate::errors::tokenizer_errors::missing_semicolon_after_character_reference_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn hexadecimal_character_reference_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_digit() => hexadecimal_character_reference_state_transition_ascii_digit(c, character_reference_code),
    Some(x) if is_ascii_upper_hex_digit(&x) => hexadecimal_character_reference_state_transition_ascii_upper_hex_digit(c, character_reference_code),
    Some(x) if is_ascii_lower_hex_digit(&x) => hexadecimal_character_reference_state_transition_ascii_lower_hex_digit(c, character_reference_code),
    Some('\u{003B}') => hexadecimal_character_reference_state_transition_semicolon(c, current_state),
    _ => hexadecimal_character_reference_state_transition_anything_else(c, current_state)
  }
}

fn is_ascii_upper_hex_digit(c: &char) -> bool {
  return c >= &'\u{0041}' && c <= &'\u{0046}';
}

fn is_ascii_lower_hex_digit(c: &char) -> bool {
  return c >= &'\u{0061}' && c <= &'\u{0066}';
}

fn hexadecimal_character_reference_state_transition_ascii_digit(
  c: Option<char>,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State ASCII Digit: '{:?}'", c);

  *character_reference_code *= 16;
  *character_reference_code += (c.unwrap() as u32) - 0x30;

  return (None, false);
}

fn hexadecimal_character_reference_state_transition_ascii_upper_hex_digit(
  c: Option<char>,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State ASCII Upper Hex Digit: '{:?}'", c);

  *character_reference_code *= 16;
  *character_reference_code += (c.unwrap() as u32) - 0x37;

  return (None, false);
}

fn hexadecimal_character_reference_state_transition_ascii_lower_hex_digit(
  c: Option<char>,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State ASCII Lower Hex Digit: '{:?}'", c);

  *character_reference_code *= 16;
  *character_reference_code += (c.unwrap() as u32) - 0x57;

  return (None, false);
}

fn hexadecimal_character_reference_state_transition_semicolon(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State Semicolon: '{:?}'", c);

  *current_state = DataState::NumericCharacterReferenceEndState;

  return (None, false);
}


fn hexadecimal_character_reference_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference State Anything Else: '{:?}'", c);

  missing_semicolon_after_character_reference_parse_error::error(DataState::DecimalCharacterReferenceStartState.to_string(), c);

  *current_state = DataState::NumericCharacterReferenceEndState;

  return (None, true);
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_ascii_upper_and_lower_hex_digit_functions() {
    assert_eq!(true, is_ascii_lower_hex_digit(&'a'));
    assert_eq!(true, is_ascii_lower_hex_digit(&'f'));
    assert_eq!(false, is_ascii_lower_hex_digit(&'g'));
    assert_eq!(false, is_ascii_lower_hex_digit(&'G'));
    assert_eq!(false, is_ascii_lower_hex_digit(&'5'));
    assert_eq!(false, is_ascii_lower_hex_digit(&'F'));
    assert_eq!(false, is_ascii_lower_hex_digit(&'A'));

    assert_eq!(true, is_ascii_upper_hex_digit(&'A'));
    assert_eq!(true, is_ascii_upper_hex_digit(&'F'));
    assert_eq!(false, is_ascii_upper_hex_digit(&'G'));
    assert_eq!(false, is_ascii_upper_hex_digit(&'g'));
    assert_eq!(false, is_ascii_upper_hex_digit(&'5'));
    assert_eq!(false, is_ascii_upper_hex_digit(&'a'));
    assert_eq!(false, is_ascii_upper_hex_digit(&'f'));
  }

  #[test]
  fn test_hexadecimal_character_reference_state_transition_ascii_digit() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::HexidecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = hexadecimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::HexidecimalCharacterReferenceState, current_state);
    assert_eq!((10 * 16) + 5, character_reference_code)
  }

  #[test]
  fn test_hexadecimal_character_reference_state_transition_ascii_upper_hex_digit() {
    const C: Option<char> = Some('C');
    let mut current_state: DataState = DataState::HexidecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = hexadecimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::HexidecimalCharacterReferenceState, current_state);
    assert_eq!((10 * 16) + 12, character_reference_code)
  }

  #[test]
  fn test_hexadecimal_character_reference_state_transition_ascii_lower_hex_digit() {
    const C: Option<char> = Some('c');
    let mut current_state: DataState = DataState::HexidecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = hexadecimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::HexidecimalCharacterReferenceState, current_state);
    assert_eq!((10 * 16) + 12, character_reference_code)
  }

  #[test]
  fn test_hexadecimal_character_reference_state_transition_semicolon() {
    const C: Option<char> = Some(';');
    let mut current_state: DataState = DataState::HexidecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = hexadecimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NumericCharacterReferenceEndState, current_state);
    assert_eq!(10, character_reference_code)
  }

  #[test]
  fn test_hexadecimal_character_reference_state_transition_anything_else() {
    const C: Option<char> = Some('X');
    let mut current_state: DataState = DataState::HexidecimalCharacterReferenceState;
    let mut character_reference_code: u32 = 10;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = hexadecimal_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NumericCharacterReferenceEndState, current_state);
    assert_eq!(10, character_reference_code)
  }
}
