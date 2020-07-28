use crate::errors::tokenizer_errors::{
  unexpected_null_character_parse_error,
  eof_in_tag_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn attribute_value_double_quoted_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State c: '{:?}'", c);

  return match c {
    Some('\u{0022}') => attribute_value_double_quoted_state_transition_quotation_mark(c, current_state),
    Some('\u{0026}') => attribute_value_double_quoted_state_transition_ampersand(c, current_state, return_state),
    Some('\u{0000}') => attribute_value_double_quoted_state_transition_null(c, current_token),
    None => attribute_value_double_quoted_state_transition_eof(c),
    _ => attribute_value_double_quoted_state_transition_anything_else(c, current_token)
  }
}

fn attribute_value_double_quoted_state_transition_quotation_mark(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State Quotation Mark: '{:?}'", c);

  *current_state = DataState::AfterAttributeValueQuotedState;

  return (None, false);
}

fn attribute_value_double_quoted_state_transition_ampersand(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State Ampersand: '{:?}'", c);

  *return_state = DataState::AttributeValueDoubleQuotedState;
  *current_state = DataState::CharacterReferenceState;

  return (None, false);
}

fn attribute_value_double_quoted_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::AttributeValueDoubleQuotedState.to_string(), c.unwrap());

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_current_attribute_value('\u{FFFD}');
  }

  return (None, false);
}

fn attribute_value_double_quoted_state_transition_eof(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State Null: '{:?}'", c);

  eof_in_tag_parse_error::error(DataState::AttributeValueDoubleQuotedState.to_string());

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn attribute_value_double_quoted_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Attribute Value Double Quoted State Anything Else: '{:?}'", c);

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_current_attribute_value(c.unwrap());
  }

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::{
    TagToken,
    Attribute
  };

  #[test]
  fn test_attribute_value_double_quoted_state_transition_quotation_mark() {
    const C: Option<char> = Some('\"');
    let mut current_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut return_state: DataState = DataState::DataState;
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
    let result = attribute_value_double_quoted_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeValueQuotedState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_value_double_quoted_state_transition_ampersand() {
    const C: Option<char> = Some('&');
    let mut current_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut return_state: DataState = DataState::DataState;
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
    let result = attribute_value_double_quoted_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CharacterReferenceState, current_state);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_value_double_quoted_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut return_state: DataState = DataState::DataState;
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
              value: "�".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_value_double_quoted_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_value_double_quoted_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut return_state: DataState = DataState::DataState;
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
    let result = attribute_value_double_quoted_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_attribute_value_double_quoted_state_transition_anything_else() {
    const C: Option<char> = Some('6');
    let mut current_state: DataState = DataState::AttributeValueDoubleQuotedState;
    let mut return_state: DataState = DataState::DataState;
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
              value: "6".to_string(),
              ..Attribute::default()
            }
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = attribute_value_double_quoted_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueDoubleQuotedState, current_state);
    assert_eq!(DataState::DataState, return_state);
    assert_eq!(expected_current_token, current_token);
  }
}
