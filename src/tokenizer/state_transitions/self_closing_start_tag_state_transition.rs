use crate::errors::tokenizer_errors::{
  eof_in_tag_parse_error,
  unexpected_solidus_in_tag_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn self_closing_start_tag_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Self Closing Start Tag State c: '{:?}'", c);

  return match c {
    Some('\u{003E}') => self_closing_start_tag_state_transition_greater_than_sign(c, current_state, current_token),
    None => self_closing_start_tag_state_transition_eof(c),
    _ => self_closing_start_tag_state_transition_anything_else(c, current_state)
  }
}

fn self_closing_start_tag_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Self Closing Start Tag State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.set_self_closing(true);
  }

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn self_closing_start_tag_state_transition_eof(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Self Closing Start Tag State EOF: '{:?}'", c);

  eof_in_tag_parse_error::error(DataState::SelfClosingStartTagState.to_string());

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn self_closing_start_tag_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Self Closing Start Tag State Anything Else: '{:?}'", c);

  unexpected_solidus_in_tag_parse_error::error(DataState::SelfClosingStartTagState.to_string(), c.unwrap());

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
  fn test_self_closing_start_tag_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::SelfClosingStartTagState;
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
          self_closing: true,
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
    let result = self_closing_start_tag_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_self_closing_start_tag_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::SelfClosingStartTagState;
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
    let result = self_closing_start_tag_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::SelfClosingStartTagState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_self_closing_start_tag_state_transition_anything_else() {
    const C: Option<char> = Some('6');
    let mut current_state: DataState = DataState::SelfClosingStartTagState;
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
    let result = self_closing_start_tag_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
