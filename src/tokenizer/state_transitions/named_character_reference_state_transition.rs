use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token,
  NAMED_CHARACTER_REFERENCE
};
use crate::errors::tokenizer_errors::missing_semicolon_after_character_reference_parse_error;

pub fn named_character_reference_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("Named Character Reference State c: '{:?}'", c);

  let mut is_last_semicolon: bool = false;
  let mut current_key: String = temporary_buffer.clone();

  // This keeps track of the longest match
  let mut longest_match: Option<String> = None;

  // Count the number of iter characters to skip if there is a match
  let mut count: usize = 0;
  let mut longest_match_count: usize = 0;

  let mut iter_peek_next: Option<char>;

  let mut longest_match_next_char: Option<char> = None;

  // The character needs to be considered the first character in the named_character_reference
  match c {
    Some(';') => {
      is_last_semicolon = true; 
      current_key.push(';');
    },
    Some(x) => current_key.push(x),
    None => return named_character_reference_state_transition_no_match(c, current_state, return_state, current_token, temporary_buffer)
  }

  // TODO Refactor this function. It got messy

  loop {
    println!("current_key: '{}', longest_match: '{:?}'", current_key, longest_match);
    
    // TODO There has got to be a better way to do this, but I couldn't figure it out right now
    // Rust just kicks my ass sometimes
    iter_peek_next = match iter.peek() {
      Some(x) => Some(x.clone()),
      None => None
    };

    // Check if there is a match to a named reference
    if NAMED_CHARACTER_REFERENCE.contains_key(&current_key) {
      // Use the newest match as the longest match
      longest_match = Some(current_key.clone());

      // Keep track of the character after the longest match
      longest_match_next_char = iter_peek_next.clone();

      // Keep track of the count at the longest match
      longest_match_count = count.clone();
    }

    // Keep only the keys that current_key can still be to see if a match is possible
    // TODO Rework this to use a trie
    let named_reference_keys_len = NAMED_CHARACTER_REFERENCE.keys().filter(|x| x.starts_with(current_key.as_str())).count();

    // If there is 1 possible key and there is a match, you've found key
    // OR there is nothing else it could be if there are no possible keys
    if named_reference_keys_len == 1 && NAMED_CHARACTER_REFERENCE.contains_key(&current_key) ||
      named_reference_keys_len <= 0 {

      println!("current_key: '{}', longest_match: '{:?}'", current_key, longest_match);

      // If we have found a match (longest_match), use the match. otherwise end here
      match longest_match {
        Some(key) => {
          // TODO This boolean should check the character after the longest_match, not iter.peek
          let mut is_next_equals_or_ascii_alphanumeric: bool = false;

          if let Some(next_char) = longest_match_next_char {
            is_next_equals_or_ascii_alphanumeric = 
              next_char == '=' ||
              next_char.is_ascii_alphanumeric();
          };

          if is_consumed_as_attribute(return_state) && 
            !is_last_semicolon &&
            is_next_equals_or_ascii_alphanumeric {
            return named_character_reference_state_transition_otherwise(c, current_state, return_state, current_token, temporary_buffer);
          }

          // Skip the characters from key in iter to consume them
          iter.nth(longest_match_count - 1);

          return named_character_reference_state_transition_match(c, current_state, return_state, current_token, temporary_buffer, is_last_semicolon, key);
        },
        None => return named_character_reference_state_transition_no_match(c, current_state, return_state, current_token, temporary_buffer)
      };
    }

    is_last_semicolon = false;

    // Add the next character to the temp_buffer
    match iter_peek_next {
      Some(';') => {
        is_last_semicolon = true; 
        current_key.push(';');
      },
      Some(x) => current_key.push(x.clone()),
      None => return named_character_reference_state_transition_no_match(c, current_state, return_state, current_token, temporary_buffer)
    }

    count += 1;
  }
}

fn named_character_reference_state_transition_match(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
  is_last_semicolon: bool,
  key: String
) -> (Option<Vec<Token>>, bool) {
  println!("Named Character Reference State Match: '{:?}'", c);

  if !is_last_semicolon {
    missing_semicolon_after_character_reference_parse_error::error(DataState::NamedCharacterReferenceState.to_string(), c);
  }

  *temporary_buffer = NAMED_CHARACTER_REFERENCE.get(&key).unwrap().to_string();

  *current_state = return_state.clone();

  let emitted_tokens = flush_code_points(return_state, current_token, temporary_buffer);

  return (emitted_tokens, false);
}

fn named_character_reference_state_transition_otherwise(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Named Character Reference State Otherwise: '{:?}'", c);

  *current_state = return_state.clone();

  let emitted_tokens = flush_code_points(return_state, current_token, temporary_buffer);

  return (emitted_tokens, true);
}

fn named_character_reference_state_transition_no_match(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Named Character Reference State No Match: '{:?}'", c);

  *current_state = DataState::AmbiguousAmpersandState;

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
  fn test_named_character_reference_state_transition_match_1() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::NamedCharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "&".to_string();
    let mut iter = itertools::multipeek("acute;123".chars());

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz\u{000C1}".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = named_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("\u{000C1}".to_string(), temporary_buffer);
    assert_eq!(Some('1'), iter.next());
  }

  #[test]
  fn test_named_character_reference_state_transition_match_2() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::NamedCharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "&".to_string();
    let mut iter = itertools::multipeek("acute 123".chars());

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz\u{000C1}".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = named_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("\u{000C1}".to_string(), temporary_buffer);
    assert_eq!(Some(' '), iter.next());
  }

  #[test]
  fn test_named_character_reference_state_transition_otherwise_1() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::NamedCharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "&".to_string();
    let mut iter = itertools::multipeek("acute123".chars());

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz&".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = named_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
    assert_eq!(Some('a'), iter.next());
  }

  #[test]
  fn test_named_character_reference_state_transition_no_match_1() {
    const C: Option<char> = Some('x');
    let mut current_state: DataState = DataState::NamedCharacterReferenceState;
    let mut return_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let mut temporary_buffer: String = "&".to_string();
    let mut iter = itertools::multipeek("xx123".chars());

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          name: "abc".to_string(),
          value: "xyz&".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = named_character_reference_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token,
      &mut temporary_buffer,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AmbiguousAmpersandState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!("&".to_string(), temporary_buffer);
    assert_eq!(Some('x'), iter.next());
  }
}
