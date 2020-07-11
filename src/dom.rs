use crate::dom_nodes::{
  element_node::ElementNode,
  text_node::TextNode,
  comment_node::CommentNode,
  node::Node,
  node_type::NodeType,
};
use crate::types::AttrMap;

pub fn text(data: &str) -> Node {
  return Node { 
    children: Vec::new(),
    node_type: NodeType::Text(TextNode {
      text: data.to_string(),
    }),
  }
}

pub fn elem(name: &str, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementNode {
        tag_name: name.to_string(),
        attributes: attrs,
    }),
  }
}

pub fn pretty_print_dom_tree(root: &Node, level: usize) {
  let indent = level * 4;
  // Print the name based on the 
  print!("{:indent$}{}\n", "", root.get_node_name(), indent=indent);

  // Loop through the children
  for child in &root.children {
    pretty_print_dom_tree(child, indent + 1);
  }
}
