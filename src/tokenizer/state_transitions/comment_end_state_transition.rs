use crate::errors::tokenizer_errors::eof_in_comment_parse_error;
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn comment_end_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State c: '{:?}'", c);

  return match c {
    Some('\u{003E}') => comment_end_state_transition_greater_than_sign(c, current_state, current_token),
    Some('\u{0021}') => comment_end_state_transition_exclamation_mark(c, current_state),
    Some('\u{002D}') => comment_end_state_transition_hyphen_minus(c, current_token),
    None => comment_end_state_transition_eof(current_token),
    _ => comment_end_state_transition_anything_else(c, current_state, current_token)
  }
}

fn comment_end_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (
    Some(vec![
      current_token.clone().unwrap()
    ]), 
    false
  );
}

fn comment_end_state_transition_exclamation_mark(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::CommentEndBangState;

  return (None, false);
}

fn comment_end_state_transition_hyphen_minus(
  c: Option<char>,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State Hyphen Minus: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push('\u{002D}');
  }

  return (None, false);
}

fn comment_end_state_transition_eof(
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State EOF");

  eof_in_comment_parse_error::error(DataState::CommentEndDashState.to_string());

  return (
    Some(vec![
      current_token.clone().unwrap(),
      Token::EOFToken
    ]), 
    false
  );
}

fn comment_end_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Comment End State Anything Else: '{:?}'", c);

  if let Some(Token::CommentToken(ref mut comment_data)) = current_token {
    comment_data.push_str("\u{002D}\u{002D}");
  }

  *current_state = DataState::CommentState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comment_end_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::CommentEndState;
    let mut current_token: Option<Token> = Some(Token::new_comment("comment"));
    
    let expected_current_token: Option<Token> = Some(Token::new_comment("comment"));
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap()
      ]), 
      false
    );
    let result = comment_end_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(expected_current_token, current_token)
  }

  #[test]
  fn test_comment_end_state_transition_exclamation_mark() {
    const C: Option<char> = Some('!');
    let mut current_state: DataState = DataState::CommentEndState;
    let mut current_token: Option<Token> = Some(Token::new_comment("comment"));
    
    let expected_current_token: Option<Token> = Some(Token::new_comment("comment"));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_end_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndBangState, current_state);
    assert_eq!(expected_current_token, current_token)
  }

  #[test]
  fn test_comment_end_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::CommentEndState;
    let mut current_token: Option<Token> = Some(Token::new_comment("comment"));
    
    let expected_current_token: Option<Token> = Some(Token::new_comment("comment-"));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = comment_end_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndState, current_state);
    assert_eq!(expected_current_token, current_token)
  }

  #[test]
  fn test_comment_end_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::CommentEndState;
    let mut current_token: Option<Token> = Some(Token::new_comment("comment"));
    
    let expected_current_token: Option<Token> = Some(Token::new_comment("comment"));
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        expected_current_token.clone().unwrap(),
        Token::EOFToken
      ]), 
      false
    );
    let result = comment_end_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentEndState, current_state);
    assert_eq!(expected_current_token, current_token)
  }

  #[test]
  fn test_comment_end_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CommentEndState;
    let mut current_token: Option<Token> = Some(Token::new_comment("comment"));
    
    let expected_current_token: Option<Token> = Some(Token::new_comment("comment--"));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = comment_end_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentState, current_state);
    assert_eq!(expected_current_token, current_token)
  }
}
