use crate::errors::tokenizer_errors::{
  unexpected_null_character_parse_error,
  eof_in_script_html_comment_like_text_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_double_escaped_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State c: '{:?}'", c);

  return match c {
    Some('\u{002D}') => script_data_double_escaped_state_transition_hyphen_minus(c, current_state),
    Some('\u{003C}') => script_data_double_escaped_state_transition_less_than_sign(c, current_state),
    Some('\u{0000}') => script_data_double_escaped_state_transition_null(c),
    None => script_data_double_escaped_state_transition_eof(),
    _ => script_data_double_escaped_state_transition_anything_else(c),
  }
}

fn script_data_double_escaped_state_transition_hyphen_minus(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State Hyphen Minus: '{:?}'", c);

  *current_state = DataState::ScriptDataDoubleEscapedDashState;

  return (
    Some(vec![
      Token::CharacterToken('\u{002D}')
    ]), 
    false
  );
}

fn script_data_double_escaped_state_transition_less_than_sign(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State Less Than Sign: '{:?}'", c);

  *current_state = DataState::ScriptDataDoubleEscapedLessThanSignState;

  return (None, false);
}

fn script_data_double_escaped_state_transition_null(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State Null: '{:?}'", c);

  unexpected_null_character_parse_error::error(DataState::ScriptDataEscapedState.to_string(), c.unwrap());

  return (
    Some(vec![
      Token::CharacterToken('\u{FFFD}')
    ]), 
    false
  );
}

fn script_data_double_escaped_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State EOF");

  eof_in_script_html_comment_like_text_parse_error::error(DataState::ScriptDataEscapedState.to_string());

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn script_data_double_escaped_state_transition_anything_else(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escaped State Anything Else: '{:?}'", c);

  return(
    Some(vec![
      Token::CharacterToken(c.unwrap())
    ]), 
    false
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_double_escaped_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapedState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{002D}'),
      ]), 
      false
    );
    let result = script_data_double_escaped_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedDashState, current_state);
  }

  #[test]
  fn test_script_data_double_escaped_state_transition_less_than_sign() {
    const C: Option<char> = Some('<');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapedState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = script_data_double_escaped_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedLessThanSignState, current_state);
  }

  #[test]
  fn test_script_data_double_escaped_state_transition_null() {
    const C: Option<char> = Some('\0');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapedState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{FFFD}'),
      ]), 
      false
    );
    let result = script_data_double_escaped_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedState, current_state);
  }

  #[test]
  fn test_script_data_double_escaped_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapedState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::EOFToken,
      ]), 
      false
    );
    let result = script_data_double_escaped_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedState, current_state);
  }

  #[test]
  fn test_script_data_double_escaped_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapedState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('7')
      ]), 
      false
    );
    let result = script_data_double_escaped_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedState, current_state);
  }
}
