pub struct CommentNode {
  pub data: String,
}

impl CommentNode {
  pub fn get_node_name(&self) -> String {
    return "#comment".to_string();
  }
}
