use crate::types::tokenizer_types::data_states::DataState;
use crate::types::tokenizer_types::tokens::Token;
use itertools;

mod state_transitions;

pub fn init_tokenization() {
  // Read data
  // Stream html
  // encode html to U+XXXX format
  // read stream and pass into tokenizer
  // tokenize and return tokens

  let html = "
    <html>
      <body>
        <h1>Title</h1>
        <div id=\"main\" class=\"test\">
          <p>Hello <em>world</em>!</p>
        </div>
      </body>
    </html>
  ";

  // The list of emitted tokens
  let mut tokens: Vec<Token> = Vec::new();

  // A created token that can be used to keep track of tokens being built over multiple characters
  let mut create_token: Option<Token> = None;

  let mut current_state = DataState::default();
  let mut return_state = DataState::default();

  let mut current_input_character: Option<char> = None;

  // The iterator going through all of the characters in the input stream
  let mut iter = itertools::multipeek(html.chars());

  let mut is_iter_empty = false;
  // Flag to check whether the next step should consume a new character or reuse the previous character
  let mut reconsume = false;

  // Loop through the chars in the html string
  while !is_iter_empty {
    // Consume next character if reconsume is false. Otherwise, reuse character
    if !reconsume {
      current_input_character = iter.next();
    }

    // Pass the iter to the state handler for specific states
    let should_pass_iter = 
      current_state == DataState::MarkupDeclarationOpenState ||
      current_state == DataState::AfterDOCTYPENameState;

    // Get the emitted tokens and whether the character is safe to iterate or if it should be reconsumed
    let (emitted_tokens, should_reconsume) = tokenize(
      current_input_character, 
      &mut current_state, 
      &mut return_state,
      &mut create_token,
      if should_pass_iter { Some(&mut iter) } else { None },
    );

    // Deal with current_input_character reconsuming
    reconsume = should_reconsume;

    // Deal with any emitted tokens
    if emitted_tokens.is_some() {
      // Add the emitted tokens to the list of all emitted tokens
      tokens.append(&mut emitted_tokens.unwrap());
    }

    // End the loop if we're at the end of the input stream
    if current_input_character.is_none() {
      is_iter_empty = true;
    }

    // TODO Return tokens somehow (iterator?)
  }
}

fn tokenize(
  c: Option<char>, 
  current_state: &mut DataState, 
  return_state: &mut DataState,
  create_token: &mut Option<Token>,
  iter: Option<&mut itertools::MultiPeek<std::str::Chars>>
) -> (Option<Vec<Token>>, bool) {
  return match current_state {
    DataState::DataState => state_transitions::data_state_transition(c, current_state, return_state),
    DataState::RCDataState => state_transitions::rcdata_state_transition(c, current_state, return_state),
    DataState::RAWTEXTState => state_transitions::rawtext_state_transition(c, current_state),
    DataState::ScriptDataState => state_transitions::script_data_state_transition(c, current_state),
    DataState::PLAINTEXTState => state_transitions::plaintext_state_transition(c),
    DataState::TagOpenState => state_transitions::tag_open_state_transition(c, current_state, create_token),
    _ => (None, false),
  }
}

  // The following code is example code for states 42 and 56
  // let mut test_string: String = "".to_string();

  // for i in 0 .. 7 {
  //   match iter.peek() {
  //     Some(v) => test_string = [test_string, v.to_string()].concat(),
  //     None => print!("run anything else code"),
  //   }

  //   if test_string.to_ascii_lowercase() == "doctype" {
  //     print!("run doctype code");
  //   }

  //   match test_string.as_str() {
  //     "--" => print!("run -- code"),
  //     "[CDATA[" => print!("run cdata code"),
  //     _ => (),
  //   }

  //   if !"doctype".starts_with(test_string.to_ascii_lowercase().as_str()) &&
  //     !"[CDATA[".starts_with(test_string.as_str()) &&
  //     !"--".starts_with(test_string.as_str()) {
  //       print!("run anything else code")
  //   }
  // }

  // println!("{:?}", iter.peek().unwrap().to_string());
