pub fn error(state: String, c: Option<char>) {
  const ERROR_CODE: &str = "missing-semicolon-after-character-reference";
  let error_state = format!("state: {}, char: {:?}", state, c);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a character reference that is not terminated by a U+003B (;) code point. Usually the parser behaves as if character reference is terminated by the U+003B (;) code point; however, there are some ambiguous cases in which the parser includes subsequent code points in the character reference.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-missing-semicolon-after-character-reference";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
