use smol_str::SmolStr;
use std::collections::HashMap;

use sourcemap::SourceMap;

use crate::source::GenMapOption;
use crate::sync::Lrc;
use crate::Source;

pub struct CachedSource<T: Source> {
  inner: T,
  cached_map: HashMap<GenMapOption, Option<Lrc<SourceMap>>>,
  cached_code: Option<SmolStr>,
}

#[cfg(feature = "concurrent")]
unsafe impl<T: Source + Sync> Sync for CachedSource<T> {}
#[cfg(feature = "concurrent")]
unsafe impl<T: Source + Send> Send for CachedSource<T> {}

impl<T: Source> CachedSource<T> {
  pub fn new(source: T) -> Self {
    Self {
      inner: source,
      cached_code: Default::default(),
      cached_map: Default::default(),
    }
  }

  pub fn into_inner(self) -> T {
    self.inner
  }
}

impl<T: Source> Source for CachedSource<T> {
  fn map(&mut self, gen_map_option: &GenMapOption) -> Option<Lrc<SourceMap>> {
    use std::collections::hash_map::Entry;
    if let Some(source_map) = self.cached_map.get(gen_map_option) {
      source_map.clone()
    } else {
      let map = self.inner.map(gen_map_option);
      self.cached_map.insert(gen_map_option.clone(), map.clone());
      map
    }
  }

  fn source(&mut self) -> SmolStr {
    if let Some(cached_code) = &self.cached_code {
      return cached_code.clone();
    }
    let code = self.inner.source();
    self.cached_code = Some(code.clone());
    code
  }
}
