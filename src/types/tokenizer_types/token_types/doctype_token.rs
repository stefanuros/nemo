use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq, Clone)]
pub struct DoctypeToken {
  force_quirks: bool
}

impl DoctypeToken {
  pub fn is_force_quirks(&self) -> bool {
    return self.force_quirks;
  }

  pub fn set_force_quirks(&mut self, force_quirks: bool) {
    self.force_quirks = force_quirks;
  }
}

impl Default for DoctypeToken {
  /// Creates a new DoctypeToken with all default values
  fn default() -> DoctypeToken {
    return DoctypeToken {
      force_quirks: false
    };
  }
}

impl Display for DoctypeToken {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
