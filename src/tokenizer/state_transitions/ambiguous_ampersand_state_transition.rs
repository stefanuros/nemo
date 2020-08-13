use crate::errors::tokenizer_errors::unknown_named_character_reference_parse_error;
use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn ambiguous_ampersand_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  return_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Ambiguous Ampersand State c: '{:?}'", c);

  return match c {
    Some(x) if x.is_ascii_alphanumeric() => ambiguous_ampersand_state_transition_ascii_alphanumeric(c, return_state, current_token),
    Some('\u{003B}') => ambiguous_ampersand_state_transition_semicolon(c, current_state, return_state),
    _ => ambiguous_ampersand_state_transition_anything_else(c, current_state, return_state)
  }
}

fn ambiguous_ampersand_state_transition_ascii_alphanumeric(
  c: Option<char>,
  return_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Ambiguous Ampersand State Ascii Alphanumeric: '{:?}'", c);

  let mut emitted_tokens: Option<Vec<Token>> = None;

  if is_consumed_as_attribute(return_state) {
    if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
      tag_token.push_to_current_attribute_value(c.unwrap());
    }
  }
  else {
    emitted_tokens = Some(vec![
      Token::CharacterToken(c.unwrap())
    ]);
  }

  return (emitted_tokens, false);
}

fn ambiguous_ampersand_state_transition_semicolon(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Ambiguous Ampersand State Number Sign: '{:?}'", c);

  unknown_named_character_reference_parse_error::error(DataState::AmbiguousAmpersandState.to_string(), c.unwrap());

  *current_state = return_state.clone();

  return (None, true);
}

fn ambiguous_ampersand_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  return_state: &mut DataState
) -> (Option<Vec<Token>>, bool) {
  println!("Ambiguous Ampersand State Anything Else: '{:?}'", c);

  *current_state = return_state.clone();

  return (None, true);
}

fn is_consumed_as_attribute(return_state: &DataState) -> bool {
  return return_state == &DataState::AttributeValueDoubleQuotedState ||
  return_state == &DataState::AttributeValueSingleQuotedState ||
  return_state == &DataState::AttributeValueUnquotedState;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::{
    Attribute,
    TagToken
  };
  
  #[test]
  fn test_ambiguous_ampersand_state_transition_ascii_alphanumeric_consumed_as_attribute() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::AmbiguousAmpersandState;
    let mut return_state: DataState = DataState::AttributeValueSingleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyzg".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = ambiguous_ampersand_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AmbiguousAmpersandState, current_state);
    assert_eq!(DataState::AttributeValueSingleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_ambiguous_ampersand_state_transition_ascii_alphanumeric_not_consumed_as_attribute() {
    const C: Option<char> = Some('g');
    let mut current_state: DataState = DataState::AmbiguousAmpersandState;
    let mut return_state: DataState = DataState::ScriptDataDDoubleEscapedDashState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('g')
      ]), 
      false
    );
    let result = ambiguous_ampersand_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AmbiguousAmpersandState, current_state);
    assert_eq!(DataState::ScriptDataDDoubleEscapedDashState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_ambiguous_ampersand_state_transition_ascii_alphanumeric_semicolon() {
    const C: Option<char> = Some(';');
    let mut current_state: DataState = DataState::AmbiguousAmpersandState;
    let mut return_state: DataState = DataState::AttributeValueSingleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = ambiguous_ampersand_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueSingleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueSingleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
  }

  #[test]
  fn test_ambiguous_ampersand_state_transition_ascii_alphanumeric_anything_else() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::AmbiguousAmpersandState;
    let mut return_state: DataState = DataState::AttributeValueSingleQuotedState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));

    let expected_current_token = Some(Token::StartTagToken(TagToken {
      attributes: vec![
        Attribute {
          value: "xyz".to_string(),
          ..Attribute::default()
        }
      ],  
      ..TagToken::default()
    }));
    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = ambiguous_ampersand_state_transition(
      C, 
      &mut current_state,
      &mut return_state,
      &mut current_token
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::AttributeValueSingleQuotedState, current_state);
    assert_eq!(DataState::AttributeValueSingleQuotedState, return_state);
    assert_eq!(expected_current_token, current_token);
  }
}
