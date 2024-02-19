#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod constants;
pub mod parser;
pub mod visitor;

use parser::{ParseOptions, ParseResult};
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
  use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

  let Config { input } = config;
  #[cfg(feature = "parallel")]
  let iterator = input.par_iter();

  let output = iterator.map(|opts| {
    let current_opts = opts.clone();
    println!("opts: {:?}", current_opts);
  });

  println!("output: {:?}", output);
}
