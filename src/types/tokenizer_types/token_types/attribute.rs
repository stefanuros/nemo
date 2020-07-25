use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
  name: String,
  value: String,
  /// A boolean value which states whether the attribute is a duplicate or not
  duplicate: bool
}

impl Attribute {
  /// Creates an attribute with a char as the starting name. Everything else is default
  pub fn new(c: char) -> Attribute {
    return Attribute {
      name: c.to_string(),
      ..Attribute::default()
    };
  }

  pub fn get_name (&self) -> String {
    return self.name.clone();
  }

  pub fn set_name (&mut self, new_name: String) {
    self.name = new_name;
  }

  /// Pushes a character to add to the attribute name
  pub fn push_to_name (&mut self, c: char) {
    self.name.push(c);
  }

  pub fn get_value (&self) -> String {
    return self.value.clone();
  }

  pub fn set_value (&mut self, new_value: String) {
    self.value = new_value;
  }

  /// Pushes a character to add to the attributes value
  pub fn push_to_value (&mut self, c: char) {
    self.value.push(c);
  }

  /// Returns the duplicate boolean stating whether the attribute is a duplicate or not
  pub fn is_duplicate(&self) -> bool {
    return self.duplicate;
  }

  /// Sets the attribute duplicate value
  pub fn set_duplicate(&mut self, duplicate_value: bool) {
    self.duplicate = duplicate_value;
  }
}

impl Default for Attribute {
  /// Creates a new Attribute with all default values
  fn default() -> Attribute {
    return Attribute {
      name: "".to_string(),
      value: "".to_string(),
      duplicate: false
    };
  }
}

impl Display for Attribute {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}

