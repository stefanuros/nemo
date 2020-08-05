pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "abrupt-doctype-system-identifier";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a U+003E (>) code point in the DOCTYPE system identifier (e.g., <!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"foo>). In such a case, if the DOCTYPE is correctly placed as a document preamble, the parser sets the Document to quirks mode.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-abrupt-doctype-system-identifier";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
