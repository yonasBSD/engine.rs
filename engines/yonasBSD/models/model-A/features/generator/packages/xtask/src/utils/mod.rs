mod errors;

use errors::*;
use miette::{IntoDiagnostic, NamedSource, Result, SourceSpan};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Attribute, ItemEnum, Lit};
use walkdir::WalkDir;

//
// ────────────────────────────────────────────────────────────────
//  Extract error variants (Miette-native)
// ────────────────────────────────────────────────────────────────
//

/// Extract all documented error variants from an `EngineError` enum.
///
/// This function inspects each variant for a `#[diagnostic(code = "...")]`
/// attribute, collects its doc comments, and returns a list of `ErrorDoc`
/// entries. If a variant has a diagnostic code but no documentation, a
/// `MissingDocsError` diagnostic is returned.
///
/// The `order` field in `ErrorDoc` preserves the original enum ordering so
/// that we can later validate that it matches the numeric code order.
pub fn extract_error_variants(e: &ItemEnum, file: &Path) -> Result<Vec<ErrorDoc>> {
    let mut out = Vec::new();
    let file_text = fs::read_to_string(file).into_diagnostic()?;

    for (idx, variant) in e.variants.iter().enumerate() {
        let code = find_diagnostic_code(&variant.attrs)?;
        if let Some(code) = code {
            let docs = collect_doc_comments(&variant.attrs);

            // Enforce that every variant with a diagnostic code has at least
            // one doc comment. We surface a span-based diagnostic pointing
            // directly at the variant identifier.
            if docs.trim().is_empty() {
                let span = variant.ident.span();
                let start = span.start();
                let end = span.end();

                let offset = linecol_to_offset(&file_text, start.line, start.column);
                let length = linecol_to_offset(&file_text, end.line, end.column) - offset;

                let src = NamedSource::new(file.display().to_string(), file_text.clone());

                return Err(MissingDocsError {
                    code,
                    name: variant.ident.to_string(),
                    src,
                    span: SourceSpan::new(offset.into(), length.into()),
                }
                .into());
            }

            out.push(ErrorDoc {
                code,
                name: variant.ident.to_string(),
                docs,
                file: file.to_path_buf(),
                order: idx,
            });
        }
    }

    Ok(out)
}

//
// ────────────────────────────────────────────────────────────────
//  Helper: convert (line, col) → byte offset
// ────────────────────────────────────────────────────────────────
//

/// Convert a 1-based `(line, column)` pair into a byte offset within `text`.
///
/// This helper is used to translate span information from `syn` into the
/// `SourceSpan` offsets expected by Miette diagnostics. It assumes that the
/// input text uses `\n` line endings and that `line` is 1-based.
fn linecol_to_offset(text: &str, line: usize, col: usize) -> usize {
    let mut offset = 0;
    for (i, l) in text.lines().enumerate() {
        if i + 1 == line {
            return offset + col;
        }
        offset += l.len() + 1;
    }
    offset
}

//
// ────────────────────────────────────────────────────────────────
//  Error Index Generation
// ────────────────────────────────────────────────────────────────
//

/// Generate the Engine.rs error index Markdown document.
///
/// This function:
/// 1. Locates the `engine-rs-lib` crate in the workspace.
/// 2. Walks its `src` tree to find all `EngineError` enums.
/// 3. Extracts and validates all error variants.
/// 4. Renders a Markdown index and writes it to `engine-rs/assets/error-index.md`.
///
/// Any structural issues (missing docs, missing sidecars, non-sequential codes,
/// or enum ordering mismatches) are surfaced as rich Miette diagnostics.
pub fn generate_error_index() -> Result<()> {
    let lib_root = find_crate_root("engine-rs-lib")?;
    let lib_src = lib_root.join("src");

    let mut errors = Vec::new();

    // Walk the library source tree and parse every Rust file, looking for
    // `EngineError` enums. This keeps the tooling resilient to file layout
    // changes as long as the enum name remains stable.
    for entry in WalkDir::new(&lib_src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path).into_diagnostic()?;
        let file = syn::parse_file(&content).into_diagnostic()?;

        for item in file.items {
            if let syn::Item::Enum(e) = item {
                if e.ident == "EngineError" {
                    errors.extend(extract_error_variants(&e, path)?);
                }
            }
        }
    }

    // Enforce global invariants across all discovered errors.
    validate_errors(&errors)?;

    // Render a stable, sorted Markdown index.
    let index = render_markdown(&errors);

    let cli_root = find_crate_root("engine-rs")?;
    let assets_dir = cli_root.join("assets");
    fs::create_dir_all(&assets_dir).into_diagnostic()?;

    let out_path = assets_dir.join("error-index.md");
    fs::write(&out_path, index).into_diagnostic()?;

    println!("Generated {}", out_path.display());
    Ok(())
}

