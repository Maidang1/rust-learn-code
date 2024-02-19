use std::path::PathBuf;

use swc_ecmascript::parser::{EsConfig, Syntax, TsConfig};

use crate::{
  constants::{JS_EXITS, TS_EXITS},
  visitor::{ExportSpecifier, ImportSpecifier},
};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ParseOptions {
  pub filename: String,
  pub code: String,
}

#[napi(object)]
#[derive(Debug)]
pub struct ParseResult {
  pub filename: String,
  pub imports: Vec<ImportSpecifier>,
  pub exports: Vec<ImportSpecifier>,
  pub facade: bool,
}

pub fn parse_code(opts: ParseOptions) {
  println!("parse_code start");
  let ParseOptions { filename, code } = opts;
  let file_info = parse_filename(&filename);
  let FileInfo {
    is_jsx,
    is_typescript,
    filename_path_buf,
    ..
  } = file_info;

  let syntax = if is_typescript {
    Syntax::Typescript(TsConfig {
      tsx: is_jsx,
      decorators: true,
      ..Default::default()
    })
  } else {
    Syntax::Es(EsConfig {
      jsx: is_jsx,
      export_default_from: true,
      ..Default::default()
    })
  };
  println!("parsing: {:?}", syntax);
}

#[derive(Debug)]
pub struct FileInfo {
  pub extension: String,
  pub filename: String,
  pub filename_path_buf: PathBuf,
  pub is_jsx: bool,
  pub is_typescript: bool,
}

fn parse_filename(filepath: &String) -> FileInfo {
  let filename = filepath.split("/").last().unwrap();
  let extension = filename.split(".").last().unwrap();
  let is_typescript = TS_EXITS.contains(&extension);
  let is_jsx = JS_EXITS.contains(&extension);
  let filename_path_buf = PathBuf::from(filename);
  FileInfo {
    extension: extension.to_string(),
    filename: filename.to_string(),
    filename_path_buf,
    is_jsx,
    is_typescript,
  }
}
