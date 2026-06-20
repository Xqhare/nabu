# Nemesis Error Handling Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor nabu's error handling to return `nemesis::NemesisError` for structured, context-rich error reporting, and update the README for XFF v4.

**Architecture:** Redefine the library-wide `Result<T>` type to return `Result<T, nemesis::NemesisError>`, implementing blanket conversions (`From<NabuError>` and `From<std::io::Error>`) so all errors automatically promote to `NemesisError`. Add specific source/context annotations using `NemesisResultExt` across high-level and internal modules.

**Tech Stack:** Rust (Edition 2024), nemesis (v1.0.0)

---

### Task 1: Update `src/error/mod.rs` to Use `nemesis`

**Files:**
- Modify: `src/error/mod.rs`

- [ ] **Step 1: Replace `Result` type alias and add `From` implementations**

Modify `src/error/mod.rs` to redefine the `Result` alias and implement `From` for standard and custom errors:

```rust
// Replace lines 161-169 with:
pub type Result<T> = std::result::Result<T, nemesis::NemesisError>;

pub use nemesis::NemesisResultExt;

impl From<NabuError> for nemesis::NemesisError {
    fn from(err: NabuError) -> Self {
        nemesis::NemesisError::new("nabu", err)
    }
}

impl From<std::io::Error> for nemesis::NemesisError {
    fn from(err: std::io::Error) -> Self {
        nemesis::NemesisError::new("io", err)
    }
}
```

- [ ] **Step 2: Verify code compiles**

Run: `cargo check`
Expected: Succeeds, although it might warn about mismatched signatures in functions explicitly returning `Result<T, NabuError>`.

- [ ] **Step 3: Commit changes**

```bash
git add src/error/mod.rs
git commit -m "refactor(error): redefine Result alias to use nemesis::NemesisError"
```

---

### Task 2: Refactor Public API Signatures in `src/lib.rs`

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Update functions in `serde` module to return `NemesisError` and add context**

Modify the signatures and bodies of the public `serde` functions in `src/lib.rs`:

```rust
// In `pub mod serde`, import NemesisError:
use nemesis::NemesisError;
use nemesis::NemesisResultExt;

// Update read:
pub fn read<P>(path: P) -> Result<XffValue, NemesisError>
where
    P: AsRef<std::path::Path>,
{
    let path_with_xff_extension = path.as_ref().with_extension("xff");
    deserialize_xff(&path_with_xff_extension).add_source("nabu::serde::read").add_ctx(format!(
        "Failed to read XFF file: {}",
        path_with_xff_extension.display()
    ))
}

// Update write:
pub fn write<P, D>(path: P, data: D) -> Result<(), NemesisError>
where
    P: AsRef<std::path::Path>,
    D: Into<Vec<XffValue>>,
{
    let path_with_xff_extension = path.as_ref().with_extension("xff");
    let byte_data = serialize_xff(data.into(), XFF_VERSION)
        .add_source("nabu::serde::write")
        .add_ctx("Failed serialization")?;
    write_bytes_to_file(&path_with_xff_extension, byte_data)
        .add_source("nabu::serde::write")
        .add_ctx(format!(
            "Failed to write bytes to file: {}",
            path_with_xff_extension.display()
        ))
}

// Update write_legacy:
pub fn write_legacy<P, D>(path: P, data: D, xff_version: u8) -> Result<(), NemesisError>
where
    P: AsRef<std::path::Path>,
    D: Into<Vec<XffValue>>,
{
    let path_with_xff_extension = path.as_ref().with_extension("xff");
    let byte_data = serialize_xff(data.into(), xff_version)
        .add_source("nabu::serde::write_legacy")
        .add_ctx("Failed serialization")?;
    write_bytes_to_file(&path_with_xff_extension, byte_data)
        .add_source("nabu::serde::write_legacy")
        .add_ctx(format!(
            "Failed to write bytes to file: {}",
            path_with_xff_extension.display()
        ))
}

// Update serialize_xff_to_buffer:
pub fn serialize_xff_to_buffer<D>(data: D, xff_version: u8) -> Result<Vec<u8>, NemesisError>
where
    D: Into<Vec<XffValue>>,
{
    serialize_xff(data.into(), xff_version).add_source("nabu::serde::serialize_xff_to_buffer")
}

// Update remove_file:
pub fn remove_file<P>(path: P) -> Result<(), NemesisError>
where
    P: AsRef<std::path::Path>,
{
    let path_with_xff_extension = path.as_ref().with_extension("xff");
    std::fs::remove_file(path_with_xff_extension)
        .add_source("nabu::serde::remove_file")
        .add_ctx(format!(
            "Failed to remove XFF file: {}",
            path_with_xff_extension.display()
        ))
}
```

