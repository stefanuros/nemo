use crate::errors::tokenizer_errors::{
  unexpected_null_character_parse_error,
  unexpected_character_before_attribute_name_parse_error
};

use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn attribute_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') |
    Some('\u{002F}') |
    Some('\u{003E}') |
    None => attribute_name_state_transition_end_of_attribute_name(c, current_state, current_token),
    Some('\u{003D}') => attribute_name_state_transition_equals_sign(c, current_state, current_token),
    Some(x) if x.is_ascii_uppercase() => attribute_name_state_transition_ascii_upper(c, current_token),
    Some('\u{0000}') => attribute_name_state_transition_null(c, current_token),
    Some('\u{0022}') |
    Some('\u{0027}') |
    Some('\u{003C}') => attribute_name_state_transition_unexpected_character(c, current_token),
    _ => attribute_name_state_transition_anything_else(c, current_token),
  }
}

fn attribute_name_state_transition_end_of_attribute_name(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State End Of Attribute Name: '{:?}'", c);

  *current_state = DataState::AfterAttributeNameState;

  set_attribute_duplicate_value(current_token);

  return (None, true);
}

fn attribute_name_state_transition_equals_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State Equals Sign: '{:?}'", c);

  *current_state = DataState::BeforeAttributeValueState;

  set_attribute_duplicate_value(current_token);
  
  return (None, false);
}

fn attribute_name_state_transition_ascii_upper(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State Ascii Upper: '{:?}'", c);

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_current_attribute_name(c.unwrap().to_ascii_lowercase());
  };
  
  return (None, false);
}

fn attribute_name_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::AttributeNameState.to_string(), c.unwrap());

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_current_attribute_name('\u{FFFD}');
  };
  
  return (None, false);
}

fn attribute_name_state_transition_unexpected_character(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State Unexpected Character: '{:?}'", c);

  unexpected_character_before_attribute_name_parse_error::error(DataState::AttributeNameState.to_string(), c.unwrap());

  return attribute_name_state_transition_anything_else(c, current_token);
}

fn attribute_name_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Name State Anything Else: '{:?}'", c);

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_current_attribute_name(c.unwrap());
  };

  return(None, false);
}

/// This function triggers when user agent leaves AttributeNameState 
fn set_attribute_duplicate_value(current_token: &mut Option<Token>) {
  if let Some(Token::StartTagToken(tag_token)) | Some(Token::EndTagToken(tag_token)) = current_token {
    tag_token.update_current_attribute_duplicate_flag();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::{
    TagToken,
    Attribute
  };

  #[test]
  fn test_attribute_name_state_transition_end_of_attribute_name() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abcd".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abcd".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_end_of_attribute_name_duplicate_attribute_name() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abc".to_string(),
              duplicate: true,
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_end_of_attribute_name_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abcd".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abcd".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_end_of_attribute_name_eof_duplicate_attribute_name() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            },
            Attribute {
              name: "abc".to_string(),
              duplicate: true,
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_equals_sign() {
    const C: Option<char> = Some('=');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeValueState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_ascii_upper() {
    const C: Option<char> = Some('D');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abcd".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc�".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_unexpected_character() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc<".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_name_state_transition_anything_else() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::AttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc5".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
