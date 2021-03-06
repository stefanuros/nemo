use crate::errors::tokenizer_errors::{
  unexpected_null_character_parse_error,
  eof_in_tag_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn tag_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => tag_name_state_transition_whitespace(c, current_state),
    Some('\u{002F}') => tag_name_state_transition_solidus(c, current_state),
    Some('\u{003E}') => tag_name_state_transition_greater_than_sign(c, current_state, current_token),
    Some(x) if x.is_ascii_uppercase() => tag_name_state_transition_ascii_upper_alpha(c, current_token),
    Some('\u{0000}') => tag_name_state_transition_null(c, current_token),
    None => tag_name_state_transition_eof(),
    _ => tag_name_state_transition_anything_else(c, current_token),
  }
}

fn tag_name_state_transition_whitespace(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Whitespace: '{:?}'", c);

  *current_state = DataState::BeforeAttributeNameState;

  return (None, false);
}

fn tag_name_state_transition_solidus(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Solidus: '{:?}'", c);

  *current_state = DataState::SelfClosingStartTagState;

  return (None, false);
}

fn tag_name_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::DataState;

  return(
    Some(vec![
      current_token.clone().unwrap()
    ]),
    false
  );
}

fn tag_name_state_transition_ascii_upper_alpha(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Ascii Upper Alpha: '{:?}'", c);

  // Add to the current tag token value
  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_tag_name(c.unwrap().to_ascii_lowercase());
  }
  
  return (None, false);
}

fn tag_name_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::TagNameState.to_string(), c.unwrap());

  // Add to the current current_token value
  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_tag_name('\u{FFFD}');
  }
  
  return (None, false);
}

fn tag_name_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State EOF");

  eof_in_tag_parse_error::error(
    DataState::DataState.to_string()
  );

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn tag_name_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Name State Anything Else: '{:?}'", c);

  // Add to the current current_token value
  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_tag_name(c.unwrap());
  }

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::TagToken;

  #[test]
  fn test_tag_name_state_transition_ascii_end_tag_token() {
    const C: Option<char> = Some('D');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("d"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_ascii_start_tag_token() {
    const C: Option<char> = Some('D');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("d"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_whitespace() {
    const C: Option<char> = Some(' ');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::default())), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::SelfClosingStartTagState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::default())), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::new("abc")));

    let expected: (Option<Vec<Token>>, bool) = 
      (
        Some(vec![
          Token::StartTagToken(TagToken::new("abc"))
        ]), 
        false
      );
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("abc"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_ascii_upper_alpha() {
    const C: Option<char> = Some('D');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("d"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_ascii_lower_alpha() {
    const C: Option<char> = Some('d');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("d"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("�"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_eof() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("�"))), current_token);
  }

  #[test]
  fn test_tag_name_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::TagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_name_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("7"))), current_token);
  }
}
