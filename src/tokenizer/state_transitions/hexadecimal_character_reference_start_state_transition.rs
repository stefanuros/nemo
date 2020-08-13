use crate::errors::tokenizer_errors::absence_of_digits_in_numeric_character_reference_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn hexadecimal_character_reference_start_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference Start State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_hexdigit() => hexadecimal_character_reference_start_state_transition_ascii_hexdigit(c, current_state),
    _ => hexadecimal_character_reference_start_state_transition_anything_else(c, current_state, return_state, current_token, temporary_buffer)
  }
}

fn hexadecimal_character_reference_start_state_transition_ascii_hexdigit(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference Start State ASCII Hexdigit: '{:?}'", c);

  *current_state = DataState::HexidecimalCharacterReferenceState;

  return (None, true);
}

fn hexadecimal_character_reference_start_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
) -> (Option<Vec<Token>>, bool) {
  println!("Hexadecimal Character Reference Start State Anything Else: '{:?}'", c);

  absence_of_digits_in_numeric_character_reference_parse_error::error(DataState::HexadecimalCharacterReferenceStartState.to_string(), c);

  let emitted_tokens = flush_code_points(return_state, current_token, temporary_buffer);

  *current_state = return_state.clone();

  return (emitted_tokens, true);
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
  use crate::types::tokenizer_types::token_types::{
    Attribute,
    TagToken
  };
  
  #[test]
  fn test_hexadecimal_character_reference_start_state_transition_ascii_hexdigit() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::HexadecimalCharacterReferenceStartState;
    let mut return_state: DataState = DataState::AttributeValueUnquotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "abc".to_string();

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = hexadecimal_character_reference_start_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::HexidecimalCharacterReferenceState, current_state);
    assert_eq!(DataState::AttributeValueUnquotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("abc".to_string(), temporary_buffer);
  }

  #[test]
  fn test_hexadecimal_character_reference_start_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::HexadecimalCharacterReferenceStartState;
    let mut return_state: DataState = DataState::AttributeValueUnquotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "abc".to_string();

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyzabc".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = hexadecimal_character_reference_start_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueUnquotedState, current_state);
    assert_eq!(DataState::AttributeValueUnquotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("abc".to_string(), temporary_buffer);
  }
}