- [ ] **Step 2: Commit changes**

```bash
git add src/lib.rs
git commit -m "refactor(serde): update public API signatures to use NemesisError and add contexts"
```

---

### Task 3: Update Main Serializer/Deserializer Signatures and Contexts

**Files:**
- Modify: `src/xff/deserializer/mod.rs`
- Modify: `src/xff/serializer/mod.rs`

- [ ] **Step 1: Update `src/xff/deserializer/mod.rs`**

Update functions returning `Result<T, NabuError>` to return `Result<T>` and import `NemesisResultExt`:

```rust
// Replace import of NabuError with NemesisResultExt, and NemesisError:
use nemesis::{NemesisError, NemesisResultExt};
// And update Result usage:
use crate::error::Result;

// Change signatures:
pub fn deserialize_xff_from_buffer(content: &[u8]) -> Result<XffValue> {
    // ...
}

pub fn deserialize_xff(path: &Path) -> Result<XffValue> {
    let content: Vec<u8> = std::fs::read(path)
        .add_source("nabu::xff::deserialize_xff")
        .add_ctx(format!("Failed to read file from path: {}", path.display()))?;
    deserialize_xff_from_buffer(&content)
}

fn deserialize_xff_key_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    ver: u8,
) -> Result<(String, XffValue)> {
    // ...
}
```

- [ ] **Step 2: Update `src/xff/serializer/mod.rs`**

Update `serialize_xff` to annotate version-specific serialization paths:

```rust
use nemesis::NemesisResultExt;

pub fn serialize_xff(data: Vec<XffValue>, version: u8) -> Result<Vec<u8>> {
    match version {
        0 => serialize_xff_v0(data).add_source("nabu::xff::serializer::v0"),
        1 => {
            if data.is_empty() {
                return Err(NabuError::TruncatedXFF(1, version).into());
            }
            serialize_xff_v1(&data).add_source("nabu::xff::serializer::v1")
        }
        2 => {
            if data.is_empty() {
                return Err(NabuError::TruncatedXFF(1, version).into());
            }
            serialize_xff_v2(&data).add_source("nabu::xff::serializer::v2")
        }
        3 => serialize_xff_v3(&data).add_source("nabu::xff::serializer::v3"),
        4 => serialize_xff_v4(&data).add_source("nabu::xff::serializer::v4"),
        _ => Err(NabuError::UnknownXFFVersion(version).into()),
    }
}
```

- [ ] **Step 3: Commit changes**

```bash
git add src/xff/deserializer/mod.rs src/xff/serializer/mod.rs
git commit -m "refactor(xff): adapt mod serializer/deserializer to use NemesisError and add context"
```

---

### Task 4: Update Version-Specific Deserializers and Serializers

**Files:**
- Modify: `src/xff/deserializer/v0.rs`
- Modify: `src/xff/deserializer/v1.rs`
- Modify: `src/xff/deserializer/v2.rs`
- Modify: `src/xff/deserializer/v3.rs`
- Modify: `src/xff/deserializer/v4.rs`
- Modify: `src/xff/serializer/v0.rs`
- Modify: `src/xff/serializer/v1.rs`
- Modify: `src/xff/serializer/v2.rs`
- Modify: `src/xff/serializer/v3.rs`
- Modify: `src/xff/serializer/v4.rs`

