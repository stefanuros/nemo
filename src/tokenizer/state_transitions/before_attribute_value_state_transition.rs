use crate::errors::tokenizer_errors::missing_attribute_value_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn before_attribute_value_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => before_attribute_value_state_transition_ignore_character(c),
    Some('\u{0022}') => before_attribute_value_state_transition_quotation_mark(c, current_state),
    Some('\u{0027}') => before_attribute_value_state_transition_apostrophe(c, current_state),
    Some('\u{003E}')  => before_attribute_value_state_transition_greater_than_sign(c, current_state, current_token),
    _ => before_attribute_value_state_transition_anything_else(c, current_state),
  }
}

fn before_attribute_value_state_transition_ignore_character(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State Ignore Character: '{:?}'", c);

  return (None, false);
}

fn before_attribute_value_state_transition_quotation_mark(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State Quotation Mark: '{:?}'", c);

  *current_state = DataState::AttributeValueDoubleQuotedState;

  return (None, false);
}

fn before_attribute_value_state_transition_apostrophe(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State Apostrophe: '{:?}'", c);

  *current_state = DataState::AttributeValueSingleQuotedState;
  
  return (None, false);
}

fn before_attribute_value_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State Exclamation Mark: '{:?}'", c);

  missing_attribute_value_parse_error::error(DataState::BeforeAttributeValueState.to_string(), c.unwrap());

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

fn before_attribute_value_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Value State Anything Else: '{:?}'", c);

  *current_state = DataState::AttributeValueUnquotedState;

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
  fn test_before_attribute_value_state_transition_ignore_character() {
    const C: Option<char> = Some('\t');
    let mut current_state: DataState = DataState::BeforeAttributeValueState;
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
    let result = before_attribute_value_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeValueState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_value_state_transition_quotation_mark() {
    const C: Option<char> = Some('"');
    let mut current_state: DataState = DataState::BeforeAttributeValueState;
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
    let result = before_attribute_value_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_value_state_transition_equals_sign() {
    const C: Option<char> = Some('\'');
    let mut current_state: DataState = DataState::BeforeAttributeValueState;
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
    let result = before_attribute_value_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueSingleQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_value_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BeforeAttributeValueState;
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

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = before_attribute_value_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_value_state_transition_anything_else() {
    const C: Option<char> = Some('6');
    let mut current_state: DataState = DataState::BeforeAttributeValueState;
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

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = before_attribute_value_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueUnquotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
