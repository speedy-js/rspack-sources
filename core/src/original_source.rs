use crate::sync::Lrc;

use smol_str::SmolStr;
use sourcemap::{SourceMap, SourceMapBuilder};

use crate::{GenMapOption, Source};

pub struct OriginalSource {
  source_code: SmolStr,
  name: SmolStr,
}

impl OriginalSource {
  pub fn new(source_code: &str, name: &str) -> Self {
    Self {
      source_code: source_code.into(),
      name: name.into(),
    }
  }
}

impl Source for OriginalSource {
  fn map(&mut self, option: &GenMapOption) -> Option<Lrc<SourceMap>> {
    let columns = option.columns;

    let mut sm_builder = SourceMapBuilder::new(None);
    let source_id = sm_builder.add_source(&self.name);
    if option.include_source_contents {
      sm_builder.set_source_contents(source_id, Some(&self.source_code));
    }

    if columns {
      let mut line = 0;
      let mut col = 0;
      self.source_code.chars().into_iter().for_each(|c| {
        if col == 0 {
          sm_builder.add(line, 0, line, 0, Some(self.name.as_str()), None);
        }

        match c {
          '\n' => {
            line += 1;
            col = 0;
          }
          ';' | '}' => {
            col += 1;
            sm_builder.add(line, col, line, col, Some(self.name.as_str()), None);
          }
          '{' => {
            sm_builder.add(line, col, line, col, Some(self.name.as_str()), None);
            col += 1;
          }
          _ => {
            col += 1;
          }
        }
      });
    } else {
      let line = self.source_code.split('\n').count();

      for index in 0..line {
        sm_builder.add(
          index as u32,
          0,
          index as u32,
          0,
          Some(self.name.as_str()),
          None,
        );
      }
    }

    Some(Lrc::new(sm_builder.into_sourcemap()))
  }

  fn source(&mut self) -> SmolStr {
    self.source_code.clone()
  }
}

#[test]
fn original_source() {
  let mut original_source = OriginalSource::new(
    r#"import { createElement } from "react";
import { render } from "react-dom";
const div = createElement("div", null, {});
render(div, document.getElementById("app"));"#,
    "app.js",
  );

  let source_map = original_source
    .map(&GenMapOption {
      columns: true,
      ..Default::default()
    })
    .expect("should generate");

  let mut writer = vec![] as Vec<u8>;
  source_map.to_writer(&mut writer);
  println!("{}", String::from_utf8(writer).unwrap());
  println!("{}", original_source.source());
}
