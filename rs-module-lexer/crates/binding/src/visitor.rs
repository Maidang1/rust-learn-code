use swc_common::sync::Lrc;
use swc_common::{SourceFile, SourceMap, Span};
use swc_ecmascript::ast;
use swc_ecmascript::visit::{VisitAllWith, VisitMut, VisitMutWith};

#[derive(Debug)]
#[napi(object)]
pub struct ImportSpecifier {
  /// source name
  pub n: Option<String>,
  /// source start index
  pub s: i32,
  /// source end index
  pub e: i32,
  /// import start index
  pub ss: i32,
  /// import end index
  pub se: i32,
  /// dynamic import start index
  pub d: i32,
  /// assert object start index (include `{}`)
  pub a: i32,
}

#[derive(Debug)]
#[napi(object)]
pub struct ExportSpecifier {
  /// export name
  pub n: String,
  /// export origin name
  pub ln: Option<String>,
  /// export name start index
  pub s: i32,
  /// export name end index
  pub e: i32,
  /// export origin name start index
  pub ls: i32,
  /// export origin name end index
  pub le: i32,
}

pub struct ImportExportVisitor {
  pub imports: Vec<ImportSpecifier>,
  pub exports: Vec<ExportSpecifier>,
  pub facade: bool,

  code_utf16: Vec<u16>,
  source_map: Lrc<SourceMap>,
  source_file: Lrc<SourceFile>,
}

impl ImportExportVisitor {
  pub fn new(code: String, source_map: Lrc<SourceMap>, source_file: Lrc<SourceFile>) -> Self {
    let code_utf16: Vec<_> = code.encode_utf16().collect();

    Self {
      imports: vec![],
      exports: vec![],
      facade: false,
      code_utf16,
      source_map,
      source_file,
    }
  }
}

impl ImportExportVisitor {
  fn parse_import(&mut self, import: &mut ast::ImportDecl) {
    if import.type_only {
      return;
    }
    // import 'b';
    if import.specifiers.is_empty() {
      let name = import.src.value.to_string();
      let import_span = self.get_real_span(import.span);
      let src_span = self.get_real_span_without_quotes(import.span);
    }
  }
}

// utils

impl ImportExportVisitor {
  fn get_real_span(&self, span: Span) -> (i32, i32) {
    let real_span = self.source_map.span_to_char_offset(&self.source_file, span);
    (real_span.0 as i32, real_span.1 as i32)
  }
  fn get_real_span_without_quotes(&self, span: Span) -> (i32, i32) {
    let real_span = self.get_real_span(span);
    (real_span.0 + 1, real_span.1 - 1)
  }
}

impl VisitMut for ImportExportVisitor {
  fn visit_mut_module(&mut self, module: &mut swc_ecmascript::ast::Module) {
    let is_facade = module.body.iter().all(|item| {
      if let swc_ecmascript::ast::ModuleItem::ModuleDecl(_) = &item {
        true
      } else {
        false
      }
    });
    self.facade = is_facade;
    module.visit_mut_children_with(self);
  }

  fn visit_mut_module_decl(&mut self, decl: &mut swc_ecmascript::ast::ModuleDecl) {
    match decl {
      ast::ModuleDecl::Import(import) => self.parse_import(import),
      _ => {}
    }
  }
}
