use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn character_reference_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Character Reference State c: '{:?}'", c);

  *temporary_buffer = "&".to_string();

  return match c {
    Some(x) if x.is_ascii_alphanumeric() => character_reference_state_transition_ascii_alphanumeric(c, current_state),
    Some('\u{0023}') => character_reference_state_transition_number_sign(c, current_state, temporary_buffer),
    _ => character_reference_state_transition_anything_else(c, current_state, return_state, current_token, temporary_buffer)
  }
}

fn character_reference_state_transition_ascii_alphanumeric(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Character Reference State Ascii Alphanumeric: '{:?}'", c);

  *current_state = DataState::NamedCharacterReferenceState;

  return (None, true);
}

fn character_reference_state_transition_number_sign(
  c: Option<char>,
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Character Reference State Number Sign: '{:?}'", c);

  temporary_buffer.push(c.unwrap());

  *current_state = DataState::NumericCharacterReferenceState;

  return (None, false);
}

fn character_reference_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Character Reference State Anything Else: '{:?}'", c);

  *current_state = return_state.clone();

  let emitted_tokens = flush_code_points(return_state, current_token, temporary_buffer);

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

  let is_consumed_as_attribute = return_state == &DataState::AttributeValueDoubleQuotedState ||
  return_state == &DataState::AttributeValueSingleQuotedState ||
  return_state == &DataState::AttributeValueUnquotedState;

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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::{
    Attribute,
    TagToken
  };
  
  #[test]
  fn test_character_reference_state_transition_ascii_alphanumeric_alpha() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
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
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NamedCharacterReferenceState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
  }

  #[test]
  fn test_character_reference_state_transition_ascii_alphanumeric_numeric() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::CharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
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
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NamedCharacterReferenceState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
  }

  #[test]
  fn test_character_reference_state_transition_number_sign() {
    const C: Option<char> = Some('#');
    let mut current_state: DataState = DataState::CharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
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
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::NumericCharacterReferenceState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&#".to_string(), temporary_buffer);
  }

  #[test]
  fn test_character_reference_state_transition_anything_else_consumed_as_attribute() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::CharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
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
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
          value: "xyz&".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
  }

  #[test]
  fn test_character_reference_state_transition_anything_else_not_consumed_as_attribute() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::CharacterReferenceState;
    let mut return_state: DataState = DataState::DataState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
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
          name: "name".to_string(),
          value: "value".to_string(),
          ..Attribute::default()
        },
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('&')
      ]), 
      true
    );
    let result = character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
  }
}
