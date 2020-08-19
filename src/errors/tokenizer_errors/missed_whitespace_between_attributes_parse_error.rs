pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "missing-whitespace-between-attributes";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters attributes that are not separated by ASCII whitespace (e.g., <div id=\"foo\"class=\"bar\">). In this case the parser behaves as if ASCII whitespace is present.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-whitespace-between-attributes";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
