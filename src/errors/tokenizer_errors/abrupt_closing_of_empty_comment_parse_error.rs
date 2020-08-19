pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "abrupt-closing-of-empty-comment";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters an empty comment that is abruptly closed by a U+003E (>) code point (i.e., <!--> or <!--->). The parser behaves as if the comment is closed correctly.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-abrupt-closing-of-empty-comment";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
