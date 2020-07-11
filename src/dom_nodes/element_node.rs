use crate::types::AttrMap;

pub struct ElementNode {
  pub tag_name: String,
  pub attributes: AttrMap,
}

impl ElementNode {
  pub fn get_node_name(&self) -> String {
    return self.tag_name.to_string();
  }
}
