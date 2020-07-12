pub fn error() {
  // TODO Move this to static assets file
  const ERROR_URL: &str = "https://html.spec.whatwg.org/multipage/parsing.html#parse-error-unexpected-equals-sign-before-attribute-name";
  println!(
    "This error occurs if the parser encounters a U+0000 NULL code point in the 
    input stream in certain positions. In general, such code points are either 
    completely ignored or, for security reasons, replaced with a U+FFFD 
    REPLACEMENT CHARACTER.\nSee here for more info: {}",
    ERROR_URL
  );
}
