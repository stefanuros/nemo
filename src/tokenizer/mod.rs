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
  let mut current_token: Option<Token> = None;

  let mut current_state = DataState::default();
  let mut return_state = DataState::default();

  let mut current_input_character: Option<char> = None;

  // The iterator going through all of the characters in the input stream
  let mut iter = itertools::multipeek(html.chars());

  let mut temporary_buffer: String = "".to_string();
  let mut recent_start_tag: Option<Token> = None;

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
      &mut current_token,
      &mut temporary_buffer,
      &recent_start_tag,
      &mut iter
    );

    // Deal with current_input_character reconsuming
    reconsume = should_reconsume;

    // Deal with any emitted tokens
    if emitted_tokens.is_some() {
      // get the most recent start tag token
      for token in emitted_tokens.as_ref().unwrap() {
        if let Token::StartTagToken(_) = token {
          recent_start_tag = Some(token.clone());
        }
      }

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
  current_token: &mut Option<Token>,
  temporary_buffer: &mut String,
  recent_start_tag: &Option<Token>,
  iter: &mut itertools::MultiPeek<std::str::Chars>
) -> (Option<Vec<Token>>, bool) {
  return match current_state {
    DataState::DataState => state_transitions::data_state_transition(c, current_state, return_state),
    DataState::RCDataState => state_transitions::rcdata_state_transition(c, current_state, return_state),
    DataState::RAWTEXTState => state_transitions::rawtext_state_transition(c, current_state),
    DataState::ScriptDataState => state_transitions::script_data_state_transition(c, current_state),
    DataState::PLAINTEXTState => state_transitions::plaintext_state_transition(c),
    DataState::TagOpenState => state_transitions::tag_open_state_transition(c, current_state, current_token),
    DataState::EndTagOpenState => state_transitions::end_tag_open_state_transition(c, current_state, current_token),
    DataState::TagNameState => state_transitions::tag_name_state_transition(c, current_state, current_token),
    DataState::RCDATALessThanSignState => state_transitions::rcdata_less_than_sign_state_transition(c, current_state, temporary_buffer),
    DataState::RCDATAEndTagOpenState => state_transitions::rcdata_end_tag_open_state_transition(c, current_state, current_token),
    DataState::RCDATAEndTagNameState => state_transitions::rcdata_end_tag_name_state_transition(c, current_state, current_token, temporary_buffer, recent_start_tag),
    DataState::RAWTEXTLessThanSignState => state_transitions::rawtext_less_than_sign_state_transition(c, current_state, temporary_buffer),
    DataState::RAWTEXTEndTagOpenState => state_transitions::rawtext_end_tag_open_state_transition(c, current_state, current_token),
    DataState::RAWTEXTEndTagNameState => state_transitions::rawtext_end_tag_name_state_transition(c, current_state, current_token, temporary_buffer, recent_start_tag),
    DataState::ScriptDataLessThanSignState => state_transitions::script_data_less_than_sign_state_transition(c, current_state, temporary_buffer),
    DataState::ScriptDataEndTagOpenState => state_transitions::script_data_end_tag_open_state_transition(c, current_state, current_token),
    DataState::ScriptDataEndTagNameState => state_transitions::script_data_end_tag_name_state_transition(c, current_state, current_token, temporary_buffer, recent_start_tag),
    DataState::ScriptDataEscapeStartState => state_transitions::script_data_escape_start_state_transition(c, current_state),
    DataState::ScriptDataEscapeStartDashState => state_transitions::script_data_escape_start_dash_state_transition(c, current_state),
    DataState::ScriptDataEscapedState => state_transitions::script_data_escaped_state_transition(c, current_state),
    DataState::ScriptDataEscapedDashState => state_transitions::script_data_escaped_dash_state_transition(c, current_state),
    DataState::ScriptDataEscapedDashDashState => state_transitions::script_data_escaped_dash_dash_state_transition(c, current_state),
    DataState::ScriptDataEscapedLessThanSignState => state_transitions::script_data_escaped_less_than_sign_state_transition(c, current_state, temporary_buffer),
    DataState::ScriptDataEscapedEndTagOpenState => state_transitions::script_data_escaped_end_tag_open_state_transition(c, current_state, current_token),
    DataState::ScriptDataEscapedEndTagNameState => state_transitions::script_data_escaped_end_tag_name_state_transition(c, current_state, current_token, temporary_buffer, recent_start_tag),
    DataState::ScriptDataDoubleEscapeStartState => state_transitions::script_data_double_escape_start_state_transition(c, current_state, temporary_buffer),
    DataState::ScriptDataDoubleEscapedState => state_transitions::script_data_double_escaped_state_transition(c, current_state),
    DataState::ScriptDataDoubleEscapedDashState => state_transitions::script_data_double_escaped_dash_state_transition(c, current_state),
    DataState::ScriptDataDoubleEscapedDashDashState => state_transitions::script_data_double_escaped_dash_dash_state_transition(c, current_state),
    DataState::ScriptDataDoubleEscapedLessThanSignState => state_transitions::script_data_double_escaped_less_than_sign_state_transition(c, current_state, temporary_buffer),
    DataState::ScriptDataDoubleEscapeEndState => state_transitions::script_data_double_escape_end_state_transition(c, current_state, temporary_buffer),
    DataState::BeforeAttributeNameState => state_transitions::before_attribute_name_state_transition(c, current_state, current_token),
    DataState::AttributeNameState => state_transitions::attribute_name_state_transition(c, current_state, current_token),
    DataState::AfterAttributeNameState => state_transitions::after_attribute_name_state_transition(c, current_state, current_token),
    DataState::BeforeAttributeValueState => state_transitions::before_attribute_value_state_transition(c, current_state, current_token),
    DataState::AttributeValueDoubleQuotedState => state_transitions::attribute_value_double_quoted_state_transition(c, current_state, return_state, current_token),
    DataState::AttributeValueSingleQuotedState => state_transitions::attribute_value_single_quoted_state_transition(c, current_state, return_state, current_token),
    DataState::AttributeValueUnquotedState => state_transitions::attribute_value_unquoted_state_transition(c, current_state, return_state, current_token),
    DataState::AfterAttributeValueQuotedState => state_transitions::after_attribute_value_quoted_state_transition(c, current_state, current_token),
    DataState::SelfClosingStartTagState => state_transitions::self_closing_start_tag_state_transition(c, current_state, current_token),
    DataState::BogusCommentState => state_transitions::bogus_comment_state_transition(c, current_state, current_token),
    DataState::MarkupDeclarationOpenState => state_transitions::markup_declaration_open_state_transition(c, current_state, current_token, iter),
    DataState::CommentStartState => state_transitions::comment_start_state_transition(c, current_state, current_token),
    DataState::CommentStartDashState => state_transitions::comment_start_dash_state_transition(c, current_state, current_token),
    DataState::CommentState => state_transitions::comment_state_transition(c, current_state, current_token),
    DataState::CommentLessThanSignState => state_transitions::comment_less_than_sign_state_transition(c, current_state, current_token),
    DataState::CommentLessThanSignBangState => state_transitions::comment_less_than_sign_bang_state_transition(c, current_state),
    DataState::CommentLessThanSignBangDashState => state_transitions::comment_less_than_sign_bang_dash_state_transition(c, current_state),
    DataState::CommentLessThanSignBangDashDashState => state_transitions::comment_less_than_sign_bang_dash_dash_state_transition(c, current_state),
    DataState::CommentEndDashState => state_transitions::comment_end_dash_state_transition(c, current_state, current_token),
    DataState::CommentEndState => state_transitions::comment_end_state_transition(c, current_state, current_token),
    DataState::CommentEndBangState => state_transitions::comment_end_bang_state_transition(c, current_state, current_token),
    DataState::DOCTYPEState => state_transitions::doctype_state_transition(c, current_state),
    DataState::BeforeDOCTYPENameState => state_transitions::before_doctype_name_state_transition(c, current_state, current_token),
    DataState::DOCTYPENameState => state_transitions::doctype_name_state_transition(c, current_state, current_token),
    DataState::AfterDOCTYPENameState => state_transitions::after_doctype_name_state_transition(c, current_state, current_token, iter),
    DataState::AfterDOCTYPEPublicKeywordState => state_transitions::after_doctype_public_keyword_state_transition(c, current_state, current_token),
    DataState::BeforeDOCTYPEPublicIdentifierState => state_transitions::before_doctype_public_identifier_state_transition(c, current_state, current_token),
    _ => (None, false),
  }
}
