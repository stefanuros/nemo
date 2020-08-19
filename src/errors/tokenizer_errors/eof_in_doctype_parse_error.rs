pub fn error(state: String) {
  const ERROR_CODE: &str = "eof-in-doctype";
  let error_state = format!("state: {}", state);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters the end of the input stream in a DOCTYPE. In such a case, if the DOCTYPE is correctly placed as a document preamble, the parser sets the Document to quirks mode.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-eof-in-doctype";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
