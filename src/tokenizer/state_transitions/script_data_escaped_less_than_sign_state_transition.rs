use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_escaped_less_than_sign_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped Less Than Sign State c: '{:?}'", c);

  return match c {
    Some('\u{002F}') => script_data_escaped_less_than_sign_state_transition_solidus(c, current_state, temporary_buffer),
    Some(x) if x.is_ascii_alphabetic() => script_data_escaped_less_than_sign_state_transition_ascii_alpha(c, current_state, temporary_buffer),
    _ => script_data_escaped_less_than_sign_state_transition_anything_else(c, current_state),
  }
}

fn script_data_escaped_less_than_sign_state_transition_solidus(
  c: Option<char>,
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped Less Than Sign State Solidus: '{:?}'", c);

  *temporary_buffer = "".to_string();
  *current_state = DataState::ScriptDataEscapedEndTagOpenState;

  return (None, false);
}

fn script_data_escaped_less_than_sign_state_transition_ascii_alpha(
  c: Option<char>,
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped Less Than Sign State Ascii Alpha: '{:?}'", c);

  *temporary_buffer = "".to_string();
  *current_state = DataState::ScriptDataDoubleEscapeStartState;

  return (
    Some(vec![
      Token::CharacterToken('\u{003C}')
    ]), 
    true
  );
}

fn script_data_escaped_less_than_sign_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped Less Than Sign State Anything Else: '{:?}'", c);

  *current_state = DataState::ScriptDataEscapedState;

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
  fn test_script_data_escaped_less_than_sign_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::ScriptDataEscapedLessThanSignState;
    let mut temporary_buffer: String = "abc".to_string();

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = script_data_escaped_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedEndTagOpenState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_escaped_less_than_sign_state_transition_ascii_alpha() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::ScriptDataEscapedLessThanSignState;
    let mut temporary_buffer: String = "abc".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}')
      ]), 
      true
    );
    let result = script_data_escaped_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapeStartState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_escaped_less_than_sign_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataEscapedLessThanSignState;
    let mut temporary_buffer: String = "abc".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}')
      ]), 
      true
    );
    let result = script_data_escaped_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedState, current_state);
    assert_eq!("abc".to_string(), temporary_buffer);
  }
}
