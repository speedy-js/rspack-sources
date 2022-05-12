use crate::sync::Lrc;
use smol_str::SmolStr;
use sourcemap::SourceMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GenMapOption {
  /// If set to false the implementation may omit mappings for columns. (default: true)
  pub columns: bool,
  pub include_source_contents: bool,
  pub file: Option<String>,
}

impl Default for GenMapOption {
  fn default() -> Self {
    Self {
      columns: true,
      include_source_contents: true,
      file: Default::default(),
    }
  }
}

pub trait Source {
  fn map(&mut self, option: &GenMapOption) -> Option<Lrc<SourceMap>>;

  fn source(&mut self) -> SmolStr;
}
