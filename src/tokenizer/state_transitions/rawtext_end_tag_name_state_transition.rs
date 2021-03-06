use crate::types::tokenizer_types::{
  data_states::DataState,
  tokens::Token
};

pub fn rawtext_end_tag_name_state_transition(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
  recent_start_tag: &Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State c: '{:?}'", c);

  return match c {
    Some('\u{0009}') |
    Some('\u{000A}') |
    Some('\u{000C}') |
    Some('\u{0020}') => rawtext_end_tag_name_state_transition_whitespace(c, current_state, current_token, temporary_buffer, recent_start_tag),
    Some('\u{002F}') => rawtext_end_tag_name_state_transition_solidus(c, current_state, current_token, temporary_buffer, recent_start_tag),
    Some('\u{003E}') => rawtext_end_tag_name_state_transition_greater_than_sign(c, current_state, current_token, temporary_buffer, recent_start_tag),
    Some(x) if x.is_ascii_uppercase() => rawtext_end_tag_name_state_transition_ascii_upper_alpha(c, current_token, temporary_buffer),
    Some(x) if x.is_ascii_lowercase() => rawtext_end_tag_name_state_transition_ascii_lower_alpha(c, current_token, temporary_buffer),
    _ => rawtext_end_tag_name_state_transition_anything_else(c, current_state, temporary_buffer),
  }
}

fn rawtext_end_tag_name_state_transition_whitespace(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &String,
  recent_start_tag: &Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Whitespace: '{:?}'", c);

  // Check if the current_token is an "appropriate end tag token"
  if let (Some(Token::StartTagToken(ref start_tag)), Some(Token::EndTagToken(ref end_tag))) = (&recent_start_tag, &current_token) {
    if start_tag == end_tag {
      *current_state = DataState::BeforeAttributeNameState;
      return (None, false);
    }
  }

  return rawtext_end_tag_name_state_transition_anything_else(c, current_state, temporary_buffer);
}

fn rawtext_end_tag_name_state_transition_solidus(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &String,
  recent_start_tag: &Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Solidus: '{:?}'", c);

  // Check if the current_token is an "appropriate end tag token"
  if let (Some(Token::StartTagToken(ref start_tag)), Some(Token::EndTagToken(ref end_tag))) = (&recent_start_tag, &current_token) {
    if start_tag == end_tag {
      *current_state = DataState::SelfClosingStartTagState;
      return (None, false);
    }
  }

  return rawtext_end_tag_name_state_transition_anything_else(c, current_state, temporary_buffer);
}

fn rawtext_end_tag_name_state_transition_greater_than_sign(
  c: Option<char>, 
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  temporary_buffer: &String,
  recent_start_tag: &Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Exclamation Mark: '{:?}'", c);

  // Check if the current_token is an "appropriate end tag token"
  if let (Some(Token::StartTagToken(ref start_tag)), Some(Token::EndTagToken(ref end_tag))) = (&recent_start_tag, &current_token) {
    if start_tag == end_tag {
      *current_state = DataState::DataState;
      return (
        Some(vec![
          current_token.clone().unwrap()
        ]), 
        false
      );
    }
  }

  return rawtext_end_tag_name_state_transition_anything_else(c, current_state, temporary_buffer);
}

fn rawtext_end_tag_name_state_transition_ascii_upper_alpha(
  c: Option<char>,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Ascii Upper Alpha: '{:?}'", c);

  // Add to the current tag token value
  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_tag_name(c.unwrap().to_ascii_lowercase());
  }

  temporary_buffer.push(c.unwrap());
  
  return (None, false);
}

fn rawtext_end_tag_name_state_transition_ascii_lower_alpha(
  c: Option<char>,
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Ascii Lower Alpha: '{:?}'", c);

  // Add to the current tag token value
  if let Some(Token::StartTagToken(ref mut tag_token)) | Some(Token::EndTagToken(ref mut tag_token)) = current_token {
    tag_token.push_to_tag_name(c.unwrap());
  }

  temporary_buffer.push(c.unwrap());
  
  return (None, false);
}

fn rawtext_end_tag_name_state_transition_anything_else(
  c: Option<char>,
  current_state: &mut DataState,
  temporary_buffer: &String
) -> (Option<Vec<Token>>, bool) {
  println!("RAWTEXT End Tag Name State Anything Else: '{:?}'", c);

  let mut output_tokens = vec![
    Token::CharacterToken('\u{003C}'),
    Token::CharacterToken('\u{002F}')
  ];

  // Emit each character in the temporary buffer as a character token
  for buffer_c in temporary_buffer.chars() {
    output_tokens.push(Token::CharacterToken(buffer_c));
  }

  *current_state = DataState::RAWTEXTState;

  return(Some(output_tokens), true);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::tokenizer_types::token_types::TagToken;

  #[test]
  fn test_rawtext_end_tag_name_state_transition_whitespace_appropriate() {
    const C: Option<char> = Some('\t');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BeforeAttributeNameState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("div"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_whitespace_incorrect_token() {
    const C: Option<char> = Some('\t');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::StartTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
        Token::CharacterToken('\u{002F}'),
        Token::CharacterToken('d'),
        Token::CharacterToken('i'),
        Token::CharacterToken('v'),
      ]), 
      true
    );
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
    assert_eq!(Some(Token::StartTagToken(TagToken::new("div"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_whitespace_incorrect_tag_value() {
    const C: Option<char> = Some('\t');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div1")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
        Token::CharacterToken('\u{002F}'),
        Token::CharacterToken('d'),
        Token::CharacterToken('i'),
        Token::CharacterToken('v'),
      ]), 
      true
    );
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("div1"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_solidus() {
    const C: Option<char> = Some('/');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::SelfClosingStartTagState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("div"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_greater_than_sign() {
    const C: Option<char> = Some('>');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::EndTagToken(TagToken::new("div"))
      ]), 
      false
    );
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DataState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("div"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_ascii_upper_alpha() {
    const C: Option<char> = Some('A');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTEndTagNameState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("diva"))), current_token);
    assert_eq!("divA".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_ascii_lower_alpha() {
    const C: Option<char> = Some('a');
    let mut current_state: DataState = DataState::RAWTEXTEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::default()));

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTEndTagNameState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("diva"))), current_token);
    assert_eq!("diva".to_string(), temporary_buffer);
  }

  #[test]
  fn test_rawtext_end_tag_name_state_transition_anything_else() {
    const C: Option<char> = Some('7');
    let mut current_state: DataState = DataState::RCDATAEndTagNameState;
    let mut current_token: Option<Token> = Some(Token::EndTagToken(TagToken::new("div")));
    let mut temporary_buffer = "div".to_string();
    let recent_start_tag = Some(Token::StartTagToken(TagToken::new("div")));

    let expected: (Option<Vec<Token>>, bool) = (
      Some(vec![
        Token::CharacterToken('\u{003C}'),
        Token::CharacterToken('\u{002F}'),
        Token::CharacterToken('d'),
        Token::CharacterToken('i'),
        Token::CharacterToken('v'),
      ]), 
      true
    );
    let result = rawtext_end_tag_name_state_transition(
      C, 
      &mut current_state, 
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::RAWTEXTState, current_state);
    assert_eq!(Some(Token::EndTagToken(TagToken::new("div"))), current_token);
    assert_eq!("div".to_string(), temporary_buffer);
  }
}
