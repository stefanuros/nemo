pub fn error(state: String, c: u32) {
  const ERROR_CODE: &str = "null-character-reference";
  let error_state = format!("state: {}, char: {:?}", state, c);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a numeric character reference that references a U+0000 NULL code point. The parser resolves such character references to a U+FFFD REPLACEMENT CHARACTER.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-null-character-reference";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
