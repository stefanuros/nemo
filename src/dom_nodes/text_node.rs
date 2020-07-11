pub struct TextNode {
  pub text: String,
}

impl TextNode {
  pub fn get_node_name(&self) -> String {
    return "#text".to_string();
  }
}
