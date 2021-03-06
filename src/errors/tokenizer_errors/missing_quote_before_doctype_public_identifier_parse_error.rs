pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "missing-quote-before-doctype-public-identifier";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters the DOCTYPE public identifier that is not preceded by a quote (e.g., <!DOCTYPE html PUBLIC -//W3C//DTD HTML 4.01//EN\">). In such a case, the parser ignores the public identifier, and if the DOCTYPE is correctly placed as a document preamble, sets the Document to quirks mode.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-quote-before-doctype-public-identifier";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
