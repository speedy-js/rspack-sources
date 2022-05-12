use smol_str::SmolStr;
use sourcemap::{SourceMap, SourceMapBuilder, Token};

use crate::cached_source::CachedSource;
use crate::result::Error;
use crate::source::{GenMapOption, Source};
use crate::sync::Lrc;

pub struct SourceMapSource {
  pub(crate) source_code: SmolStr,
  pub(crate) name: SmolStr,
  pub(crate) source_map: Lrc<SourceMap>,
  pub(crate) original_source: Option<SmolStr>,
  pub(crate) inner_source_map: Option<Lrc<SourceMap>>,
  pub(crate) remove_original_source: bool,

  pub(crate) sourcemap_remapped: Option<Lrc<SourceMap>>,
}

#[cfg(feature = "concurrent")]
unsafe impl Sync for SourceMapSource {}
#[cfg(feature = "concurrent")]
unsafe impl Send for SourceMapSource {}

pub struct SourceMapSourceSliceOptions<'a> {
  pub source_code: &'a [u8],
  pub name: String,
  pub source_map: SourceMap,
  pub original_source: Option<&'a [u8]>,
  pub inner_source_map: Option<SourceMap>,
  pub remove_original_source: bool,
}

pub struct SourceMapSourceOptions {
  pub source_code: String,
  pub name: String,
  pub source_map: SourceMap,
  pub original_source: Option<String>,
  pub inner_source_map: Option<SourceMap>,
  pub remove_original_source: bool,
}

impl SourceMapSource {
  pub fn new(options: SourceMapSourceOptions) -> Self {
    let SourceMapSourceOptions {
      source_code,
      name,
      source_map,
      original_source,
      inner_source_map,
      remove_original_source,
    } = options;

    let original_source: Option<SmolStr> = original_source.map(Into::into);
    let source_map = Self::ensure_source_map(source_map, name.as_str(), original_source.clone());

    Self {
      source_code: source_code.into(),
      name: name.into(),
      source_map: Lrc::new(source_map),
      original_source,
      inner_source_map: inner_source_map.map(Lrc::new),
      remove_original_source,
      sourcemap_remapped: Default::default(),
    }
  }

  pub fn from_slice(options: SourceMapSourceSliceOptions) -> Result<Self, Error> {
    let SourceMapSourceSliceOptions {
      source_code,
      name,
      source_map,
      original_source,
      inner_source_map,
      remove_original_source,
    } = options;

    let original_source = if let Some(original_source) = original_source {
      Some(String::from_utf8(original_source.to_vec())?)
    } else {
      None
    };

    let original_source: Option<SmolStr> = original_source.map(Into::into);
    let source_map = Self::ensure_source_map(source_map, name.as_str(), original_source.clone());

    Ok(Self {
      source_code: String::from_utf8(source_code.to_vec())?.into(),
      name: name.into(),
      source_map: Lrc::new(source_map),
      original_source,
      inner_source_map: inner_source_map.map(Lrc::new),
      remove_original_source,

      sourcemap_remapped: Default::default(),
    })
  }

  pub fn ensure_source_map(
    mut source_map: SourceMap,
    name: &str,
    original_source: Option<SmolStr>,
  ) -> SourceMap {
    let current_file_name = name;
    let source_idx = source_map.sources().enumerate().find_map(|(idx, source)| {
      if source == current_file_name {
        Some(idx)
      } else {
        None
      }
    });

    if let Some(source_idx) = source_idx {
      if source_map.get_source(source_idx as u32).is_none() {
        source_map.set_source_contents(
          source_idx as u32,
          original_source.as_ref().map(|s| s.as_str()),
        );
      }
    }

    source_map
  }

