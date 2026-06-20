# Design Spec: Nemesis Error Handling Integration & v4 Updates

This specification details the implementation plan for replacing `nabu`'s custom error return types with the `nemesis::NemesisError` structured error hierarchy. It also outlines the updates needed for the project's documentation to support XFF Version 4.

## Core Architecture

The `nemesis` library provides structured, nestable, and context-annotated error types. In `nabu`, we will move from returning a flat `NabuError` to wrapping parser, validator, and IO failures inside a `NemesisError` chain.

### 1. Error Type Redefinitions (`src/error/mod.rs`)

We will redefine `Result<T>` to alias `std::result::Result<T, nemesis::NemesisError>`. 

- **Conversion**: Implement `From<NabuError> for NemesisError` with a default source label of `"nabu"`.
- **IO Errors**: Map `std::io::Error` directly to `NemesisError` via `From` with source `"io"`. This allows seamless `?` conversions on standard IO calls.
- **Extension Trait**: Re-export `nemesis::NemesisResultExt` from the `error` module so callers can easily chain `.add_ctx()` and `.add_source()`.

### 2. Public API Surface (`src/lib.rs`)

The public functions in the `nabu::serde` module will be updated:
- `read` -> `Result<XffValue, NemesisError>`
- `write` -> `Result<(), NemesisError>`
- `write_legacy` -> `Result<(), NemesisError>`
- `serialize_xff_to_buffer` -> `Result<Vec<u8>, NemesisError>`
- `remove_file` -> `Result<(), NemesisError>`

High-level context regarding the targeted file paths and serializations will be appended using `.add_source` and `.add_ctx`.

### 3. Parser and Serializer Integration (`src/xff/`)

Within version-specific parsing modules (v0 to v4), operations that fail will be annotated with their respective layer source tags (e.g., `"nabu::xff::deserializer::v4"`) and context (e.g., parent index deserialization or checksum checks).

### 4. Documentation Updates (`README.md`)

The `README.md` will be updated to:
- Document XFF Version 4 as the current version.
- Detail the features of v4 (delta encoding, naive temporal types, graph types, ASCII-TEXT, signed NaNs).
- Add the v4 specification reference (`specifications/v4.md`).
- Document the new error handling paradigm: printing `NemesisError` chains and downcasting to leaf errors.

## Verification Plan

- **Cargo Check**: Ensure compile correctness.
- **Cargo Test**: Validate the full test suite (unit tests, version integration tests, and fuzzing).
- **Doc Tests**: Ensure examples in documentation compile and pass under the new `NemesisError` signatures.
