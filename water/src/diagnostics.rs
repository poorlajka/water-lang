pub mod emitter;

use logos::Span;
use std::fmt;

pub enum Severity {
    Error,
    Warning,
    Note,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Note => "Note",
        };
        write!(f, "{str}")
    }
}

pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub labels: Vec<Label>,
}

pub struct Label {
    pub span: Span,
    pub message: Option<String>,
}

pub enum LabelStyle {
    Primary,
    Secondary,
}