  fn find_original_token<'a, 'b>(&'a self, token: &'b Token<'a>) -> (Token<'a>, Option<&str>) {
    let load_source_contents = || self.source_map.get_source_contents(token.get_src_id());

    if let Some(inner_source_map) = &self.inner_source_map {
      let source = token.get_source();
      let src_line = token.get_src_line();
      let src_col = token.get_src_col();

      if inner_source_map.get_file() == source {
        if let Some(original_token) = inner_source_map.lookup_token(src_line, src_col) {
          (
            original_token,
            inner_source_map.get_source_contents(original_token.get_src_id()),
          )
        } else {
          (*token, load_source_contents())
        }
      } else {
        (*token, load_source_contents())
      }
    } else {
      (*token, load_source_contents())
    }
  }

  pub(crate) fn remap_with_inner_sourcemap(
    &mut self,
    gen_map_option: &GenMapOption,
  ) -> Option<SourceMap> {
    let mut source_map_builder = SourceMapBuilder::new(Some(&self.name));

    if self.inner_source_map.is_some() {
      let source_map = &self.source_map;
      source_map.tokens().for_each(|token| {
        let (original_token, source_content) = self.find_original_token(&token);

        let raw_token = source_map_builder.add(
          token.get_dst_line(),
          token.get_dst_col(),
          original_token.get_src_line(),
          original_token.get_src_col(),
          original_token.get_source(),
          original_token.get_name(),
        );

        if gen_map_option.include_source_contents && !self.remove_original_source {
          source_map_builder.set_source_contents(raw_token.src_id, source_content);
        }
      });

      return Some(source_map_builder.into_sourcemap());
    }

    None
  }
}

impl Source for SourceMapSource {
  fn source(&mut self) -> SmolStr {
    self.source_code.clone()
  }

  fn map(&mut self, option: &GenMapOption) -> Option<Lrc<SourceMap>> {
    let remapped = self.remap_with_inner_sourcemap(option);
    self.sourcemap_remapped = remapped.map(Lrc::new);

    Some(
      self
        .sourcemap_remapped
        .as_ref()
        .unwrap_or(&self.source_map)
        .clone(),
    )
  }
}

impl From<SourceMapSource> for CachedSource<SourceMapSource> {
  fn from(source_map: SourceMapSource) -> Self {
    Self::new(source_map)
  }
}

#[test]
fn test_source_map_source() {
  let base_fixure = ::std::path::PathBuf::from("tests/fixtures/transpile-minify/files/helloworld");

  let mut original_map_path = base_fixure.clone();
  original_map_path.set_extension("js.map");
  let mut transformed_map_path = base_fixure.clone();
  transformed_map_path.set_extension("min.js.map");

  let mut original_code_path = base_fixure.clone();
  original_code_path.set_extension("js");
  let mut transformed_code_path = base_fixure.clone();
  transformed_code_path.set_extension("min.js");

  let original_map_buf = ::std::fs::read(original_map_path).expect("unable to find test fixture");
  let transformed_map_buf =
    ::std::fs::read(transformed_map_path).expect("unable to find test fixture");
  let original_code_buf = ::std::fs::read(original_code_path).expect("unable to find test fixture");
  let transformed_code_buf =
    ::std::fs::read(transformed_code_path).expect("unable to find test fixture");

  let mut source_map_source = SourceMapSource::from_slice(SourceMapSourceSliceOptions {
    source_code: &transformed_code_buf,
    name: "helloworld.min.js".into(),
    source_map: sourcemap::SourceMap::from_slice(&transformed_map_buf).unwrap(),
    original_source: Some(&original_code_buf),
    inner_source_map: Some(sourcemap::SourceMap::from_slice(&original_map_buf).unwrap()),
    remove_original_source: false,
  })
  .expect("failed");

  let new_source_map = source_map_source.map(&Default::default()).expect("failed");
  let token = new_source_map.lookup_token(15, 47).expect("failed");

  assert_eq!(token.get_source(), Some("helloworld.mjs"));
  assert_eq!(token.get_src_col(), 20);
  assert_eq!(token.get_src_line(), 18);
  assert_eq!(token.get_name(), Some("alert"));
}
