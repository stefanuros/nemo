pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "missing-doctype-name";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a DOCTYPE that is missing a name (e.g., <!DOCTYPE>). In such a case, if the DOCTYPE is correctly placed as a document preamble, the parser sets the Document to quirks mode.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-doctype-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
