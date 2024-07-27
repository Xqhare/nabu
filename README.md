# Nabu

> [!warning]
> This is a hobby project. It is not intended nor ready to be used in production.

Nabu is a rust library for reading and writing `.xff` files.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` files are my attempt at a general purpose file format, with Nabu as the translation layer, giving the opportunity for downstream projects to create their own files suiting their needs.
I am also trying out the `Feature` flags for the first time, to extend the usability of `.xff` files for a wide range of use-cases without needing to roll a bespoke library.

## Motivation
After finishing the [Mawu library](https://github.com/Xqhare/mawu), I wanted to dive deeper into file structures and working with bytes directly, instead of `&str` and later `chars` in Mawu. Around this time I also had my first deep dive on ASCII after rewatching "The Martian" and thus decided on making my own file format.
I wrote v0 of the `.xff` specification in just a few days, and then started working on the implementation.
This library is also a major part of my own tech stack.
As `xff` is meant to be a jack of all trades, it is important that it can be used in a wide range of use-cases.

## Roadmap
- Configuration wizard
    - For writing and reading `.xff` files containing all data needed for a project to configure itself
- Logging wizard
    - For writing and reading `.xff` files containing all data needed for a project to log its behaviour

## Implemented Features
- Key-value store

## `.xff` specifications
All specifications are in the `specifications` directory. 
V0 can be found [here](specifications/v0.md).

## Usage
All paths shown in example code are valid paths inside this repository.

### Importing
Nabu can be imported from GitHub directly:
```toml
[dependencies]
nabu = { git = "https://github.com/Xqhare/nabu" }
```

Nabu provides many features, they can be enabled by adding the following to your `Cargo.toml`:
```toml
[dependencies]
nabu = { git = "https://github.com/Xqhare/nabu", features = ["logging_wizard", "config_wizard", "key_value_store"] }
```

## Feature Flags
Nabu provides an ever increasing number of feature flags that can be enabled to extend its capabilities.
These flags have been carefully designed for full interoperability. Meaning no mutally exclusive features are present at all.

Some features have more than one flag.

### Logging Wizard

### Config Wizard

### Key Value Store
The key value store is a simple key value store for in place storage and manipulation of data.
It consists of two feature flags, `key_value_core` and `key_value_store`.
`key_value_core` provides the core functionality and can be enabled separately from `key_value_store`, however `key_value_store` can not be enabled separately from `key_value_core` as it depends on it.

Using `key_value_store` is recommended, especially if you want to store it as a `.xff` file.

#### Key Value Core
`key_value_core` is a small feature flag containing `read_core()` and `write_core()`.

`read_core()` takes a path and returns a `BTreeMap<String, XffValue>`.
A `BTreeMap<String, XffValue>` can be saved to a `.xff` file using `write_core()`.
All interaction with the BTreeMap and it's values is handled by the caller.

##### Key Value Core Usage
```rust
use nabu::key_value_core::{read_core, write_core};
use nabu::xff::value::{XffValue, Number, Data};
use std::collections::BTreeMap;

# #[cfg(feature = "key_value_core")]
# fn main() {
let mut map: BTreeMap<String, XffValue> = Default::default();
map.insert("key0".to_string(), XffValue::Number(Number::Int(42)));
let write = write_core("xff-example-data/xff-core-example-file.xff", map.clone());
assert!(write.is_ok());
let read = read_core("xff-example-data/xff-core-example-file.xff");
assert!(read.is_ok());
# }
```

#### Key Value Store
`key_value_store` is a feature flag containing `NabuDB`, a struct for in place storage and manipulation of data.
`NabuDB` is a one-stop solution for key value store holding arbitrary data.
While it is possible to simply save ASCII text and control characters, it is also possible to store arbitrary data like a picture.
Please note that in `V0` no metadata is stored with the data in the `.xff` file directly, only the data itself.
Storing this metadata is left to the user.

All possible interactions with it are shown in the example code below.

##### Key Value Store Usage
```rust
use nabu::key_value_store::NabuDB;
