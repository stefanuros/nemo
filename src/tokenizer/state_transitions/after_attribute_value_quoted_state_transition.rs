use crate::errors::tokenizer_errors::{
  eof_in_tag_parse_error,
  missed_whitespace_between_attributes_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn after_attribute_value_quoted_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => after_attribute_value_quoted_state_transition_whitespace(c, current_state),
    Some('\u{002F}') => after_attribute_value_quoted_state_transition_solidus(c, current_state),
    Some('\u{003E}') => after_attribute_value_quoted_state_transition_greater_than_sign(c, current_state, current_token),
    None => after_attribute_value_quoted_state_transition_eof(c),
    _ => after_attribute_value_quoted_state_transition_anything_else(c, current_state)
  }
}

fn after_attribute_value_quoted_state_transition_whitespace(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State Whitespace: '{:?}'", c);

  *current_state = DataState::BeforeAttributeNameState;

  return (None, false);
}

fn after_attribute_value_quoted_state_transition_solidus(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State Solidus: '{:?}'", c);

  *current_state = DataState::SelfClosingStartTagState;

  return (None, false);
}

fn after_attribute_value_quoted_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn after_attribute_value_quoted_state_transition_eof(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State EOF: '{:?}'", c);

  eof_in_tag_parse_error::error(DataState::AfterAttributeValueQuotedState.to_string());

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn after_attribute_value_quoted_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("After Attribute Value Quoted State Anything Else: '{:?}'", c);

  missed_whitespace_between_attributes_parse_error::error(DataState::AfterAttributeValueQuotedState.to_string(), c.unwrap());

  *current_state = DataState::BeforeAttributeNameState;

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
  fn test_after_attribute_value_quoted_state_transition_whitespace() {
    const C: Option<char> = Some('\u{000C}');
    let mut current_state: DataState = DataState::AfterAttributeValueQuotedState;
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
    let result = after_attribute_value_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_value_quoted_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::AfterAttributeValueQuotedState;
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
    let result = after_attribute_value_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::SelfClosingStartTagState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_value_quoted_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AfterAttributeValueQuotedState;
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
    let result = after_attribute_value_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_value_quoted_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AfterAttributeValueQuotedState;
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

    let expected: (Option<Vec<Token>>, bool) = (Some(vec![Token::EOFToken]), false);
    let result = after_attribute_value_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeValueQuotedState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_after_attribute_value_quoted_state_transition_anything_else() {
    const C: Option<char> = Some('6');
    let mut current_state: DataState = DataState::AfterAttributeValueQuotedState;
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
    let result = after_attribute_value_quoted_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
