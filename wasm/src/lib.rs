use wasm_bindgen::prelude::*;

use speedy_sourcemap::add as speedy_add;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
  speedy_add(a, b)
}
