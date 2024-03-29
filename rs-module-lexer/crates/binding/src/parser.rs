use std::path::PathBuf;

use swc_common::{
  comments::SingleThreadedComments, input::StringInput, sync::Lrc, FileName, Globals, SourceMap,
};
use swc_ecmascript::{
  parser::{lexer::Lexer, EsConfig, Parser, Syntax, TsConfig},
  visit::VisitMutWith,
};

use crate::{
  constants::{JS_EXITS, TS_EXITS},
  visitor::{ExportSpecifier, ImportExportVisitor, ImportSpecifier},
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
  pub exports: Vec<ExportSpecifier>,
  pub facade: bool,
}

pub fn parse_code(opts: ParseOptions) -> Result<ParseResult, anyhow::Error> {
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

  let source_map = Lrc::new(SourceMap::default());
  let source_file = source_map.new_source_file(
    FileName::Real(filename_path_buf.clone()),
    code.clone().into(),
  );
  let comment = SingleThreadedComments::default();
  let lexer = Lexer::new(
    syntax,
    Default::default(),
    StringInput::from(&*source_file),
    Some(&comment),
  );

  let mut parser = Parser::new_from(lexer);
  let module = parser.parse_module().expect("fail to parse module");

  swc_common::GLOBALS.set(&Globals::new(), || {
    let mut module = module;
    let mut visitor = ImportExportVisitor::new(code, source_map, source_file);

    module.visit_mut_with(&mut visitor);

    println!("start parse module");
    Ok(ParseResult {
      imports: visitor.imports,
      exports: visitor.exports,
      facade: visitor.facade,
      filename: filename,
    })
  })
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
