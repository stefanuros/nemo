use crate::errors::tokenizer_errors::{
  unexpected_null_character_parse_error,
  eof_in_comment_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State c: '{:?}'", c);

  return match c {
    Some('\u{003C}') => comment_state_transition_less_than_sign(c, current_state, current_token),
    Some('\u{002D}') => comment_state_transition_hyphen_minus(c, current_state),
    Some('\u{0000}') => comment_state_transition_null(c, current_token),
    None => comment_state_transition_eof(current_token),
    _ => comment_state_transition_anything_else(c, current_token)
  }
}

fn comment_state_transition_less_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State Less Than Sign: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push(c.unwrap());
  }

  *current_state = DataState::CommentLessThanSignState;

  return (None, false);
}

fn comment_state_transition_hyphen_minus(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State Hyphen Minus: '{:?}'", c);

  *current_state = DataState::CommentEndDashState;

  return (None, false);
}

fn comment_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::CommentState.to_string(), c.unwrap());

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push('\u{FFFD}');
  }

  return (None, false);
}

fn comment_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State EOF");

  eof_in_comment_parse_error::error(DataState::CommentState.to_string());

  return (
    Some(vec![
      current_token.clone().unwrap(),
      Token::EOFToken
    ]), 
    false
  );
}

fn comment_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment State Anything Else: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push(c.unwrap());
  }

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::CommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment<")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentLessThanSignState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::CommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndDashState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::CommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment�")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::CommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap(),
        Token::EOFToken
      ]), 
      false
    );
    let result = comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("commentg")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
