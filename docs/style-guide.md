# Engine.rs Style Guide  
### A Declarative, Vertical, Diff‑Friendly Rust Formatting Philosophy

This project follows a highly intentional formatting style designed to make complex Rust codebases easier to read, maintain, and evolve. The goal is not to mimic the default Rust style, but to adopt a layout that supports:

- **Vertical readability** for complex builder patterns  
- **Predictable diffs** during refactors  
- **Clear diagnostic construction** (Ariadne, Miette, custom error pipelines)  
- **Contributor‑friendly onboarding**  
- **Stable formatting across toolchains**

This guide explains *why* our `rustfmt.toml` looks the way it does and how contributors should write code to fit the project’s philosophy.

---

## 1. Core Principles

### 🧱 1.1 Verticality over horizontal density  
We prefer tall, declarative layouts. This improves scanning, especially in:

- multi‑argument functions  
- fluent builder chains  
- diagnostic constructors  
- nested control flow  

If a line feels “wide,” break it.

### 🔄 1.2 Diff‑friendliness  
Formatting should minimize churn. We avoid alignment columns and single‑line struct literals because they create noisy diffs when identifiers change.

### 🧭 1.3 Predictability  
Formatting should be deterministic across contributors. We pin the formatter version and use the 2024 style edition to ensure consistent output.

### 📚 1.4 Documentation clarity  
Comments and doc examples should remain readable on standard displays. We wrap comments and format code blocks inside documentation.

---

## 2. Layout & Indentation

### 2.1 Block indentation  
We use **block** indentation, not visual alignment. This prevents “indent drift” as identifiers grow.

### 2.2 Tall function signatures  
Functions, closures, and calls with multiple parameters should expand vertically:

```rust
fn build_error(
    span: Span,
    message: String,
    help: Option<String>,
) -> Diagnostic { … }
```

This is intentional and expected.

### 2.3 Multi‑line where clauses  
We always expand `where` clauses:

```rust
fn parse<T>()
where
    T: DeserializeOwned + Debug,
{ … }
```

This matches our vertical style.

---

## 3. Structs, Enums & Match Arms

### 3.1 Struct literals are always multi‑line  
Even small ones:

```rust
let config = Config {
    root,
    mode,
    verbose,
};
```

### 3.2 No alignment columns  
We disable alignment thresholds for:

- struct fields  
- enum discriminants  

This avoids massive diffs when a single identifier changes length.

### 3.3 Trailing commas in match arms  
Always include trailing commas. This makes adding or reordering arms painless.

---

## 4. Fluent APIs & Builder Patterns

This project uses fluent APIs heavily (Ariadne, Miette, custom diagnostics).  
We optimize formatting for readability:

- Chains break early (`chain_width = 60`)  
- Overflowing delimited expressions are allowed  
- Each method call should appear on its own line when it improves clarity  

Example:

```rust
report
    .with_message("Invalid configuration")
    .with_label(label)
    .with_help("Check your engine.toml file");
```

---

## 5. Imports

### 5.1 Crate‑level granularity  
Imports are grouped at the crate level, not per‑module.

### 5.2 Category grouping  
Imports appear in this order:

1. Standard library  
2. External crates  
3. Local modules  

### 5.3 Automatic reordering  
Imports are always sorted. Contributors should not manually reorder them.

---

## 6. Documentation & Comments

### 6.1 Wrapped comments  
We wrap comments at 100 characters to avoid ultra‑wide drift.

### 6.2 Normalized comment formatting  
We normalize whitespace and structure in comments for consistency.

### 6.3 Formatted code blocks in docs  
Doc examples are formatted by rustfmt. Contributors should write examples that are idiomatic and compilable when possible.

---

## 7. Toolchain & Stability

### 7.1 Nightly requirement  
We use:

- `style_edition = "2024"`  
- `unstable_features = true`  

Contributors must use the nightly toolchain.

### 7.2 Formatter version pinning  
We lock formatting behavior to the Rust 2.0 formatter spec:

```
version = "Two"
```

This ensures consistent diffs across machines and CI.

---

## 8. Practical Tips for Contributors

- Prefer **tall** code over wide code.  
- Break chains early.  
- Avoid clever one‑liners.  
- Keep diagnostics readable.  
- Don’t manually align anything.  
- Let rustfmt handle imports.  
- Write doc examples that future contributors can trust.  
- If something looks “too horizontal,” expand it.  

---

## 9. Philosophy Summary

This style guide is not about aesthetics — it’s about **maintainability**.

The engine.rs codebase is built around:

- declarative configuration  
- macro‑driven DSLs  
- complex diagnostics  
- multi‑layered command systems  

A vertical, diff‑friendly style makes these systems easier to understand and evolve.  
Consistency is the real goal.
