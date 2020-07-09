struct Parser {
  pos: usize,
  input: String,
}

impl Parser {
  // Read the current character without consuming it.
  fn next_char(&self) -> char {
    // self.input is the full input string
    // self.pos.. gets a string slice from the current position to the end of the string
    // chars creates an iterator for the chars in the string
    // next gets the next value in the iterator (currently the first value since the iter was just made)
    // TODO unwrap does something with errors, not sure
    self.input[self.pos..].chars().next().unwrap()
  }

  // Do the next characters start with the given string?
  fn starts_with(&self, s: &str) -> bool {
    self.input[self.pos..].starts_with(s)
  }

  // Return true if all input is consumed.
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }
}
