use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn numeric_character_reference_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String,
  character_reference_code: &mut i32
) -> (Option<Vec<Token>>, bool) {
  println!("Numeric Character Reference State c: '{:?}'", c);

  *character_reference_code = 0;

  return match c {
    Some('\u{0078}') |
    Some('\u{0058}') => numeric_character_reference_state_transition_latin_letter_x(c, current_state, temporary_buffer),
    _ => numeric_character_reference_state_transition_anything_else(c, current_state)
  }
}

fn numeric_character_reference_state_transition_latin_letter_x(
  c: Option<char>,
  current_state: &mut DataState,
  temporary_buffer: &mut String,
) -> (Option<Vec<Token>>, bool) {
  println!("Numeric Character Reference State Latin Letter X: '{:?}'", c);

  temporary_buffer.push(c.unwrap());

  *current_state = DataState::HexadecimalCharacterReferenceStartState;

  return (None, false);
}

fn numeric_character_reference_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Numeric Character Reference State Anything Else: '{:?}'", c);

  *current_state = DataState::DecimalCharacterReferenceStartState;

  return (None, true);
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_numeric_character_reference_state_transition_latin_letter_x() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::NumericCharacterReferenceState;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: i32 = 1;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = numeric_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::HexadecimalCharacterReferenceStartState, current_state);
    assert_eq!("abcx".to_string(), temporary_buffer);
    assert_eq!(0, character_reference_code);
  }

  #[test]
  fn test_numeric_character_reference_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::NumericCharacterReferenceState;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: i32 = 1;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = numeric_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DecimalCharacterReferenceStartState, current_state);
    assert_eq!("abc".to_string(), temporary_buffer);
    assert_eq!(0, character_reference_code);
  }
}
