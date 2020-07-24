use super::Attribute;

pub struct TagToken {
  attributes: Vec<Attribute>,
  tag_name: String
}

impl TagToken {
  // Sets the tag name with a given string
  fn set_tag_name (&mut self, new_tag_name: String) {
    self.tag_name = new_tag_name;
  }

  // Pushes a char to the tag name
  fn push_to_tag_name (&mut self, c: char) {
    self.tag_name.push(c);
  }

  // Returns the current attribute
  fn get_current_attribute (&mut self) -> Option<&mut Attribute> {
    return self.attributes.last_mut();
  }

  // Creates a new default attribute, adds it to the list, and returns a mutable reference to it
  fn add_new_attribute(&mut self) -> Option<&mut Attribute> {
    self.attributes.push(Attribute::default());

    return self.get_current_attribute();
  }

  // Sets the is_duplicate value for the current attribute
  fn update_current_attribute_duplicate_flag(&mut self) -> bool {

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
  fn default() -> TagToken {
    return TagToken {
      attributes: vec![],
      tag_name: "".to_string()
    };
  }
}
