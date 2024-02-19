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
pub struct Result {
  pub output: Vec<ParseResult>,
}

#[napi]
#[cfg(feature = "official")]
fn parse(config: Config) {
  let Config { input } = config;
  let iterator = input.iter();

  iterator
    .map(|opts| {
      let opts = opts.clone();
      parse_code(opts.clone())
    })
    .collect()

  // let _ = iterator.map(|opts| {
  //   println!("opts: {:?}", opts.clone());
  //   parse_code(opts.clone());
  // });
}
