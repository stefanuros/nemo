use crate::errors::tokenizer_errors::{
  unexpected_question_mark_instead_of_tag_name_parse_error,
  eof_before_tag_name_parse_error,
  invalid_first_character_of_tag_name_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn tag_open_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State c: '{:?}'", c);

  return match c {
    Some('\u{0021}') => tag_open_state_transition_exclamation_mark(c, current_state),
    Some('\u{002F}') => tag_open_state_transition_solidus(c, current_state),
    Some('\u{003F}') => tag_open_state_transition_question_mark(c, current_state, current_token),
    Some(x) if x.is_ascii_alphabetic() => tag_open_state_transition_ascii_alpha(c, current_state, current_token),
    None => tag_open_state_transition_eof(),
    _ => tag_open_state_transition_anything_else(c, current_state),
  }
}

fn tag_open_state_transition_exclamation_mark(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::MarkupDeclarationOpenState;

  return (None, false);
}

fn tag_open_state_transition_solidus(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State Solidus: '{:?}'", c);
  *current_state = DataState::EndTagOpenState;

  return (None, false);
}

fn tag_open_state_transition_question_mark(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State Question Mark: '{:?}'", c);

  unexpected_question_mark_instead_of_tag_name_parse_error::error(
    DataState::DataState.to_string(), 
    c.unwrap()
  );

  *current_state = DataState::BogusCommentState;
  *current_token = Some(
    Token::CommentToken("".to_string())
  );
  
  return (None, true);
}

fn tag_open_state_transition_ascii_alpha(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State Ascii Alpha: '{:?}'", c);

  *current_state = DataState::TagNameState;
  *current_token = Some(
    Token::StartTagToken("".to_string())
  );
  
  return (None, true);
}

fn tag_open_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State EOF");

  eof_before_tag_name_parse_error::error(
    DataState::DataState.to_string()
  );

  return (
    Some(vec![
      Token::CharacterToken('\u{003C}'), 
      Token::EOFToken
    ]), 
    false
  );
}

fn tag_open_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Tag Open State Anything Else: '{:?}'", c);

  invalid_first_character_of_tag_name_parse_error::error(
    DataState::DataState.to_string(),
    c.unwrap()
  );

  *current_state = DataState::DataState;

  return(
    Some(vec![
      Token::CharacterToken('\u{003C}')
    ]), 
    true
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tag_open_state_transition_exclamation_mark() {
    const C: Option<char> = Some('!');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::MarkupDeclarationOpenState, current_state);
    assert_eq!(None, current_token);
  }

  #[test]
  fn tag_open_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::EndTagOpenState, current_state);
    assert_eq!(None, current_token);
  }

  #[test]
  fn tag_open_state_transition_question_mark() {
    const C: Option<char> = Some('?');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(Some(Token::CommentToken("".to_string())), current_token);
  }

  #[test]
  fn tag_open_state_transition_ascii_alpha_uppercase() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken("".to_string())), current_token);
  }

  #[test]
  fn tag_open_state_transition_ascii_alpha_lowercase() {
    const C: Option<char> = Some('a');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagNameState, current_state);
    assert_eq!(Some(Token::StartTagToken("".to_string())), current_token);
  }

  #[test]
  fn test_tag_open_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = 
      (
        Some(vec![
          Token::CharacterToken('\u{003C}'), 
          Token::EOFToken
        ]), 
        false
      );
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::TagOpenState, current_state);
    assert_eq!(None, current_token);
  }

  #[test]
  fn test_tag_open_state_transition_anything_else() {
    const C: Option<char> = Some('5');
    let mut current_state: DataState = DataState::TagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = 
      (
        Some(vec![
          Token::CharacterToken('\u{003C}')
        ]), 
        true
      );
    let result = tag_open_state_transition(C, &mut current_state, &mut current_token);

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(None, current_token);
  }
}
