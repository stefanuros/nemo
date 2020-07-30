pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "incorrectly-closed-comment";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a comment that is closed by the \"--!>\" code point sequence. The parser treats such comments as if they are correctly closed by the \"-->\" code point sequence.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-incorrectly-closed-comment";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
