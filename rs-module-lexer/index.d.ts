/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ParseOptions {
  filename: string
  code: string
}
export interface ParseResult {
  filename: string
  imports: Array<ImportSpecifier>
  exports: Array<ImportSpecifier>
  facade: boolean
}
export interface ImportSpecifier {
  /** source name */
  n?: string
  /** source start index */
  s: number
  /** source end index */
  e: number
  /** import start index */
  ss: number
  /** import end index */
  se: number
  /** dynamic import start index */
  d: number
  /** assert object start index (include `{}`) */
  a: number
}
export interface ExportSpecifier {
  /** export name */
  n: string
  /** export origin name */
  ln?: string
  /** export name start index */
  s: number
  /** export name end index */
  e: number
  /** export origin name start index */
  ls: number
  /** export origin name end index */
  le: number
}
export interface Config {
  input: Array<ParseOptions>
}
export interface Result {
  output: Array<ParseResult>
}
export function parse(config: Config): void
