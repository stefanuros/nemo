use crate::errors::tokenizer_errors::eof_in_tag_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn after_attribute_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => after_attribute_name_state_transition_ignore_character(c),
    Some('\u{002F}') => after_attribute_name_state_transition_solidus(c, current_state),
    Some('\u{003D}') => after_attribute_name_state_transition_equals_sign(c, current_state),
    Some('\u{003E}')  => after_attribute_name_state_transition_greater_than_sign(c, current_state, current_token),
    None => after_attribute_name_state_transition_eof(c),
    _ => after_attribute_name_state_transition_anything_else(c, current_state, current_token),
  }
}

fn after_attribute_name_state_transition_ignore_character(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State Ignore Character: '{:?}'", c);

  return (None, false);
}

fn after_attribute_name_state_transition_solidus(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State Solidus: '{:?}'", c);

  *current_state = DataState::SelfClosingStartTagState;

  return (None, false);
}

fn after_attribute_name_state_transition_equals_sign(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State Equals Sign: '{:?}'", c);

  *current_state = DataState::BeforeAttributeValueState;
  
  return (None, false);
}

fn after_attribute_name_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(
      vec![
        current_token.clone().unwrap()
      ]
    ), 
    false
  );
}

fn after_attribute_name_state_transition_eof(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State EOF: '{:?}'", c);

  eof_in_tag_parse_error::error(DataState::AfterAttributeNameState.to_string());
  
  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn after_attribute_name_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Name State Anything Else: '{:?}'", c);

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.add_default_attribute();
  };

  *current_state = DataState::AttributeNameState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::{
    TagToken,
    Attribute
  };

  #[test]
  fn test_after_attribute_name_state_transition_ignore_character() {
    const C: Option<char> = Some('\t');
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_name_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::SelfClosingStartTagState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_name_state_transition_equals_sign() {
    const C: Option<char> = Some('=');
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeValueState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_name_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::EOFToken
      ]), 
      false
    );
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_name_state_transition_anything_else() {
    const C: Option<char> = Some('6');
    let mut current_state: DataState = DataState::AfterAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute {
              name: "abc".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string()
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
            Attribute::default()
          ],
          tag_name: "div".to_string()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = after_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