//
// ────────────────────────────────────────────────────────────────
//  ErrorDoc
// ────────────────────────────────────────────────────────────────
//

/// In-memory representation of a single error variant.
///
/// This struct captures both the semantic information (code, name, docs) and
/// structural metadata (source file, enum order) needed for validation and
/// index generation.
#[derive(Debug, Clone)]
pub(crate) struct ErrorDoc {
    /// The diagnostic error code (e.g. `E0001`).
    pub code: String,

    /// The Rust enum variant name (e.g. `IoError`).
    pub name: String,

    /// The collected doc comments for this variant.
    pub docs: String,

    /// The file where this variant is defined.
    pub file: PathBuf,

    /// The zero-based position of this variant within the enum.
    pub order: usize,
}

//
// ────────────────────────────────────────────────────────────────
//  Extract #[diagnostic(code = "...")]
// ────────────────────────────────────────────────────────────────
//

/// Find the `code = "..."` value inside a `#[diagnostic(...)]` attribute.
///
/// Returns `Ok(Some(code))` if a diagnostic code is present, `Ok(None)` if the
/// attribute is absent, and a diagnostic error if the attribute cannot be
/// parsed.
pub fn find_diagnostic_code(attrs: &[Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if attr.path().is_ident("diagnostic") {
            let mut found: Option<String> = None;

            // Use `parse_nested_meta` to walk the attribute arguments and
            // extract the `code = "..."` value if present.
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("code") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        found = Some(s.value());
                    }
                }
                Ok(())
            })
            .into_diagnostic()?;

            if found.is_some() {
                return Ok(found);
            }
        }
    }
    Ok(None)
}

//
// ────────────────────────────────────────────────────────────────
//  Collect doc comments
// ────────────────────────────────────────────────────────────────
//

/// Collect all `///` doc comments from a list of attributes into a single string.
///
/// Each doc line is concatenated with a trailing newline to preserve the
/// original paragraph structure as closely as possible.
pub fn collect_doc_comments(attrs: &[Attribute]) -> String {
    let mut docs = String::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Ok(meta) = attr.meta.require_name_value() {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(s) = &expr_lit.lit {
                        docs.push_str(&s.value());
                        docs.push('\n');
                    }
                }
            }
        }
    }

    docs
}

//
// ────────────────────────────────────────────────────────────────
//  Render Markdown
// ────────────────────────────────────────────────────────────────
//

/// Render a Markdown error index from a list of `ErrorDoc` entries.
///
/// The output is sorted by error code to provide a stable, predictable
/// document layout, regardless of the physical enum ordering in the source.
pub fn render_markdown(errors: &[ErrorDoc]) -> String {
    let mut out = String::new();

    out.push_str("# Engine.rs Error Index\n\n");
    out.push_str("This document is generated by `cargo xtask generate`.\n\n---\n\n");

    let mut sorted = errors.to_vec();
    sorted.sort_by(|a, b| a.code.cmp(&b.code));

    for e in sorted {
        out.push_str(&format!("## {}: {}\n\n", e.code, e.name));
        out.push_str(&e.docs);
        out.push('\n');

        // Optionally append extended documentation from the sidecar file, if
        // one exists for this error code.
        if let Some(extra) = load_sidecar(&e.code) {
            out.push_str(&extra);
            out.push('\n');
        }

        out.push_str("\n---\n\n");
    }

    out
}

