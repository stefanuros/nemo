pub struct Attribute {
  name: String,
  value: String,
  duplicate: bool
}

impl Attribute {
  pub fn get_name (&self) -> String {
    return self.name.clone();
  }

  pub fn set_name (&mut self, new_name: String) {
    self.name = new_name;
  }

  pub fn push_to_name (&mut self, c: char) {
    self.name.push(c);
  }

  pub fn get_value (&self) -> String {
    return self.value.clone();
  }

  pub fn set_value (&mut self, new_value: String) {
    self.value = new_value;
  }

  pub fn push_to_value (&mut self, c: char) {
    self.value.push(c);
  }

  pub fn is_duplicate(&self) -> bool {
    return self.duplicate;
  }

  pub fn set_duplicate(&mut self, duplicate_value: bool) {
    self.duplicate = duplicate_value;
  }
}

impl Default for Attribute {
  fn default() -> Attribute {
    return Attribute {
      name: "".to_string(),
      value: "".to_string(),
      duplicate: false
    };
  }
}
