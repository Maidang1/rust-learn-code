use crate::visitor::{ExportSpecifier, ImportSpecifier};

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
