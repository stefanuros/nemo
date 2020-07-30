use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq, Clone)]
pub struct DoctypeToken {
  pub force_quirks: bool,
  pub name: Option<String>
}

impl DoctypeToken {
  /// Create a new DoctypeToken with a string as the name
  pub fn new(name: &str) -> DoctypeToken {
    return DoctypeToken {
      name: Some(name.to_string()),
      ..DoctypeToken::default()
    };
  }

  /// Create a new DoctypeToken with a character as the name
  pub fn new_c(name: char) -> DoctypeToken {
    return DoctypeToken {
      name: Some(name.to_string()),
      ..DoctypeToken::default()
    };
  }

  pub fn is_force_quirks(&self) -> bool {
    return self.force_quirks;
  }

  pub fn set_force_quirks(&mut self, force_quirks: bool) {
    self.force_quirks = force_quirks;
  }

  pub fn push_to_name(&mut self, c: char) {
    match self.name {
      Some(ref mut name) => name.push(c),
      None => self.name = Some(c.to_string())
    };
  }
}

impl Default for DoctypeToken {
  /// Creates a new DoctypeToken with all default values
  fn default() -> DoctypeToken {
    return DoctypeToken {
      force_quirks: false,
      name: None
    };
  }
}

impl Display for DoctypeToken {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
