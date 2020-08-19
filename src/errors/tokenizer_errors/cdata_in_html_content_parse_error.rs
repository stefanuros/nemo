pub fn error(state: String) {
  const ERROR_CODE: &str = "cdata-in-html-content";
  let error_state = format!("state: {}", state);
  const ERROR_MESSAGE: &str = "This error occurs if the parser encounters a CDATA section outside of foreign content (SVG or MathML). The parser treats such CDATA sections (including leading \"[CDATA[\" and trailing \"]]\" strings) as comments.";
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-cdata-in-html-content";
  println!(
    "ERROR:  {}\n\t{}\n\n\t{}\n\tSee here for more info: {}", 
    ERROR_CODE, 
    error_state, 
    ERROR_MESSAGE, 
    ERROR_URL
  );
}
