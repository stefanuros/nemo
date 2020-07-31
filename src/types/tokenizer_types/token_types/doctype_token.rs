use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq, Clone)]
pub struct DoctypeToken {
  pub force_quirks: bool,
  pub name: Option<String>,
  pub public_identifier: Option<String>,
  pub system_identifier: Option<String>
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

  pub fn set_public_identifier(&mut self, public_identifier: &str) {
    self.public_identifier = Some(public_identifier.to_string());
  }

  pub fn push_to_public_identifier(&mut self, c: char) {
    match self.public_identifier {
      Some(ref mut public_identifier) => public_identifier.push(c),
      None => self.public_identifier = Some(c.to_string())
    };
  }

  pub fn set_system_identifier(&mut self, system_identifier: &str) {
    self.system_identifier = Some(system_identifier.to_string());
  }

  pub fn push_to_system_identifier(&mut self, c: char) {
    match self.system_identifier {
      Some(ref mut system_identifier) => system_identifier.push(c),
      None => self.system_identifier = Some(c.to_string())
    };
  }
}

impl Default for DoctypeToken {
  /// Creates a new DoctypeToken with all default values
  fn default() -> DoctypeToken {
    return DoctypeToken {
      force_quirks: false,
      name: None,
      public_identifier: None,
      system_identifier: None
    };
  }
}

impl Display for DoctypeToken {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
