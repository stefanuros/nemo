pub fn error(state: String, c: u32) {
  const ERROR_CODE: &str = "character-reference-outside-unicode-range";
  let error_state = format!("state: {}, char: {:?}", state, c);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a numeric character reference that references a code point that is greater than the valid Unicode range. The parser resolves such a character reference to a U+FFFD REPLACEMENT CHARACTER.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-character-reference-outside-unicode-range";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
