use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn cdata_section_bracket_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section Bracket State c: '{:?}'", c);

  return match c {
    Some('\u{005D}') => cdata_section_bracket_state_transition_right_square_bracket(c, current_state),
    _ => cdata_section_bracket_state_transition_anything_else(c, current_state)
  }
}

fn cdata_section_bracket_state_transition_right_square_bracket(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section Bracket State Right Square Bracket: '{:?}'", c);

  *current_state = DataState::CDATASectionEndState;

  return (None, false);
}

fn cdata_section_bracket_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section Bracket State Anything Else: '{:?}'", c);

  *current_state = DataState::CDATASectionState;

  return(
    Some(vec![
      Token::CharacterToken('\u{005D}')
    ]), 
    true
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_cdata_section_bracket_state_transition_right_square_bracket() {
    const C: Option<char> = Some(']');
    let mut current_state: DataState = DataState::CDATASectionBracketState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = cdata_section_bracket_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionEndState, current_state);
  }

  #[test]
  fn test_cdata_section_bracket_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CDATASectionBracketState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{005D}')
      ]),
      true
    );
    let result = cdata_section_bracket_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionState, current_state);
  }
}
