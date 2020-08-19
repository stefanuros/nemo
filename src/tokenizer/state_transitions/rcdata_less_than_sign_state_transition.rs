use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn rcdata_less_than_sign_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA Less Than Sign State c: '{:?}'", c);

  return match c {
    Some('\u{002F}') => rcdata_less_than_sign_state_transition_solidus(c, current_state, temporary_buffer),
    _ => rcdata_less_than_sign_state_transition_anything_else(c, current_state),
  }
}

fn rcdata_less_than_sign_state_transition_solidus(
  c: Option<char>, 
  current_state: &mut DataState,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA Less Than Sign State Solidus: '{:?}'", c);

  *temporary_buffer = "".to_string();
  *current_state = DataState::RCDATAEndTagOpenState;

  return (None, false);
}

fn rcdata_less_than_sign_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("RCDATA Less Than Sign State Anything Else: '{:?}'", c);

  *current_state = DataState::RCDataState;

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
  fn test_rcdata_less_than_sign_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::RCDATALessThanSignState;
    let mut temporary_buffer: String = "g".to_string();

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rcdata_less_than_sign_state_transition(C, &mut current_state, &mut temporary_buffer);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDATAEndTagOpenState, current_state);
    assert_eq!("".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rcdata_less_than_sign_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::RCDATALessThanSignState;
    let mut temporary_buffer: String = "g".to_string();

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}')
      ]), 
      true
    );
    let result = rcdata_less_than_sign_state_transition(C, &mut current_state, &mut temporary_buffer);

    assert_eq!(expected, result);
    assert_eq!(DataState::RCDataState, current_state);
    assert_eq!("g".to_string(), temporary_buffer);
  }
}
