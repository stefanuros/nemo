type AttrMap = HashMap<String, String>;

struct Node {
  children: Vec<Node>,
  nodeType: NodeType,
}

enum NodeType {
  Element(ElementNode),
  // Attr(AttributeNode),
  Text(TextNode),
  // CDATASection(CDATASectionNode),
  // ProcessingInstruction(ProcessingInstructionNode),
  // Comment(CommentNode),
  // Document(DocumentNode),
  // DocumentType(DocumentTypeNode),
  // DocumentFragment(DocumentFragmentNode),
}

struct ElementNode {
  tagName: String,
  attributes: AttrMap,
}


// impl ElementNode {
//   pub fn get() -> String {
//     return self.tagName;
//   }
// }

struct TextNode {
  text: String,
}

// impl TextNode {
//   pub fn get() -> String {
//     return "#text";
//   }
// }

fn text(data: String) -> Node {
  return Node { 
    children: Vec::new(),
    nodeType: NodeType::Text(TextNode {
      text: data,
    }),
  }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementNode {
        tagName: name,
        attributes: attrs,
    }),
  }
}
