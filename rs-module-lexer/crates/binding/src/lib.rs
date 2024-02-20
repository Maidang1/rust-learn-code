#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod constants;
pub mod parser;
pub mod visitor;

use parser::{parse_code, ParseOptions, ParseResult};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[napi(object)]
pub struct Config {
  pub input: Vec<ParseOptions>,
}

#[napi(object)]
#[derive(Debug, Default)]
pub struct ResultOutput {
  pub output: Vec<ParseResult>,
}

#[napi]
#[cfg(feature = "official")]
fn parse(config: Config) -> Result<ResultOutput, anyhow::Error> {
  use anyhow::Ok;

  let Config { input } = config;
  let iterator = input.iter();

  let mut output = iterator
    .map(|opts| {
      let opts = opts.clone();
      parse_code(opts.clone())
    })
    .collect::<Result<Vec<ParseResult>, anyhow::Error>>()?;

  let result = ResultOutput { output };
  Ok(result)
}
