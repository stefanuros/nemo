pub fn error(state: String) {
  const ERROR_CODE: &str = "eof-in-script-html-comment-like-text";
  let error_state = format!("state: {}", state);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters the end of the input stream in text that resembles an HTML comment inside script element content (e.g., <script><!-- foo).";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-eof-in-script-html-comment-like-text";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
