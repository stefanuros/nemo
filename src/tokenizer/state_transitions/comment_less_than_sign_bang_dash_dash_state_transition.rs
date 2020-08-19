use crate::errors::tokenizer_errors::nested_comment_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_less_than_sign_bang_dash_dash_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang Dash Dash State c: '{:?}'", c);

  return match c {
    None |
    Some('\u{003E}') => comment_less_than_sign_bang_dash_dash_state_transition_greater_than_sign(c, current_state),
    _ => comment_less_than_sign_bang_dash_dash_state_transition_anything_else(c, current_state)
  }
}

fn comment_less_than_sign_bang_dash_dash_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang Dash Dash State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::CommentEndState;

  return (None, true);
}

fn comment_less_than_sign_bang_dash_dash_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang Dash Dash State Anything Else: '{:?}'", c);

  nested_comment_parse_error::error(DataState::CommentLessThanSignBangDashDashState.to_string(), c.unwrap());

  *current_state = DataState::CommentEndState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_less_than_sign_bang_dash_dash_state_transition_greater_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::CommentLessThanSignBangDashDashState;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_less_than_sign_bang_dash_dash_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndState, current_state);
  }

  #[test]
  fn test_comment_less_than_sign_bang_dash_dash_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentLessThanSignBangDashDashState;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_less_than_sign_bang_dash_dash_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndState, current_state);
  }
}
