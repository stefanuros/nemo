use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_less_than_sign_bang_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang State c: '{:?}'", c);

  return match c {
    Some('\u{002D}') => comment_less_than_sign_bang_state_transition_hyphen_minus(c, current_state),
    _ => comment_less_than_sign_bang_state_transition_anything_else(c, current_state)
  }
}

fn comment_less_than_sign_bang_state_transition_hyphen_minus(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang State Hyphen Minus: '{:?}'", c);

  *current_state = DataState::CommentLessThanSignBangDashState;

  return (None, false);
}

fn comment_less_than_sign_bang_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign Bang State Anything Else: '{:?}'", c);

  *current_state = DataState::CommentState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_less_than_sign_bang_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::CommentLessThanSignBangState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_less_than_sign_bang_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentLessThanSignBangDashState, current_state);
  }

  #[test]
  fn test_comment_less_than_sign_bang_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentLessThanSignBangState;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_less_than_sign_bang_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
  }
}
