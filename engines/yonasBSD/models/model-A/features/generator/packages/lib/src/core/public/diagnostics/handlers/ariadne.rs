use ariadne::{Color, Label, Report, ReportKind, Source};
use miette::{Diagnostic, ReportHandler};
use std::fmt;

use crate::EngineError;
use crate::wrapped::WrappedDiagnostic;

pub fn install_ariadne_hook() {
    miette::set_hook(Box::new(|_| Box::new(AriadneHandler)))
        .expect("failed to install Ariadne handler");
}

#[derive(Debug)]
pub struct AriadneHandler;

impl ReportHandler for AriadneHandler {
    fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_from_diag(diagnostic, f)
    }

    fn display(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if let Some(engine) = error.downcast_ref::<EngineError>() {
            return self.render_from_engine(engine, f);
        }

        if let Some(wrapped) = error.downcast_ref::<WrappedDiagnostic>() {
            return self.render_from_diag(&*wrapped.0, f);
        }

        write!(f, "{}", error)
    }
}

impl AriadneHandler {
    fn render_from_engine(&self, engine: &EngineError, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_id = "engine";
        let code = engine.code();
        let msg = format!("error[{code}]: {engine}");

        let mut builder = Report::build(ReportKind::Error, (file_id, 0..0)).with_message(msg);

        match engine {
            EngineError::InvalidPath {
                spans,
                suggestion,
                ..
            } => {
                let mut spans = spans.clone();
                spans.sort_by_key(|s| s.offset());

                for (i, span) in spans.iter().enumerate() {
                    let start = span.offset();
                    let end = start + span.len();

                    builder = builder.with_label(
                        Label::new((file_id, start..end))
                            .with_message("empty segment here")
                            .with_color(Color::Red)
                            .with_order(i as i32),
                    );
                }

                builder =
                    builder.with_note("empty segments occur when two dots appear consecutively");
                builder = builder.with_note("module paths must not contain empty identifiers");

                if let Some(s) = suggestion {
                    builder = builder.with_help(format!("did you mean `{}`?", s));
                }

                builder = builder.with_help("try removing extra dots");
                builder = builder.with_help(format!(
                    "for more information about this error, run: engine --explain {code}",
                ));
            }

            EngineError::Scaffolder(err) => {
                // Forward the scaffolder diagnostic to the generic diagnostic renderer
                return self.render_from_diag(err, f);
            }
        }

        let full = match engine {
            EngineError::InvalidPath {
                full, ..
            } => full.0.as_str(),
            _ => "<no source>",
        };

        let mut out = Vec::new();

        builder
            .finish()
            .write((file_id, Source::from(full)), &mut out)
            .map_err(|_| fmt::Error)?;

        write!(f, "{}", String::from_utf8_lossy(&out))
    }

    fn render_from_diag(&self, diag: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_id = "engine";
        let mut builder =
            Report::build(ReportKind::Error, (file_id, 0..0)).with_message(diag.to_string());

        let labels: Vec<_> = diag.labels().map(|l| l.collect()).unwrap_or_default();

        for label in &labels {
            let start = label.offset();
            let end = start + label.len();

            builder = builder.with_label(
                Label::new((file_id, start..end))
                    .with_message(label.label().unwrap_or("here"))
                    .with_color(Color::Red),
            );
        }

        if let Some(src) = diag.source_code() {
            if let Some(label) = labels.first() {
                let start = label.offset();
                let len = label.len();
                let span = miette::SourceSpan::new(start.into(), len.into());

                if let Ok(contents) = src.read_span(&span, 0, 0) {
                    let text = std::str::from_utf8(contents.data()).unwrap_or("<invalid utf8>");

                    let mut out = Vec::new();

                    builder
                        .finish()
                        .write((file_id, Source::from(text)), &mut out)
                        .map_err(|_| fmt::Error)?;

                    return write!(f, "{}", String::from_utf8_lossy(&out));
                }
            }
        }

        write!(f, "{}", diag)
    }
}