//
// ────────────────────────────────────────────────────────────────
//  Load sidecar
// ────────────────────────────────────────────────────────────────
//

/// Load the Markdown sidecar for a given error code, if it exists.
///
/// Sidecars live under `engine-rs-lib/errors/<CODE>.md` and provide extended
/// documentation beyond the short Rustdoc attached to the enum variant.
pub fn load_sidecar(code: &str) -> Option<String> {
    let lib_root = find_crate_root("engine-rs-lib").ok()?;
    let path = lib_root.join("errors").join(format!("{code}.md"));
    fs::read_to_string(path).ok()
}

//
// ────────────────────────────────────────────────────────────────
//  Crate discovery
// ────────────────────────────────────────────────────────────────
//

/// Locate the root directory of a crate by its package name.
///
/// This uses `cargo_metadata` to inspect the current workspace and returns
/// the directory containing the crate's `Cargo.toml`. If the crate cannot be
/// found, a `CrateNotFoundError` diagnostic is returned.
pub fn find_crate_root(crate_name: &str) -> Result<PathBuf> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .into_diagnostic()?;

    for pkg in metadata.packages {
        if pkg.name == crate_name {
            return Ok(pkg.manifest_path.parent().unwrap().to_path_buf().into());
        }
    }

    Err(CrateNotFoundError {
        crate_name: crate_name.to_string(),
    }
    .into())
}

//
// ────────────────────────────────────────────────────────────────
//  new-error scaffolder
// ────────────────────────────────────────────────────────────────
//

/// Scaffold a new error variant and sidecar for `EngineError`.
///
/// This function:
/// 1. Validates the error code format (`E0001`-style).
/// 2. Ensures the sidecar directory exists.
/// 3. Prevents overwriting an existing sidecar.
/// 4. Inserts a new variant into `EngineError`.
/// 5. Writes a stub sidecar Markdown file.
///
/// It is intentionally conservative and will fail fast if any invariant is
/// violated, surfacing a structured diagnostic.
pub fn new_error(code: &str, name: Option<&str>) -> Result<()> {
    // Enforce the `E0001`-style format: leading `E` + four ASCII digits.
    if !code.starts_with('E') || code.len() != 5 || !code[1..].chars().all(|c| c.is_ascii_digit()) {
        return Err(InvalidCodeFormatError {
            code: code.into(),
        }
        .into());
    }

    let lib_root = find_crate_root("engine-rs-lib")?;
    let errors_dir = lib_root.join("errors");
    fs::create_dir_all(&errors_dir).into_diagnostic()?;

    let sidecar_path = errors_dir.join(format!("{code}.md"));
    if sidecar_path.exists() {
        return Err(SidecarExistsError {
            path: sidecar_path.display().to_string(),
        }
        .into());
    }

    let engine_path = lib_root.join("src/core/public/diagnostics/errors/engine.rs");
    let mut content = fs::read_to_string(&engine_path).into_diagnostic()?;

    // Prevent duplicate codes by scanning the existing enum content.
    if content.contains(&format!("code = \"{code}\"")) {
        return Err(CodeAlreadyExistsError {
            code: code.into(),
        }
        .into());
    }

    let variant_name = name.unwrap_or("NewError");
    let insertion = format!(
        r#"
    /// TODO: Document {variant_name}.
    #[error("TODO: message for {variant_name}")]
    #[diagnostic(code = "{code}")]
    {variant_name},
"#
    );

    // Insert the new variant just before the closing `}` of the enum. This
    // keeps the enum definition contiguous and avoids re-parsing the file.
    if let Some(pos) = content.rfind('}') {
        content.insert_str(pos, &insertion);
    } else {
        return Err(EngineEnumNotFoundError.into());
    }

    fs::write(&engine_path, content).into_diagnostic()?;

    // Create a stub sidecar file to be filled in by the developer.
    let sidecar = format!(
        r#"# {code}: {variant_name}

TODO: Write extended documentation for {variant_name}.
"#
    );
    fs::write(&sidecar_path, sidecar).into_diagnostic()?;

    println!("Created:");
    println!("  - enum variant in {}", engine_path.display());
    println!("  - sidecar file {}", sidecar_path.display());

    Ok(())
}

