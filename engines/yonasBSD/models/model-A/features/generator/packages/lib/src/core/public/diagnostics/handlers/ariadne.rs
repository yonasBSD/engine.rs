use ariadne::{Color, Label, Report, ReportKind, Source};
use miette::{Diagnostic, LabeledSpan, ReportHandler};
use std::fmt;

use crate::wrapped::WrappedDiagnostic;

pub fn install_ariadne_hook() {
    miette::set_hook(Box::new(|_| Box::new(AriadneHandler)))
        .expect("failed to install Ariadne handler");
}

#[derive(Debug)]
struct AriadneHandler;

impl ReportHandler for AriadneHandler {
    fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(diagnostic, f)
    }

    fn display(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        // FIX 1: Use the specialized miette::Diagnostic trait check
        // instead of error.downcast_ref::<dyn Diagnostic>()
        if let Some(diag) = error.as_diagnostic() {
            self.render(diag, f)
        } else {
            write!(f, "{}", error)
        }
    }
}

impl AriadneHandler {
    fn render(&self, diag: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_id = "engine";

        // FIX 2: ariadne::Report::build takes TWO arguments.
        // The second argument must be a Span. A tuple (Id, Range) implements Span.
        let mut builder =
            Report::build(ReportKind::Error, (file_id, 0..0)).with_message(diag.to_string());

        if let Some(labels) = diag.labels() {
            for label in labels {
                let start = label.offset();
                let end = start + label.len();
                builder = builder.with_label(
                    Label::new((file_id, start..end))
                        .with_message(label.label().unwrap_or("here"))
                        .with_color(Color::Red),
                );
            }
        }

        if let Some(src) = diag.source_code() {
            let span = LabeledSpan::new_with_span(None, (0, 0));

            if let Ok(contents) = src.read_span(span.inner(), 0, 0) {
                let mut out = Vec::new();
                let source_str = String::from_utf8_lossy(contents.data());

                builder
                    .finish()
                    .write((file_id, Source::from(source_str.as_ref())), &mut out)
                    .map_err(|_| fmt::Error)?;

                return write!(f, "{}", String::from_utf8_lossy(&out));
            }
        }

        write!(f, "{}", diag)
    }
}

// Helper trait to solve the E0277 downcast issue
trait AsDiagnostic {
    fn as_diagnostic(&self) -> Option<&dyn Diagnostic>;
}

impl AsDiagnostic for dyn std::error::Error + 'static {
    fn as_diagnostic(&self) -> Option<&dyn Diagnostic> {
        if let Some(wrapped) = self.downcast_ref::<WrappedDiagnostic>() {
            return Some(&*wrapped.0);
        }
        None
    }
}
