use crate::errors::tokenizer_errors::{
  missing_end_tag_name_parse_error,
  eof_before_tag_name_parse_error,
  invalid_first_character_of_tag_name_parse_error
};
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token,
  token_types::TagToken
};

pub fn end_tag_open_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("End Tag Open State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_alphabetic() => end_tag_open_state_transition_ascii_alpha(c, current_state, current_token),
    Some('\u{003E}') => end_tag_open_state_transition_greater_than_sign(c, current_state),
    None => end_tag_open_state_transition_eof(),
    _ => end_tag_open_state_transition_anything_else(c, current_state, current_token),
  }
}

fn end_tag_open_state_transition_ascii_alpha(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("End Tag Open State Ascii Alpha: '{:?}'", c);

  *current_state = DataState::TagNameState;
  *current_token = Some(Token::EndTagToken(TagToken::default()));
  
  return(None, true);
}

fn end_tag_open_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("End Tag Open State Greater Than Sign: '{:?}'", c);

  missing_end_tag_name_parse_error::error(DataState::EndTagOpenState.to_string(), c.unwrap());

  *current_state = DataState::DataState;

  return(None, false);
}

fn end_tag_open_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("End Tag Open State EOF");

  eof_before_tag_name_parse_error::error(DataState::EndTagOpenState.to_string());

  return(
    Some(vec![
      Token::CharacterToken('\u{003C}'), 
      Token::CharacterToken('\u{002F}'), 
      Token::EOFToken
    ]), 
    false
  );
}

fn end_tag_open_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("End Tag Open State Anything Else: '{:?}'", c);

  invalid_first_character_of_tag_name_parse_error::error(
    DataState::EndTagOpenState.to_string(),
    c.unwrap()
  );

  *current_state = DataState::BogusCommentState;
  *current_token = Some(Token::empty_comment());

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn end_tag_open_state_transition_ascii_alpha() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::EndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = end_tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::default())), current_token);
  }

  #[test]
  fn end_tag_open_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::EndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = end_tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(None, current_token);
  }

  #[test]
  fn test_end_tag_open_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::EndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = 
      (
        Some(vec![
          Token::CharacterToken('\u{003C}'), 
          Token::CharacterToken('\u{002F}'), 
          Token::EOFToken
        ]), 
        false
      );
    let result = end_tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::EndTagOpenState, current_state);
    assert_eq!(None, current_token);
  }

  #[test]
  fn test_end_tag_open_state_transition_anything_else() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::EndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = end_tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(Some(Token::empty_comment()), current_token);
  }
}
