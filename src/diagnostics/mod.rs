/*
Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>


*/

//! Contains JS errors, warnings and related structures

enum DiagnosticKind {
  Error,
  Warning,
}

struct Diagnostic {
  kind: DiagnosticKind,
  message: String,
  span: Span,
}

struct Span {
  start: usize,
  end: usize,
}
