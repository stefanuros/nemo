pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "unexpected-character-in-attribute-name";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a U+0022 (\"), U+0027 ('), or U+003C (<) code point in an attribute name. The parser includes such code points in the attribute name.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-unexpected-character-in-attribute-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
