pub enum DataState {
  DataState(),
  RCDataState(),
  RAWTEXTSState(),
  ScriptDataState(),
  PLAINTEXTState(),
  TagOpenState(),
  EndTagOpenState(),
  TagNameState(),
  RCDATALessThanSignState(),
  RCDATAEndTagOpenState(),
  RCDATAEndTagNameState(),
  RAWTEXTLessThanSignState(),
  RAWTEXTEndTagOpenState(),
  RAWTEXTEndTagNameState(),
  ScriptDataLessThanState(),
  ScriptDataEndTagOpenState(),
  ScriptDataEndTagNameState(),
  ScriptDataEscapeStartDate(),
  ScriptDataEscapeStartDashState(),
  ScriptDataEscapedState(),
  ScriptDataEscapedDashState(),
  ScriptDataEscapedDashDashState(),
  ScriptDataEscapedLessThanSignState(),
  ScriptDataEscapedEndTagOpenState(),
  ScriptDataEscapedEndTagNameState(),
  ScriptDataDoubleEscapeStartState(),
  ScriptDataDoubleEscapedState(),
  ScriptDataDDoubleEscapedDashState(),
  ScriptDataDoubleEscapedDashState(),
  ScriptDataDoubleEscapedLessThanSignState(),
  ScriptDataDoubleEscapeEndState(),
  BeforeAttributeNameState(),
  AttributeNameState(),
  AfterAttributeNameState(),
  BeforeAttributeValueState(),
  AttributeValueDoubleQuotedState(),
  AttributeValueSingleQuotedState(),
  AttributeValueUnquotedState(),
  AfterAttributeQuotedState(),
  SelfClosingStartTagState(),
  BogusCommentState(),
  MarkupDeclarationOpenState(),
  CommentStartState(),
  CommentStartDashState(),
  CommentState(),
  CommentLessThanSignState(),
  CommentLessThanSignBangState(),
  CommentLessThanSignBangDashState(),
  CommentLessThanSignBangDashDashState(),
  CommentEndDashState(),
  CommentEndState(),
  CommentEndBangState(),
  DOCTYPEState(),
  BeforeDOCTYPENameState(),
  DOCTYPENameState(),
  AfterDOCTYPENameState(),
  AfterDOCTYPEPublicKeywordState(),
  BeforeDOCTYPEPublicIdentifierState(),
  DOCTYPEPublicIdentifierDoubleQuotedState(),
  DOCTYPEPublicIdentifierSingleQuotedState(),
  AfterDOCTYPEPublicIdentifierState(),
  BetweenDOCTYPEPublicAndSystemIdentifiersState(),
  AfterDOCTYPESystemKeywordState(),
  BeforeDOCTYPESystemIdentifierState(),
  DOCTYPESystemIdentifierDoubleQuotedState(),
  DOCTYPESystemIdentifierSingleQuotedState(),
  AfterDOCTYPESystemIdentifierState(),
  BogusDOCTYPEState(),
  CDATASectionState(),
  CDATASectionBracketState(),
  CDATASectionEndState(),
  CharacterReferenceState(),
  NamedCharacterReferenceState(),
  AmbiguousAmpersandState(),
  NumericCharacterReferenceState(),
  HexadecimalCharacterReferenceStartState(),
  DecimalCharacterReferenceStartState(),
  HexidecimalCharacterReferenceState(),
  DecimalCharacterReferenceState(),
  NumericCharacterReferenceEndState(),
}

impl Default for DataState {
  fn default() -> Self {
    return DataState::DataState();
  }
}
