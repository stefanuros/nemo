pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "missing-whitespace-before-doctype-name";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a DOCTYPE whose \"DOCTYPE\" keyword and name are not separated by ASCII whitespace. In this case the parser behaves as if ASCII whitespace is present.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-whitespace-before-doctype-name";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
