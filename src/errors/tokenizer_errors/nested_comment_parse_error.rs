pub fn error(state: String, c: char) {
  const ERROR_CODE: &str = "nested-comment";
  let error_state = format!("state: {}, char: {}", state, c.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a nested comment (e.g., <!-- <!-- nested --> -->). Such a comment will be closed by the first occuring \"-->\" code point sequence and everything that follows will be treated as markup.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-nested-comment";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
