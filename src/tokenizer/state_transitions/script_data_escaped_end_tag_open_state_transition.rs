use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn script_data_escaped_end_tag_open_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped End Tag Open State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_alphabetic() => script_data_escaped_end_tag_open_state_transition_ascii_alpha(c, current_state, current_token),
    _ => script_data_escaped_end_tag_open_state_transition_anything_else(c, current_state),
  }
}

fn script_data_escaped_end_tag_open_state_transition_ascii_alpha(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped End Tag Open State Ascii Alpha: '{:?}'", c);

  *current_token = Some(Token::EndTagToken("".to_string()));
  *current_state = DataState::ScriptDataEscapedEndTagNameState;

  return (None, true);
}

fn script_data_escaped_end_tag_open_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Script Data Escaped End Tag Open State Anything Else: '{:?}'", c);

  *current_state = DataState::ScriptDataEscapedState;

  return(
    Some(vec![
      Token::CharacterToken('\u{003C}'),
      Token::CharacterToken('\u{002F}'),
    ]), 
    true
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_data_escaped_end_tag_open_state_transition_ascii_alpha() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::ScriptDataEscapedEndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = script_data_escaped_end_tag_open_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedEndTagNameState, current_state);
    assert_eq!(Some(Token::EndTagToken("".to_string())), current_token);
  }

  #[test]
  fn test_script_data_escaped_end_tag_open_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::ScriptDataEscapedEndTagOpenState;
    let mut current_token: Option<Token> = None;

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
        Token::CharacterToken('\u{002F}'),
      ]), 
      true
    );
    let result = script_data_escaped_end_tag_open_state_transition(
      C, 
      &mut current_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::ScriptDataEscapedState, current_state);
    assert_eq!(None, current_token);
  }
}
