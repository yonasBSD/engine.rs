use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

//
// ────────────────────────────────────────────────────────────────
//  Missing Docs
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Missing documentation for `{name}`")]
#[diagnostic(
    //code(engine::missing_docs),
    help("Add a `///` doc comment describing what this error represents")
)]
pub struct MissingDocsError {
    pub code: String,
    pub name: String,

    #[source_code]
    pub src: NamedSource<String>,

    #[label("this error variant has no documentation")]
    pub span: SourceSpan,
}

//
// ────────────────────────────────────────────────────────────────
//  Crate Not Found
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Crate `{crate_name}` was not found in the workspace")]
#[diagnostic(
    //code(engine::crate_not_found),
    help("Ensure the crate exists and is included in the workspace members list")
)]
pub struct CrateNotFoundError {
    pub crate_name: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Invalid Code Format
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid error code format: `{code}`")]
#[diagnostic(
    //code(engine::invalid_code_format),
    help("Use the format `E0001`, `E0002`, …")
)]
pub struct InvalidCodeFormatError {
    pub code: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Sidecar Already Exists
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Sidecar file already exists at `{path}`")]
#[diagnostic(
    //code(engine::sidecar_exists),
    help("Remove the existing file or choose a different error code")
)]
pub struct SidecarExistsError {
    pub path: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Code Already Exists
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Error code `{code}` already exists in `EngineError`")]
#[diagnostic(
    //code(engine::code_exists),
    help("Choose a new unique error code")
)]
pub struct CodeAlreadyExistsError {
    pub code: String,
}

//
// ────────────────────────────────────────────────────────────────
//  EngineError Enum Not Found
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Could not find the `EngineError` enum in the source file")]
#[diagnostic(
    //code(engine::enum_not_found),
    help("Ensure the file contains a `pub enum EngineError` declaration")
)]
pub struct EngineEnumNotFoundError;

//
// ────────────────────────────────────────────────────────────────
//  Duplicate Error Code
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Duplicate error code `{code}` found")]
#[diagnostic(
    //code(engine::duplicate_code),
    help("Ensure each error code is defined only once")
)]
pub struct DuplicateCodeError {
    pub code: String,
    pub file1: String,
    pub file2: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Missing Sidecar File
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Missing sidecar file for error code `{code}`")]
#[diagnostic(
    //code(engine::missing_sidecar),
    help("Create the sidecar file at the expected path")
)]
pub struct MissingSidecarError {
    pub code: String,
    pub expected: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Non‑Sequential Error Codes
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Error codes are not sequential")]
#[diagnostic(
    //code(engine::non_sequential),
    help("Ensure codes follow the sequence E0001, E0002, … with no gaps")
)]
pub struct NonSequentialCodeError {
    pub expected: String,
    pub found: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Enum Order Mismatch
// ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Error, Diagnostic)]
#[error("Error variant order does not match numeric code order")]
#[diagnostic(
    //code(engine::enum_order_mismatch),
    help("Reorder the variants in `EngineError` to match the sorted code order")
)]
pub struct EnumOrderError {
    pub expected: String,
    pub found: String,
    pub file: String,
}
