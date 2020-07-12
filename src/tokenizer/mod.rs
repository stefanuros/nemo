use crate::types::data_states::DataState;

mod state_transitions;

fn initTokenization() {
  // Read data
  // Stream html
  // encode html to U+XXXX format
  // read stream and pass into tokenizer
  // tokenize and return tokens
}

fn tokenize(state: DataState, c: char) {
  match state {
    DataState::DataState() => state_transitions::data_state_transition(c),
    DataState::RCDataState() => state_transitions::rcdata_state_transition(c),
    _ => println!("Else"),
  }
}
