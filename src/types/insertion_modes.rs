use std::fmt::{
  Display,
  Formatter,
  Result
};

#[derive(Debug)]
pub enum InsertionMode {
  Initial,
  BeforeHtml,
  BeforeHead,
  InHead,
  InHeadNoScript,
  AfterHead,
  InBody,
  Text,
  InTable,
  InTableText,
  InCaption,
  InColumnGroup,
  InTableBody,
  InRow,
  InCell,
  InSelect,
  InSelectInTable,
  InTemplate,
  AfterBody,
  InFrameSet,
  AfterFrameSet,
  AfterAfterBody,
  AfterAfterFrameset,
}

impl Default for InsertionMode {
  fn default() -> Self {
    return InsertionMode::Initial;
  }
}

impl Display for InsertionMode {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}
