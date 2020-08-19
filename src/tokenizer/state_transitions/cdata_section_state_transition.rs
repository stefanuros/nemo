use crate::errors::tokenizer_errors::eof_in_cdata_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn cdata_section_state_transition(
  c: Option<char>, 
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section State c: '{:?}'", c);

  return match c {
    Some('\u{005D}') => cdata_section_state_transition_right_square_bracket(c, current_state),
    None => cdata_section_state_transition_eof(),
    _ => cdata_section_state_transition_anything_else(c)
  }
}

fn cdata_section_state_transition_right_square_bracket(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section State Right Square Bracket: '{:?}'", c);

  *current_state = DataState::CDATASectionBracketState;

  return (None, false);
}

fn cdata_section_state_transition_eof() -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section State EOF");

  eof_in_cdata_parse_error::error(DataState::BogusDOCTYPEState.to_string());

  return (
    Some(vec![
      Token::EOFToken
    ]), 
    false
  );
}

fn cdata_section_state_transition_anything_else(
  c: Option<char>
) -> (Option<Vec<Token>>, bool) {
  println!("CDATA Section State Anything Else: '{:?}'", c);

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
  fn test_cdata_section_state_transition_right_square_bracket() {
    const C: Option<char> = Some(']');
    let mut current_state: DataState = DataState::CDATASectionState;

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = cdata_section_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionBracketState, current_state);
  }

  #[test]
  fn test_cdata_section_state_transition_eof() {
    const C: Option<char> = None;
    let mut current_state: DataState = DataState::CDATASectionState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::EOFToken
      ]), 
      false
    );
    let result = cdata_section_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionState, current_state);
  }

  #[test]
  fn test_cdata_section_state_transition_anything_else() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::CDATASectionState;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('g')
      ]),
      false
    );
    let result = cdata_section_state_transition(
      C, 
      &mut current_state
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CDATASectionState, current_state);
  }
}
