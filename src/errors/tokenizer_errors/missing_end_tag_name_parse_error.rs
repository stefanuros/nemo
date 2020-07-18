pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "missing-end-tag-name";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a U+003E (>) code point where an end tag name is expected, i.e., </>. The parser completely ignores whole \"</>\" code point sequence.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-end-tag-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
