use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_double_escape_start_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escape Start State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') |
    Some('\u{002F}') |
    Some('\u{003E}') => script_data_double_escape_start_state_transition_greater_than_sign(c, current_state, temporary_buffer),
    Some(x) if x.is_ascii_uppercase() => script_data_double_escape_start_state_transition_ascii_upper_alpha(c, temporary_buffer),
    Some(x) if x.is_ascii_lowercase() => script_data_double_escape_start_state_transition_ascii_lower_alpha(c, temporary_buffer),
    _ => script_data_double_escape_start_state_transition_anything_else(c, current_state),
  }
}

fn script_data_double_escape_start_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escape Start State Exclamation Mark: '{:?}'", c);

  if temporary_buffer == "script" {
    *current_state = DataState::ScriptDataDoubleEscapedState;
  }
  else {
    *current_state = DataState::ScriptDataEscapedState;
  }

  return (
    Some(vec![
      Token::CharacterToken(c.unwrap())
    ]),
    false
  );

}

fn script_data_double_escape_start_state_transition_ascii_upper_alpha(
  c: Option<char>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escape Start State Ascii Upper Alpha: '{:?}'", c);

  temporary_buffer.push(c.unwrap().to_ascii_lowercase());
  
  return (
    Some(vec![
      Token::CharacterToken(c.unwrap())
    ]),
    false
  );
}

fn script_data_double_escape_start_state_transition_ascii_lower_alpha(
  c: Option<char>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escape Start State Ascii Lower Alpha: '{:?}'", c);

  temporary_buffer.push(c.unwrap());
  
  return (
    Some(vec![
      Token::CharacterToken(c.unwrap())
    ]),
    false
  );
}

fn script_data_double_escape_start_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Double Escape Start State Anything Else: '{:?}'", c);

  *current_state = DataState::ScriptDataEscapedState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_double_escape_start_state_transition_greater_than_sign_script() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapeStartState;
    let mut temporary_buffer = "script".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('>')
      ]), 
      false
    );
    let result = script_data_double_escape_start_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapedState, current_state);
    assert_eq!("script".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_double_escape_start_state_transition_greater_than_sign_non_script() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapeStartState;
    let mut temporary_buffer = "notscript".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('>')
      ]), 
      false
    );
    let result = script_data_double_escape_start_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedState, current_state);
    assert_eq!("notscript".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_double_escape_start_state_transition_ascii_upper_alpha() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapeStartState;
    let mut temporary_buffer = "notscript".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('A')
      ]), 
      false
    );
    let result = script_data_double_escape_start_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapeStartState, current_state);
    assert_eq!("notscripta".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_double_escape_start_state_transition_ascii_lower_alpha() {
    const C: Option<char> = Some('a');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapeStartState;
    let mut temporary_buffer = "notscript".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('a')
      ]), 
      false
    );
    let result = script_data_double_escape_start_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataDoubleEscapeStartState, current_state);
    assert_eq!("notscripta".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_double_escape_start_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataDoubleEscapeStartState;
    let mut temporary_buffer = "notscript".to_string();

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = script_data_double_escape_start_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedState, current_state);
    assert_eq!("notscript".to_string(), temporary_buffer);
  }
}