- [ ] **Step 1: Update version-specific deserializer function signatures**

For each of the deserializer files `v0.rs`, `v1.rs`, `v2.rs`, update their signatures to return `Result<XffValue>` (using `crate::error::Result`) instead of returning `Result<XffValue, NabuError>`. Make sure to import `crate::error::Result` and `nemesis::NemesisResultExt` where needed.

For `v3.rs` and `v4.rs`, they already return `Result<XffValue>`, so they will automatically return the new `Result` alias containing `NemesisError`.

- [ ] **Step 2: Update version-specific serializer function signatures**

Ensure they return `Result<Vec<u8>>` and use the proper type alias. Add `nemesis::NemesisResultExt` to annotate nested errors where appropriate.

- [ ] **Step 3: Run `cargo check` to verify all code builds**

Run: `cargo check`
Expected: Success with no syntax or type errors.

- [ ] **Step 4: Commit changes**

```bash
git add src/xff/deserializer/v0.rs src/xff/deserializer/v1.rs src/xff/deserializer/v2.rs src/xff/deserializer/v3.rs src/xff/deserializer/v4.rs src/xff/serializer/v0.rs src/xff/serializer/v1.rs src/xff/serializer/v2.rs src/xff/serializer/v3.rs src/xff/serializer/v4.rs
git commit -m "refactor(xff): update all version-specific signatures to use NemesisError Result"
```

---

### Task 5: Verify the Test Suite and Resolve Test compilation failures

**Files:**
- Modify: `tests/metadata.rs` (if needed)
- Modify: `tests/v0.rs` (if needed)
- Modify: `tests/v1.rs` (if needed)
- Modify: `tests/v2.rs` (if needed)
- Modify: `tests/v3.rs` (if needed)
- Modify: `tests/v4.rs` (if needed)

- [ ] **Step 1: Run standard Cargo tests**

Run: `cargo test`
Expected: All tests pass. If any tests fail compilation due to signature or type mismatch, resolve them. For example, tests might match on `NabuError` variant if they import it, but standard error propagation assertions like `.is_err()` should pass without changes.

- [ ] **Step 2: Commit any test adjustments**

```bash
git add tests/
git commit -m "test: align tests with Nemesis error return type"
```

---

### Task 6: Update project documentation (`README.md`)

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update README.md version and feature list**

Modify the version references in `README.md` to target Version 4:
- In introductory notes, reference Version 4 as the most recent specification.
- In `## purpose` and `## Features` sections, add mentions of the delta encoding, remapped markers (Hamming distance of 4), naive temporal types, graph parent type, ASCII-TEXT, signed NaNs, and integrated Nemesis error handling.

- [ ] **Step 2: Rewrite `### Errors` section**

Replace the old error section with:

```markdown
### Errors

Nabu returns `nemesis::NemesisError` for error handling, which integrates structural error nesting, source tagging, and context propagation.

#### Printing Errors

A `NemesisError` prints the entire causal error chain along with source labels and contexts.

```rust
if let Err(err) = nabu::serde::read("data.xff") {
    // Prints formatted nested error hierarchy
    eprintln!("{}", err);
}
```

#### Downcasting to Leaf Errors

You can downcast the leaf error to investigate the root cause (such as a specific `NabuError` variant or a standard `std::io::Error`):

```rust
use std::io;
use nabu::error::NabuError;

if let Err(err) = nabu::serde::read("data.xff") {
    if let Some(io_err) = err.downcast_ref::<io::Error>() {
        eprintln!("IO issue: {}", io_err);
    } else if let Some(nabu_err) = err.downcast_ref::<NabuError>() {
        match nabu_err {
            NabuError::EmptyXFF => eprintln!("Empty XFF file"),
            _ => eprintln!("Parsing error: {}", nabu_err),
        }
    }
}
```
```

- [ ] **Step 3: Run doc-tests**

Run: `cargo test --doc`
Expected: PASS

- [ ] **Step 4: Commit README.md**

```bash
git add README.md
git commit -m "docs(readme): update README for v4 and Nemesis error handling"
```
