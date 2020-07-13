use crate::types::data_states::DataState;

pub fn error(state: String, c: char) {
  // TODO Move this to static assets file
  const ERROR_CODE: &str = "unexpected-null-character";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a U+0000 NULL code point in the input stream in certain positions. In general, such code points are either completely ignored or, for security reasons, replaced with a U+FFFD REPLACEMENT CHARACTER.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-unexpected-equals-sign-before-attribute-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
