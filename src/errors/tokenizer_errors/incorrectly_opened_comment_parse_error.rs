pub fn error(state: String, s: String) {
  const ERROR_CODE: &str = "incorrectly-opened-comment";
  let error_state = format!("state: {}, string: {}", state, s.escape_unicode());
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters the \"<!\" code point sequence that is not immidiately followed by two U+002D (-) code points and that is not the start of a DOCTYPE or a CDATA section. All content that follows the \"<!\" code point sequence up to a U+003E (>) code point (if present) or to the end of the input stream is treated as a comment.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-incorrectly-opened-comment";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
