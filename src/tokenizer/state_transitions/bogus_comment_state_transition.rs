use crate::errors::tokenizer_errors::unexpected_null_character_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn bogus_comment_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus Comment State c: '{:?}'", c);

  return match c {
    Some('\u{003E}') => bogus_comment_state_transition_greater_than_sign(c, current_state, current_token),
    None => bogus_comment_state_transition_eof(c, current_token),
    Some('\u{0000}') => bogus_comment_state_transition_null(c, current_token),
    _ => bogus_comment_state_transition_anything_else(c, current_token)
  }
}

fn bogus_comment_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus Comment State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn bogus_comment_state_transition_eof(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus Comment State EOF: '{:?}'", c);

  return (
    Some(vec![
      current_token.clone().unwrap(),
      Token::EOFToken
    ]), 
    false
  );
}

fn bogus_comment_state_transition_null(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus Comment State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::BogusCommentState.to_string(), c.unwrap());

  if let Some(Token::CommentToken(ref mut comment)) = current_token {
    comment.push('\u{FFFD}');
  }

  return(None, false);
}

fn bogus_comment_state_transition_anything_else(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Bogus Comment State Anything Else: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment)) = current_token {
    comment.push(c.unwrap());
  }

  return(None, false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bogus_comment_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::BogusCommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = bogus_comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_comment_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::BogusCommentState;
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
    let result = bogus_comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_comment_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::BogusCommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment�")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = bogus_comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_bogus_comment_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::BogusCommentState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("commentg")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = bogus_comment_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
