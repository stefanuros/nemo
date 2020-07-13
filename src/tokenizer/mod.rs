use crate::types::data_states::DataState;
use crate::types::tokens::Token;

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
  let mut iter = html.chars();

  let mut is_iter_empty = false;
  // Flag to check whether the next step should consume a new character or reuse the previous character
  let mut reconsume = false;

  // Loop through the chars in the html string
  while !is_iter_empty {
    // Consume next character if reconsume is false. Otherwise, reuse character
    if !reconsume {
      current_input_character = iter.next();
    }

    // Get the emitted tokens and whether the character is safe to iterate or if it should be reconsumed
    let (emitted_tokens, should_reconsume) = tokenize(
      current_input_character, 
      &mut current_state, 
      &mut return_state,
      &mut create_token
    );

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
  create_token: &mut Option<Token>
) -> (Option<Vec<Token>>, bool) {
  match current_state {
    DataState::DataState => state_transitions::data_state_transition(c, current_state, return_state, create_token),
    // DataState::RCDataState => state_transitions::rcdata_state_transition(c),
    _ => (None, false),
  }
}
