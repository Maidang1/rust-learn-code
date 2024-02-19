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
