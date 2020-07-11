use super::node_type::NodeType;
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

impl Node {
  pub fn get_node_name(&self) -> String {
    return self.node_type.get_node_name()
  }
}
