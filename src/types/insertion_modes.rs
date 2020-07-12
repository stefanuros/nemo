pub enum InsertionMode {
  Initial(),
  BeforeHtml(),
  BeforeHead(),
  InHead(),
  InHeadNoScript(),
  AfterHead(),
  InBody(),
  Text(),
  InTable(),
  InTableText(),
  InCaption(),
  InColumnGroup(),
  InTableBody(),
  InRow(),
  InCell(),
  InSelect(),
  InSelectInTable(),
  InTemplate(),
  AfterBody(),
  InFrameSet(),
  AfterFrameSet(),
  AfterAfterBody(),
  AfterAfterFrameset(),
}

impl Default for InsertionMode {
  fn default() -> Self {
    return InsertionMode::Initial();
  }
}
