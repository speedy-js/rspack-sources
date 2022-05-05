use napi::bindgen_prelude::*;
use napi_derive::napi;

use speedy_sourcemap::add as speedy_add;

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi]
fn add(a: u32, b: u32) -> u32 {
  speedy_add(a, b)
}
