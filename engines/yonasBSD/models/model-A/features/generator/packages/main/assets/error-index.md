# Engine.rs Error Index

This document provides extended explanations for engine.rs error codes.
Use `engine --explain <CODE>` to view these entries from the command line.

---

## E0001: Invalid custom module path

This error occurs when a custom module path contains one or more empty segments.
Empty segments appear when two dots occur consecutively:

    api..core

Empty segments are not valid identifiers in engine.rs module paths.

Example:

    .add_custom_module("api..core", &["graphql"])

Why this happens:

Engine.rs interprets dotted module paths as hierarchical identifiers.
Two dots in a row (`..`) indicate a missing identifier, which is not allowed.

Fix:

Remove the extra dots:

    .add_custom_module("api.core", &["graphql"])

Or, for deeper paths:

    .add_custom_module("api.core.a.b.c", &["graphql"])

---
