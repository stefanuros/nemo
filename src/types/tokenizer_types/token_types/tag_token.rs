use std::fmt::{
  Display,
  Formatter,
  Result
};

use super::Attribute;

#[derive(Debug, PartialEq, Clone)]
pub struct TagToken {
  /// The list of attributes associated with this tag
  /// The last attribute in the list is the current_attribute
  pub attributes: Vec<Attribute>,
  /// The name of the tag
  pub tag_name: String
}

impl TagToken {
  /// Create a new TagToken with a string as the tag name
  pub fn new(s: &str) -> TagToken {
    return TagToken {
      attributes: vec![],
      tag_name: s.to_string()
    };
  }

  fn get_tag_name (&self) -> String {
    return self.tag_name.clone();
  }

  fn set_tag_name (&mut self, new_tag_name: String) {
    self.tag_name = new_tag_name;
  }

  /// Pushes a char to the tag name
  pub fn push_to_tag_name (&mut self, c: char) {
    self.tag_name.push(c);
  }

  /// Returns the current attribute (the last attribute in the list or None if the
  /// the list of attributes is empty)
  fn get_current_attribute (&mut self) -> Option<&mut Attribute> {
    return self.attributes.last_mut();
  }

  /// Pushes a character to the name of the current attribute
  pub fn push_to_current_attribute_name (&mut self, c: char) {
    if let Some(attr) = self.get_current_attribute() {
      attr.push_to_name(c);
    }
  }

  /// Pushes a character to the value of the current attribute
  pub fn push_to_current_attribute_value (&mut self, c: char) {
    if let Some(attr) = self.get_current_attribute() {
      attr.push_to_value(c);
    }
  }

  /// Creates a new default attribute, adds it to the list, and returns a mutable reference to it
  pub fn add_default_attribute(&mut self) -> &mut Attribute {
    self.attributes.push(Attribute::default());

    return self.get_current_attribute().unwrap();
  }

  /// Creates a new attribute with a specific character as the start of the attribute name.
  /// Returns a mutable reference to the newly created attribute
  pub fn add_new_attribute(&mut self, c: char) -> &mut Attribute {
    self.attributes.push(Attribute::new(c));

    return self.get_current_attribute().unwrap();
  }

  /// Sets the is_duplicate value for the current attribute
  pub fn update_current_attribute_duplicate_flag(&mut self) -> bool {

    let current_attribute_name = match self.get_current_attribute() {
      Some(current_attribute) => current_attribute.get_name(),
      None => return false,
    };

    // Count the number of duplicate values and if its 1 or more, there is a duplicate
    // Start at -1 to account for checking current_attribute against current_attribute
    let is_duplicate = self.attributes.iter()
    .fold(-1, |num_dups, attr| {
      if attr.get_name() == current_attribute_name {
        return num_dups + 1;
      }
      return num_dups;
    }) >= 1;

    if is_duplicate {
      self.get_current_attribute().unwrap().set_duplicate(true);
    }

    return is_duplicate;
  }
}

impl Default for TagToken {
  /// Creates a new TagToken with all default values
  fn default() -> TagToken {
    return TagToken {
      attributes: vec![],
      tag_name: "".to_string()
    };
  }
}

impl Display for TagToken {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
