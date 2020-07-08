use std::collections::HashMap;

type AttrMap = HashMap<String, String>;


// 
// Node
// 

pub struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

impl Node {
  fn get_node_name(&self) -> String {
    return self.node_type.get_node_name()
  }
}

// 
// Node Type
// 

enum NodeType {
  Element(ElementNode),
  // Attr(AttributeNode),
  Text(TextNode),
  // CDATASection(CDATASectionNode),
  // ProcessingInstruction(ProcessingInstructionNode),
  Comment(CommentNode),
  // Document(DocumentNode),
  // DocumentType(DocumentTypeNode),
  // DocumentFragment(DocumentFragmentNode),
}

impl NodeType {
  fn get_node_name(&self) -> String {
    match *self {
      NodeType::Element(ref e) => return e.get_node_name(),
      NodeType::Text(ref t) => return t.get_node_name(),
      NodeType::Comment(ref c) => return c.get_node_name(),
    }
  }
}

// 
// Element Node
// 

struct ElementNode {
  tag_name: String,
  attributes: AttrMap,
}

impl ElementNode {
  fn get_node_name(&self) -> String {
    return self.tag_name.to_string();
  }
}

// 
// Text Node
// 

struct TextNode {
  text: String,
}

impl TextNode {
  fn get_node_name(&self) -> String {
    return "#text".to_string();
  }
}

// 
// Comment Node
// 

struct CommentNode {
  data: String,
}

impl CommentNode {
  fn get_node_name(&self) -> String {
    return "#comment".to_string();
  }
}

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

pub fn pretty_print_dom_tree(root: Node, level: usize) {
  let indent = level * 4;
  // Print the name based on the 
  print!("{:indent$}{}\n", "", root.get_node_name(), indent=indent);

  // Loop through the children
  for child in root.children {
    pretty_print_dom_tree(child, indent + 1);
  }
}
