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

pub fn extract_error_variants(e: &ItemEnum, file: &Path) -> Result<Vec<ErrorDoc>> {
    let mut out = Vec::new();
    let file_text = fs::read_to_string(file).into_diagnostic()?;

    for (idx, variant) in e.variants.iter().enumerate() {
        let code = find_diagnostic_code(&variant.attrs)?;
        if let Some(code) = code {
            let docs = collect_doc_comments(&variant.attrs);

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

pub fn generate_error_index() -> Result<()> {
    let lib_root = find_crate_root("engine-rs-lib")?;
    let lib_src = lib_root.join("src");

    let mut errors = Vec::new();

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

    validate_errors(&errors)?;

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

#[derive(Debug, Clone)]
pub(crate) struct ErrorDoc {
    pub code: String,
    pub name: String,
    pub docs: String,
    pub file: PathBuf,
    pub order: usize,
}

//
// ────────────────────────────────────────────────────────────────
//  Extract #[diagnostic(code = "...")]
// ────────────────────────────────────────────────────────────────
//

pub fn find_diagnostic_code(attrs: &[Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if attr.path().is_ident("diagnostic") {
            let mut found: Option<String> = None;

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

pub fn new_error(code: &str, name: Option<&str>) -> Result<()> {
    if !code.starts_with('E') || code.len() != 5 || !code[1..].chars().all(|c| c.is_ascii_digit()) {
        return Err(InvalidCodeFormatError { code: code.into() }.into());
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

    if content.contains(&format!("code = \"{code}\"")) {
        return Err(CodeAlreadyExistsError { code: code.into() }.into());
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

    if let Some(pos) = content.rfind('}') {
        content.insert_str(pos, &insertion);
    } else {
        return Err(EngineEnumNotFoundError.into());
    }

    fs::write(&engine_path, content).into_diagnostic()?;

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

pub fn validate_errors(errors: &[ErrorDoc]) -> Result<()> {
    use std::collections::HashMap;

    let mut seen = HashMap::new();
    let lib_root = find_crate_root("engine-rs-lib")?;
    let errors_dir = lib_root.join("errors");

    for e in errors {
        if let Some(prev) = seen.insert(e.code.clone(), e) {
            return Err(DuplicateCodeError {
                code: e.code.clone(),
                file1: prev.file.display().to_string(),
                file2: e.file.display().to_string(),
            }
            .into());
        }

        let sidecar = errors_dir.join(format!("{}.md", e.code));
        if !sidecar.exists() {
            return Err(MissingSidecarError {
                code: e.code.clone(),
                expected: sidecar.display().to_string(),
            }
            .into());
        }
    }

    // Sequential numeric ordering
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

    // Enum ordering
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

pub fn check_errors() -> Result<()> {
    let lib_root = find_crate_root("engine-rs-lib")?;
    let lib_src = lib_root.join("src");

    let mut errors = Vec::new();

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

    validate_errors(&errors)?;
    println!("All errors validated successfully.");
    Ok(())
}
