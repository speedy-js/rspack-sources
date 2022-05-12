#![deny(clippy::all)]

mod result;
mod sync;

mod cached_source;
mod concat_source;
mod helpers;
mod original_source;
mod raw_source;
mod source;
mod source_map_source;

pub use cached_source::CachedSource;
pub use concat_source::ConcatSource;
pub use original_source::OriginalSource;
pub use raw_source::RawSource;
pub use result::{Error, RspackSourcesError};
pub use source::{GenMapOption, Source};
pub use source_map_source::{SourceMapSource, SourceMapSourceOptions, SourceMapSourceSliceOptions};
