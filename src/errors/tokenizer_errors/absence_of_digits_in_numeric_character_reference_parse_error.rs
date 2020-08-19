pub fn error(state: String, c: Option<char>) {
  const ERROR_CODE: &str = "absence-of-digits-in-numeric-character-reference";
  let error_state = format!("state: {}, char: {:?}", state, c);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a numeric character reference that doesn't contain any digits (e.g., &#qux;). In this case the parser doesn't resolve the character reference.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-absence-of-digits-in-numeric-character-reference";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