//
// ────────────────────────────────────────────────────────────────
//  Validation (duplicates, sidecars, sequential, enum order)
// ────────────────────────────────────────────────────────────────
//

/// Validate global invariants across all discovered error definitions.
///
/// This function enforces:
/// - uniqueness of error codes,
/// - existence of sidecar files,
/// - strict sequential numeric ordering (`E0001`, `E0002`, …),
/// - and alignment between enum variant order and numeric code order.
///
/// Any violation is surfaced as a rich Miette diagnostic.
pub fn validate_errors(errors: &[ErrorDoc]) -> Result<()> {
    use std::collections::HashMap;

    let mut seen = HashMap::new();
    let lib_root = find_crate_root("engine-rs-lib")?;
    let errors_dir = lib_root.join("errors");

    for e in errors {
        // Enforce global uniqueness of error codes.
        if let Some(prev) = seen.insert(e.code.clone(), e) {
            return Err(DuplicateCodeError {
                code: e.code.clone(),
                file1: prev.file.display().to_string(),
                file2: e.file.display().to_string(),
            }
            .into());
        }

        // Ensure that every error code has a corresponding sidecar file.
        let sidecar = errors_dir.join(format!("{}.md", e.code));
        if !sidecar.exists() {
            return Err(MissingSidecarError {
                code: e.code.clone(),
                expected: sidecar.display().to_string(),
            }
            .into());
        }
    }

    // Sequential numeric ordering: codes must form a dense sequence starting
    // at `E0001`. This keeps the error index compact and predictable.
    let mut codes: Vec<_> = errors.iter().map(|e| e.code.clone()).collect();
    codes.sort();

    for (i, code) in codes.iter().enumerate() {
        let expected = format!("E{:04}", i + 1);
        if *code != expected {
            return Err(NonSequentialCodeError {
                expected,
                found: code.clone(),
            }
            .into());
        }
    }

    // Enum ordering: the physical order of variants in the enum must match
    // the sorted numeric order of their codes. This keeps diffs stable and
    // makes the enum easy to scan.
    let mut by_enum_order = errors.to_vec();
    by_enum_order.sort_by_key(|e| e.order);

    for (i, err) in by_enum_order.iter().enumerate() {
        let expected = &codes[i];
        if &err.code != expected {
            return Err(EnumOrderError {
                expected: expected.clone(),
                found: err.code.clone(),
                file: err.file.display().to_string(),
            }
            .into());
        }
    }

    Ok(())
}

//
// ────────────────────────────────────────────────────────────────
//  check-errors command
// ────────────────────────────────────────────────────────────────
//

/// Run the full error validation pipeline without generating the index.
///
/// This is the core implementation behind the `check` subcommand and is
/// suitable for use in CI or local pre-commit checks.
pub fn check_errors() -> Result<()> {
    let lib_root = find_crate_root("engine-rs-lib")?;
    let lib_src = lib_root.join("src");

    let mut errors = Vec::new();

    // Walk the library source tree and collect all `EngineError` variants.
    for entry in WalkDir::new(&lib_src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path).into_diagnostic()?;
        let file = syn::parse_file(&content).into_diagnostic()?;

        for item in file.items {
            if let syn::Item::Enum(e) = item {
                if e.ident == "EngineError" {
                    errors.extend(extract_error_variants(&e, path)?);
                }
            }
        }
    }

    // Reuse the same validation logic as the index generator.
    validate_errors(&errors)?;
    println!("All errors validated successfully.");
    Ok(())
}
