# engine.rs

![Licenses](https://github.com/yonasBSD/engine.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/engine.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/engine.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/engine.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/engine.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/engine.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/engine.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_engine.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_engine.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_engine.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_engine.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_engine.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_engine.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/engine.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/engine.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/engine.rs/pkgs/container/engine.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/engine.rs/example.svg)](https://hub.docker.com/r/engine.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/engine.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/engine.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/engine.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/engine.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/engine.rs.svg)](https://github.com/yonasBSD/engine.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/engine.rs.svg)](https://github.com/yonasBSD/engine.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

engine.rs - Core Engine Generator

---

# Core Engine Generator

This tool initializes a specialized, deep-hierarchy Rust workspace (12-level
tree) designed for any Rust project.

It solves the "multiple workspace roots" collision by establishing a single
source of truth at the `features` level while maintaining modularity across
nested packages.

## 🛡 Cryptographic Integrity (BLAKE3)

Unlike standard scaffolders, this engine implements a Deep-Hash Manifest system:

    Atomic Generation: Files are written with specific, compiler-ready boilerplate.

    Manifesting: Every file's content is hashed using the BLAKE3 algorithm during creation.

    Fail-Fast Verification: Before the process completes, the engine performs a second pass, reading every file from the disk to verify its hash against the manifest.

    Zero-Tolerance: If a single byte is corrupted or a file fails to write, the engine triggers an immediate exit to prevent building on a broken foundation.

## 🏗 Directory Architecture

The generator builds the following structure to ensure the Rust compiler correctly recognizes nested modules across the filesystem:

```text

engines/
└── yonasBSD/
    └── models/
        └── model-A/
            └── features/ [Workspace Root]
                ├── Cargo.toml
                └── feature-A/
                    └── packages/
                        ├── traits/
                        │   ├── Cargo.toml
                        │   └── src/ (lib.rs)
                        └── package-A/
                            ├── Cargo.toml
                            ├── mod.rs
                            ├── benches/
                            ├── examples/
                            ├── src/
                            ├── vendor/
                            ├── tests/
                            │   └── common/
                            ├── enums/
                            │   ├── mod.rs
                            │   └── tests/ (unit, integration)
                            ├── utils/
                            │   ├── mod.rs
                            │   └── tests/ (unit, integration)
                            ├── traits/
                            │   ├── mod.rs
                            │   └── tests/ (unit, integration)
                            └── core/
                                ├── mod.rs
                                ├── internal/
                                │   ├── mod.rs
                                │   └── tests/ (unit, integration)
                                ├── backends/
                                │   ├── mod.rs
                                │   └── tests/ (unit, integration)
                                └── frontends/
                                    ├── Cargo.toml
                                    ├── src/ (lib.rs)
                                    └── tests/ (unit, integration)
```

## 🚀 Getting Started

### Prerequisites

- Rust: 1.56+ (2021 Edition)

### Execution

To generate the project structure:

```sh
cargo run --release
```

## 🛠 Workspace Logic

### Unified Root

Cargo restricts workspaces to a single root. This structure bypasses nested workspace errors by:

- Workspace Anchoring: The [workspace] definition is placed at the features level to allow deep nesting without root collisions.
- Boilerplate Injection: Automatically populates mod.rs and lib.rs to ensure every level is reachable by the compiler via pub mod declarations.
- Fail-Fast Verification: If any generated file does not match the hardcoded BLAKE3 hash, the build is flagged as compromised.

### Module Wiring

The generator automatically populates mod.rs and lib.rs files to ensure every directory in the 12-level tree is reachable by the compiler. It adheres to a strict "one declaration per line" format for pub mod and use statements.

## 📝 Components Created

- Traits Package: Defines the core ModelComponent trait.
- Logic Tests: Pre-configured integration_logic.rs and structure.rs for immediate validation.
- Support Trees: Comprehensive directory mapping for unit and integration tests across all sub-modules.

> [!NOTE]
> Every generated directory includes a README.md anchor to ensure directory persistence across version control systems.
