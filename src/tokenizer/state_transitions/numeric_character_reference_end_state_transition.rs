use crate::errors::tokenizer_errors::{
  null_character_reference_parse_error,
  character_reference_outside_unicode_range_parse_error,
  surrogate_character_reference_parse_error,
  noncharacter_character_reference_parse_error,
  control_character_reference_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token,
  NUMERIC_CHARACTER_REFERENCE
};
use std::char;

pub fn numeric_character_reference_end_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
  character_reference_code: &mut u32
) -> (Option<Vec<Token>>, bool) {
  println!("Numeric Character Reference End State c: '{:?}'", c);

  match character_reference_code.clone() {
    0x00 => numeric_character_reference_end_state_transition_null(character_reference_code),
    x if x > 0x10FFFF => numeric_character_reference_end_state_transition_out_of_range(character_reference_code),
    x if is_surrogate(x) => numeric_character_reference_end_state_transition_surrogate(character_reference_code),
    x if is_noncharacter(x) => numeric_character_reference_end_state_transition_noncharacter(character_reference_code),
    x if is_control(x) => numeric_character_reference_end_state_transition_control(character_reference_code),
    _ => ()
  };

  // Make a code point with the current character reference code
  // NOTE: This function (from_u32) is experimental at the time this was done
  *temporary_buffer = match char::from_u32(character_reference_code.clone()) {
    Some(x) => x.to_string(),
    None => '\u{FFFD}'.to_string()
  };

  let emitted_tokens: Option<Vec<Token>> = flush_code_points(return_state, current_token, temporary_buffer);

  *current_state = return_state.clone();

  return (emitted_tokens, false);
}

fn numeric_character_reference_end_state_transition_null(character_reference_code: &mut u32) {
  println!("Numeric Character Reference End State Null c: '{:?}'", character_reference_code);
  null_character_reference_parse_error::error(DataState::NumericCharacterReferenceEndState.to_string(), character_reference_code.clone());

  *character_reference_code = 0xFFFD;
}

fn numeric_character_reference_end_state_transition_out_of_range(character_reference_code: &mut u32) {
  println!("Numeric Character Reference End State Out of Range c: '{:?}'", character_reference_code);
  character_reference_outside_unicode_range_parse_error::error(DataState::NumericCharacterReferenceEndState.to_string(), character_reference_code.clone());

  *character_reference_code = 0xFFFD;
}

fn numeric_character_reference_end_state_transition_surrogate(character_reference_code: &mut u32) {
  println!("Numeric Character Reference End State Surrogate c: '{:?}'", character_reference_code);
  surrogate_character_reference_parse_error::error(DataState::NumericCharacterReferenceEndState.to_string(), character_reference_code.clone());

  *character_reference_code = 0xFFFD;
}

fn numeric_character_reference_end_state_transition_noncharacter(character_reference_code: &mut u32) {
  println!("Numeric Character Reference End State Noncharacter c: '{:?}'", character_reference_code);
  noncharacter_character_reference_parse_error::error(DataState::NumericCharacterReferenceEndState.to_string(), character_reference_code.clone());
}

fn numeric_character_reference_end_state_transition_control(character_reference_code: &mut u32) {
  println!("Numeric Character Reference End State Control c: '{:?}'", character_reference_code);
  control_character_reference_parse_error::error(DataState::NumericCharacterReferenceEndState.to_string(), character_reference_code.clone());

  // the key is a string of a hex value
  let key: String = format!("{:x}", character_reference_code.clone());

  // If character reference code is a numeric_character_reference, get the val
  // otherwise it will be None so we won't do anything with the value
  if let Some(x) = NUMERIC_CHARACTER_REFERENCE.get(&key.as_ref()) {
    *character_reference_code = x.clone();
  }
}

fn is_surrogate(character_reference_code: u32) -> bool {
  return character_reference_code >= 0xD800 && character_reference_code <= 0xDFFF;
}

