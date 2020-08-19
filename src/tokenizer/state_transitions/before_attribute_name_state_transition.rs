use crate::errors::tokenizer_errors::unexpected_equals_sign_before_attribute_name_parse_error;

use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn before_attribute_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => before_attribute_name_state_transition_whitespace(c),
    Some('\u{002F}') |
    Some('\u{003E}') |
    None => before_attribute_name_state_transition_greater_than_sign(c, current_state),
    Some('\u{003D}') => before_attribute_name_state_transition_equals_sign(c, current_state, current_token),
    _ => before_attribute_name_state_transition_anything_else(c, current_state, current_token),
  }
}

fn before_attribute_name_state_transition_whitespace(c: Option<char>) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Name State Exclamation Mark: '{:?}'", c);

  return (None, false);
}

fn before_attribute_name_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Name State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::AfterAttributeNameState;

  return (None, true);
}

fn before_attribute_name_state_transition_equals_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Name State Equals Sign: '{:?}'", c);

  unexpected_equals_sign_before_attribute_name_parse_error::error(
    DataState::BeforeAttributeNameState.to_string(), 
    c.unwrap()
  );

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.add_new_attribute(c.unwrap());
  };

  *current_state = DataState::AttributeNameState;
  
  return (None, false);
}

fn before_attribute_name_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Before Attribute Name State Anything Else: '{:?}'", c);

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
  fn test_before_attribute_name_state_transition_whitespace() {
    const C: Option<char> = Some(' ');
    let mut current_state: DataState = DataState::BeforeAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BeforeAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = before_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_name_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BeforeAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = before_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AfterAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_name_state_transition_equals_sign() {
    const C: Option<char> = Some('=');
    let mut current_state: DataState = DataState::BeforeAttributeNameState;
    let mut current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> = Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute::new('=')
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = before_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_before_attribute_name_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::BeforeAttributeNameState;
    let mut current_token: Option<Token> =  Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected_current_token: Option<Token> =  Some(
      Token::StartTagToken(
        TagToken {
          attributes: vec![
            Attribute::default()
          ],
          tag_name: "div".to_string(),
          ..TagToken::default()
        }
      )
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = before_attribute_name_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
