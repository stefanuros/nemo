pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "unexpected-question-mark-instead-of-tag-name";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a U+003F (?) code point where first code point of a start tag name is expected. The U+003F (?) and all content that follows up to a U+003E (>) code point (if present) or to the end of the input stream is treated as a comment.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-unexpected-question-mark-instead-of-tag-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
