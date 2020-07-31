use crate::errors::tokenizer_errors::{
  incorrectly_opened_comment_parse_error,
  cdata_in_html_content_parse_error
};
use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;

pub fn markup_declaration_open_state_transition(
  c: Option<char>,
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("Markup Declaration Open State c: '{:?}'", c);

  // if c is none, it won't match anything
  if let None = c {
    return markup_declaration_open_state_transition_anything_else("".to_string(), current_state, current_token)
  }

  // c is the starting character
  let mut peek = c.unwrap().to_string();

  // Loop to 6 because the longest string we're matching is 7 characters and c is the first character
  for _ in 0..6 {
    // Peek at the next character
    match iter.peek() {
      Some(x) => peek.push(x.clone()),
      None => break
    };

    println!("Markup Declaration Open State c: '{:?}'", peek);

    // Catch a case-insensitive match for the word DOCTYPE
    if peek.to_ascii_uppercase() == "DOCTYPE" {
      return markup_declaration_open_state_transition_doctype(current_state, iter);
    }

    // Match the other strings
    match peek.as_str() {
      "--" => return markup_declaration_open_state_transition_hyphens(current_state, current_token, iter),
      "[CDATA[" => return markup_declaration_open_state_transition_cdata(current_state, current_token, iter),
      _ => ()
    };
  }

  // If it doesn't match anything else, go here
  return markup_declaration_open_state_transition_anything_else(peek, current_state, current_token);
}

fn markup_declaration_open_state_transition_hyphens(
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("Markup Declaration Open State Hyphens");

  // Skip 2 elements of the iterator since we used them to get to here
  iter.nth(0);

  *current_token = Some(Token::empty_comment());
  *current_state = DataState::CommentStartState;
  
  return (None, false);
}

fn markup_declaration_open_state_transition_doctype(
  current_state: &mut DataState,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("Markup Declaration Open State DOCTYPE");

  iter.nth(5);

  *current_state = DataState::DOCTYPEState;
  
  return (None, false);
}

fn markup_declaration_open_state_transition_cdata(
  current_state: &mut DataState,
  current_token: &mut Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  println!("Markup Declaration Open State CDATA");
  
  cdata_in_html_content_parse_error::error(DataState::MarkupDeclarationOpenState.to_string());

  iter.nth(5);

  *current_token = Some(Token::new_comment("[CDATA["));
  *current_state = DataState::BogusCommentState;
  
  // TODO CDATA Logic
  // ! Not worrying about CDATA details now. This will likely depend on the tree 
  // ! construction stage. Defaulting to cdata-in-html parse error

  return (None, false);
}

fn markup_declaration_open_state_transition_anything_else(
  peek: String,
  current_state: &mut DataState,
  current_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  println!("Markup Declaration Open State Anything Else");

  incorrectly_opened_comment_parse_error::error(DataState::MarkupDeclarationOpenState.to_string(), peek);

  *current_token = Some(Token::empty_comment());
  *current_state = DataState::BogusCommentState;

  return (None, true);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn markup_declaration_open_state_transition_hyphens() {
    const C: Option<char> = Some('-');
    let mut current_state: DataState = DataState::MarkupDeclarationOpenState;
    let mut current_token: Option<Token> = Some(
      Token::CommentToken("comment".to_string())
    );
    let mut iter = itertools::multipeek("-A".chars());

    let expected_current_token: Option<Token> = Some(
      Token::CommentToken("".to_string())
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = markup_declaration_open_state_transition(
      C,
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::CommentStartState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn markup_declaration_open_state_transition_doctype() {
    const C: Option<char> = Some('d');
    let mut current_state: DataState = DataState::MarkupDeclarationOpenState;
    let mut current_token: Option<Token> = Some(
      Token::CommentToken("comment".to_string())
    );
    let mut iter = itertools::multipeek("OCTYPEA".chars());

    let expected_current_token: Option<Token> = Some(
      Token::CommentToken("comment".to_string())
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = markup_declaration_open_state_transition(
      C,
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::DOCTYPEState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn markup_declaration_open_state_transition_cdata() {
    const C: Option<char> = Some('[');
    let mut current_state: DataState = DataState::MarkupDeclarationOpenState;
    let mut current_token: Option<Token> = Some(
      Token::CommentToken("comment".to_string())
    );
    let mut iter = itertools::multipeek("CDATA[A".chars());

    let expected_current_token: Option<Token> = Some(
      Token::CommentToken("[CDATA[".to_string())
    );

    let expected: (Option<Vec<Token>>, bool) = (None, false);
    let result = markup_declaration_open_state_transition(
      C,
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('A'));
  }

  #[test]
  fn markup_declaration_open_state_transition_anything_else() {
    const C: Option<char> = Some('a');
    let mut current_state: DataState = DataState::MarkupDeclarationOpenState;
    let mut current_token: Option<Token> = Some(
      Token::CommentToken("comment".to_string())
    );
    let mut iter = itertools::multipeek("bc".chars());

    let expected_current_token: Option<Token> = Some(
      Token::CommentToken("".to_string())
    );

    let expected: (Option<Vec<Token>>, bool) = (None, true);
    let result = markup_declaration_open_state_transition(
      C,
      &mut current_state,
      &mut current_token,
      &mut iter
    );

    assert_eq!(expected, result);
    assert_eq!(DataState::BogusCommentState, current_state);
    assert_eq!(expected_current_token, current_token);
    assert_eq!(iter.next(), Some('b'));
  }
}
