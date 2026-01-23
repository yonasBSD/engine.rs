use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

//
// ────────────────────────────────────────────────────────────────
//  Missing Docs
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when an `EngineError` variant is missing documentation.
///
/// This error is raised during error extraction when a variant has a
/// `#[diagnostic(code = "...")]` attribute but no associated `///` doc
/// comments. It points directly at the offending variant.
#[derive(Debug, Error, Diagnostic)]
#[error("Missing documentation for `{name}`")]
#[diagnostic(
    //code(engine::missing_docs),
    help("Add a `///` doc comment describing what this error represents")
)]
pub struct MissingDocsError {
    /// The error code associated with the undocumented variant.
    pub code: String,

    /// The name of the undocumented enum variant.
    pub name: String,

    /// The full source file containing the variant.
    #[source_code]
    pub src: NamedSource<String>,

    /// Span pointing at the undocumented variant identifier.
    #[label("this error variant has no documentation")]
    pub span: SourceSpan,
}

//
// ────────────────────────────────────────────────────────────────
//  Crate Not Found
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when a target crate cannot be found in the workspace.
///
/// This typically indicates a mismatch between the expected crate name and
/// the actual workspace configuration.
#[derive(Debug, Error, Diagnostic)]
#[error("Crate `{crate_name}` was not found in the workspace")]
#[diagnostic(
    //code(engine::crate_not_found),
    help("Ensure the crate exists and is included in the workspace members list")
)]
pub struct CrateNotFoundError {
    /// The crate name that was requested but not found.
    pub crate_name: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Invalid Code Format
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when an error code does not match the expected format.
///
/// Valid codes follow the pattern `E0001`, `E0002`, … with a leading `E`
/// and four ASCII digits.
#[derive(Debug, Error, Diagnostic)]
#[error("Invalid error code format: `{code}`")]
#[diagnostic(
    //code(engine::invalid_code_format),
    help("Use the format `E0001`, `E0002`, …")
)]
pub struct InvalidCodeFormatError {
    /// The invalid code string that was provided.
    pub code: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Sidecar Already Exists
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when attempting to scaffold a sidecar that already exists.
///
/// This prevents accidental overwrites of manually curated documentation.
#[derive(Debug, Error, Diagnostic)]
#[error("Sidecar file already exists at `{path}`")]
#[diagnostic(
    //code(engine::sidecar_exists),
    help("Remove the existing file or choose a different error code")
)]
pub struct SidecarExistsError {
    /// The path to the existing sidecar file.
    pub path: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Code Already Exists
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when an error code is already present in `EngineError`.
///
/// This ensures that each error code remains globally unique.
#[derive(Debug, Error, Diagnostic)]
#[error("Error code `{code}` already exists in `EngineError`")]
#[diagnostic(
    //code(engine::code_exists),
    help("Choose a new unique error code")
)]
pub struct CodeAlreadyExistsError {
    /// The duplicate error code.
    pub code: String,
}

//
// ────────────────────────────────────────────────────────────────
//  EngineError Enum Not Found
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when the `EngineError` enum cannot be located.
///
/// This typically indicates that the expected file layout or enum name has
/// changed without updating the xtask tooling.
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

/// Diagnostic emitted when the same error code appears more than once.
///
/// This is detected during validation by tracking all discovered codes and
/// reporting the first duplicate pair.
#[derive(Debug, Error, Diagnostic)]
#[error("Duplicate error code `{code}` found")]
#[diagnostic(
    //code(engine::duplicate_code),
    help("Ensure each error code is defined only once")
)]
pub struct DuplicateCodeError {
    /// The duplicated error code.
    pub code: String,

    /// Path to the first file where the code was found.
    pub file1: String,

    /// Path to the second file where the code was found.
    pub file2: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Missing Sidecar File
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when an error code has no corresponding sidecar file.
///
/// This enforces the convention that every error has extended documentation
/// stored alongside the library in Markdown form.
#[derive(Debug, Error, Diagnostic)]
#[error("Missing sidecar file for error code `{code}`")]
#[diagnostic(
    //code(engine::missing_sidecar),
    help("Create the sidecar file at the expected path")
)]
pub struct MissingSidecarError {
    /// The error code missing a sidecar.
    pub code: String,

    /// The path where the sidecar was expected to exist.
    pub expected: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Non‑Sequential Error Codes
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when error codes are not strictly sequential.
///
/// This enforces a dense sequence (`E0001`, `E0002`, …) with no gaps, which
/// simplifies navigation and keeps the error index predictable.
#[derive(Debug, Error, Diagnostic)]
#[error("Error codes are not sequential")]
#[diagnostic(
    //code(engine::non_sequential),
    help("Ensure codes follow the sequence E0001, E0002, … with no gaps")
)]
pub struct NonSequentialCodeError {
    /// The code that was expected at a given position.
    pub expected: String,

    /// The code that was actually found.
    pub found: String,
}

//
// ────────────────────────────────────────────────────────────────
//  Enum Order Mismatch
// ────────────────────────────────────────────────────────────────
//

/// Diagnostic emitted when enum variant order does not match numeric code order.
///
/// This ensures that the `EngineError` enum remains ordered by code, which
/// improves readability and keeps diffs stable as new errors are added.
#[derive(Debug, Error, Diagnostic)]
#[error("Error variant order does not match numeric code order")]
#[diagnostic(
    //code(engine::enum_order_mismatch),
    help("Reorder the variants in `EngineError` to match the sorted code order")
)]
pub struct EnumOrderError {
    /// The code that should appear at this position.
    pub expected: String,

    /// The code that actually appears at this position.
    pub found: String,

    /// The file containing the mismatched enum.
    pub file: String,
}