fn is_noncharacter(character_reference_code: u32) -> bool {
  return match character_reference_code {
    x if x >= 0xFDD0 && x <= 0xFDEF => true,
    0xFFFE |
    0xFFFF |
    0x1FFFE |
    0x1FFFF |
    0x2FFFE |
    0x2FFFF |
    0x3FFFE |
    0x3FFFF |
    0x4FFFE |
    0x4FFFF |
    0x5FFFE |
    0x5FFFF |
    0x6FFFE |
    0x6FFFF |
    0x7FFFE |
    0x7FFFF |
    0x8FFFE |
    0x8FFFF |
    0x9FFFE |
    0x9FFFF |
    0xAFFFE |
    0xAFFFF |
    0xBFFFE |
    0xBFFFF |
    0xCFFFE |
    0xCFFFF |
    0xDFFFE |
    0xDFFFF |
    0xEFFFE |
    0xEFFFF |
    0xFFFFE |
    0xFFFFF |
    0x10FFFE |
    0x10FFFF => true,
    _ => false
  }
}

fn is_control(character_reference_code: u32) -> bool {
  let is_ascii_whitespace = match character_reference_code {
    0x0009 |
    0x000A |
    0x000C |
    0x000D |
    0x0020 => true,
    _ => false
  };

  return match character_reference_code {
    _ if is_ascii_whitespace => false,
    0x0D => true,
    // Dont need to check that its >= 0x0000 cause unsigned cant be below 0
    x if x <= 0x001F => true,
    x if x >= 0x007F && x <= 0x009F => true,
    _ => false
  };
}

fn flush_code_points(
  return_state: &DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> Option<Vec<Token>>{

  // When a state says to flush code points consumed as a character reference, 
  // it means that for each code point in the temporary buffer (in the order they 
  // were added to the buffer) user agent must append the code point from the 
  // buffer to the current attribute's value if the character reference was 
  // consumed as part of an attribute, or emit the code point as a character token 
  // otherwise.

  let is_consumed_as_attribute = is_consumed_as_attribute(return_state);

  let mut emitted_tokens: Vec<Token> = vec![];

  for code_point in temporary_buffer.chars() {
    if is_consumed_as_attribute {
      if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
        tag_token.push_to_current_attribute_value(code_point);
      }
    }
    else {
      emitted_tokens.push(
        Token::CharacterToken(code_point)
      );
    }
  }

  if emitted_tokens.len() <= 0 {
    return None;
  }

  return Some(emitted_tokens);
}

fn is_consumed_as_attribute(return_state: &DataState) -> bool {
  return return_state == &DataState::AttributeValueDoubleQuotedState ||
  return_state == &DataState::AttributeValueSingleQuotedState ||
  return_state == &DataState::AttributeValueUnquotedState;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_numeric_character_reference_end_state_transition_null() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0x00;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{FFFD}')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("�".to_string(), temporary_buffer);
    assert_eq!(0xFFFD, character_reference_code)
  }

  #[test]
  fn test_numeric_character_reference_end_state_transition_outside_unicode_range() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0x11FFFF;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{FFFD}')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("�".to_string(), temporary_buffer);
    assert_eq!(0xFFFD, character_reference_code)
  }

  #[test]
  fn test_numeric_character_reference_end_state_transition_surrogate() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0xD800;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{FFFD}')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("�".to_string(), temporary_buffer);
    assert_eq!(0xFFFD, character_reference_code)
  }

  #[test]
  fn test_numeric_character_reference_end_state_transition_noncharacter() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0xFFFF;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{FFFF}')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("\u{FFFF}".to_string(), temporary_buffer);
    assert_eq!(0xFFFF, character_reference_code)
  }

  #[test]
  fn test_numeric_character_reference_end_state_transition_control() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0x80;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('€')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("€".to_string(), temporary_buffer);
    assert_eq!(0x20AC, character_reference_code)
  }

  #[test]
  fn test_numeric_character_reference_end_state_transition_otherwise() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::NumericCharacterReferenceEndState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = None;
    let mut temporary_buffer: String = "abc".to_string();
    let mut character_reference_code: u32 = 0x050;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('P')
      ]), 
      false
    );
    let result = numeric_character_reference_end_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut character_reference_code
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(None, current_token);
    assert_eq!("P".to_string(), temporary_buffer);
    assert_eq!(0x050, character_reference_code)
  }
}
