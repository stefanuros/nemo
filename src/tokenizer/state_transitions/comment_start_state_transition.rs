use crate::errors::tokenizer_errors::abrupt_closing_of_empty_comment_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_start_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Start State c: '{:?}'", c);

  return match c {
    Some('\u{002D}') => comment_start_state_transition_hyphen_minus(c, current_state),
    Some('\u{003E}') => comment_start_state_transition_greater_than_sign(c, current_state, current_token),
    _ => comment_start_state_transition_anything_else(c, current_state)
  }
}

fn comment_start_state_transition_hyphen_minus(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Start State Hyphen Minus: '{:?}'", c);

  *current_state = DataState::CommentStartDashState;

  return (None, false);
}

fn comment_start_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Start State Greater Than Sign: '{:?}'", c);

  abrupt_closing_of_empty_comment_parse_error::error(DataState::CommentStartState.to_string(), c.unwrap());

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn comment_start_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Start State Anything Else: '{:?}'", c);

  *current_state = DataState::CommentState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_start_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::CommentStartState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_start_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentStartDashState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_start_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::CommentStartState;
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
    let result = comment_start_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_start_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentStartState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_start_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
