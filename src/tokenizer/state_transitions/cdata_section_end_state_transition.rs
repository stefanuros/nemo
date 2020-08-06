use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn cdata_section_end_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section End State c: '{:?}'", c);

  return match c {
    Some('\u{005D}') => cdata_section_end_state_transition_right_square_bracket(c),
    Some('\u{003E}') => cdata_section_end_state_transition_greater_than_sign(c, current_state),
    _ => cdata_section_end_state_transition_anything_else(c, current_state)
  }
}

fn cdata_section_end_state_transition_right_square_bracket(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section End State Right Square Bracket: '{:?}'", c);

  return (
    Some(vec![
      Token::CharacterToken('\u{005D}')
    ]), 
    false
  );
}

fn cdata_section_end_state_transition_greater_than_sign(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section End State Greater Than Sign: '{:?}'", c);

  *current_state = DataState::DataState;

  return (None, false);
}

fn cdata_section_end_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section End State Anything Else: '{:?}'", c);

  *current_state = DataState::CDATASectionState;

  return(
    Some(vec![
      Token::CharacterToken('\u{005D}'),
      Token::CharacterToken('\u{005D}')
    ]), 
    true
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_cdata_section_end_state_transition_right_square_bracket() {
    const C: Option<char> = Some(']');
    let mut current_state: DataState = DataState::CDATASectionEndState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken(']')
      ]), 
      false
    );
    let result = cdata_section_end_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionEndState, current_state);
  }

  #[test]
  fn test_cdata_section_end_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::CDATASectionEndState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = cdata_section_end_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
  }

  #[test]
  fn test_cdata_section_end_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CDATASectionEndState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{005D}'),
        Token::CharacterToken('\u{005D}')
      ]),
      true
    );
    let result = cdata_section_end_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionState, current_state);
  }
}
