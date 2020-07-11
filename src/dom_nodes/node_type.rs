use super::{
  element_node::ElementNode,
  text_node::TextNode,
  comment_node::CommentNode
};

pub enum NodeType {
  Element(ElementNode),
  // Element(ElementNode),
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
  pub fn get_node_name(&self) -> String {
    match self {
      NodeType::Element(ref e) => return e.get_node_name(),
      NodeType::Text(ref t) => return t.get_node_name(),
      NodeType::Comment(ref c) => return c.get_node_name(),
    }
  }
}
