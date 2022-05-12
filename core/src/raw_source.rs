use crate::sync::Lrc;
use smol_str::SmolStr;
use sourcemap::SourceMap;

use crate::{Error, GenMapOption, Source};

pub struct RawSource {
  source_code: SmolStr,
}

impl RawSource {
  pub fn new(source_code: &str) -> Self {
    Self {
      source_code: source_code.into(),
    }
  }

  pub fn from_slice(source_code: &[u8]) -> Result<Self, Error> {
    Ok(Self {
      source_code: String::from_utf8(source_code.to_vec())?.into(),
    })
  }
}

impl Source for RawSource {
  fn map(&mut self, _option: &GenMapOption) -> Option<Lrc<SourceMap>> {
    None
  }

  fn source(&mut self) -> SmolStr {
    self.source_code.clone()
  }
}
