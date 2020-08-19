use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_less_than_sign_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Less Than State c: '{:?}'", c);

  return match c {
    Some('\u{002F}') => script_data_less_than_sign_state_transition_solidus(c, current_state, temporary_buffer),
    Some('\u{0021}') => script_data_less_than_sign_state_transition_exclamation_mark(c, current_state),
    _ => script_data_less_than_sign_state_transition_anything_else(c, current_state),
  }
}

fn script_data_less_than_sign_state_transition_solidus(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Less Than State Solidus: '{:?}'", c);

  *temporary_buffer = "".to_string();
  *current_state = DataState::ScriptDataEndTagOpenState;

  return (None, false);
}

fn script_data_less_than_sign_state_transition_exclamation_mark(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Less Than State Exclamation Mark: '{:?}'", c);

  *current_state = DataState::ScriptDataEscapeStartState;

  return (
    Some(vec![
      Token::CharacterToken('\u{003C}'),
      Token::CharacterToken('\u{0021}'),
    ])
    , 
    false
  );
}

fn script_data_less_than_sign_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Less Than State Anything Else: '{:?}'", c);

  *current_state = DataState::ScriptDataState;

  return(
    Some(vec![
      Token::CharacterToken('\u{003C}'),
    ]), 
    true
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_less_than_sign_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::ScriptDataLessThanSignState;
    let mut temporary_buffer: String = "abc".to_string();

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = script_data_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEndTagOpenState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_less_than_sign_state_transition_exclamation_mark() {
    const C: Option<char> = Some('!');
    let mut current_state: DataState = DataState::ScriptDataLessThanSignState;
    let mut temporary_buffer: String = "".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
        Token::CharacterToken('\u{0021}'),
      ]), 
      false
    );
    let result = script_data_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapeStartState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }

  #[test]
  fn test_script_data_less_than_sign_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataLessThanSignState;
    let mut temporary_buffer: String = "".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
      ]), 
      true
    );
    let result = script_data_less_than_sign_state_transition(
      C, 
      &mut current_state,
      &mut temporary_buffer
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }
}
