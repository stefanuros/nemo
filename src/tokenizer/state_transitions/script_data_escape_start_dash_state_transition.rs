use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_escape_start_dash_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escape Start Dash State c: '{:?}'", c);

  return match c {
    Some('\u{002D}') => script_data_escape_start_dash_state_transition_hyphen_minus(c, current_state),
    _ => script_data_escape_start_dash_state_transition_anything_else(c, current_state),
  }
}

fn script_data_escape_start_dash_state_transition_hyphen_minus(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escape Start Dash State Hyphen Minus: '{:?}'", c);

  *current_state = DataState::ScriptDataEscapedDashDashState;

  return (
    Some(vec![
      Token::CharacterToken('\u{002D}')
    ]), 
    false
  );
}

fn script_data_escape_start_dash_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escape Start Dash State Anything Else: '{:?}'", c);

  *current_state = DataState::ScriptDataState;

  return(None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_escape_start_dash_state_transition_hyphen_minus() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::ScriptDataEscapeStartDashState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{002D}'),
      ]), 
      false
    );
    let result = script_data_escape_start_dash_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedDashDashState, current_state);
  }

  #[test]
  fn test_script_data_escape_start_dash_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataEscapeStartDashState;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = script_data_escape_start_dash_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataState, current_state);
  }
}
