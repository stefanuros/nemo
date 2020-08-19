use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_less_than_sign_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign State c: '{:?}'", c);

  return match c {
    Some('\u{0021}') => comment_less_than_sign_state_transition_exclamation_mark(c, current_state, current_token),
    Some('\u{003C}') => comment_less_than_sign_state_transition_less_than_sign(c, current_token),
    _ => comment_less_than_sign_state_transition_anything_else(c, current_state)
  }
}

fn comment_less_than_sign_state_transition_exclamation_mark(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign State Exclamation Mark: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push(c.unwrap());
  }

  *current_state = DataState::CommentLessThanSignBangState;

  return (None, false);
}

fn comment_less_than_sign_state_transition_less_than_sign(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign State Less Than Sign: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push(c.unwrap());
  }

  return (None, false);
}

fn comment_less_than_sign_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment Less Than Sign State Anything Else: '{:?}'", c);

  *current_state = DataState::CommentState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_less_than_sign_state_transition_exclamation_mark() {
    const C: Option<char> = Some('!');
    let mut current_state: DataState = DataState::CommentLessThanSignState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment!")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentLessThanSignBangState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_less_than_sign_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::CommentLessThanSignState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment<")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentLessThanSignState, current_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_comment_less_than_sign_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentLessThanSignState;
    let mut current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected_current_token: Option<Token> = Some(
      Token::new_comment("comment")
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token);
  }
}
