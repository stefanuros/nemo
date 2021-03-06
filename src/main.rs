#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

mod dom_nodes;
mod types;
mod tokenizer;
mod tree_constructor;
mod errors;
mod dom;

fn main() {

  let text = dom::text("text");
  let elem = dom::elem("elem", HashMap::new(), vec![text]);

  println!("Hello, world!");

  // dom::pretty_print_dom_tree(&elem, 0);
  // dom::pretty_print_dom_tree(&elem, 0);

  // tokenizer::tokenize(types::data_states::DataState::RCDataState, '<')
  tokenizer::init_tokenization();

}

