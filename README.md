# Nabu

> [!note]
> This is a hobby project. It is not intended nor ready to be used in production.

Nabu is a rust library for reading and writing `.xff` files.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` is a general purpose file format, with Nabu as the translation layer.

As with all my projects, this documentation contains everything you never wanted to know about `.xff` files, Nabu and how to work with them.

This README documents the usage of the most recent version of `.xff`: Version 1.
The documentation for the previous version 0 is [here](LEGACY_V0_README.md).

All features present in the codebase are used in version 0 only.

## Motivation
After finishing [Mawu](https://github.com/Xqhare/mawu), I wanted to dive deeper into file structures and working with bytes directly, instead of `&str` and later `chars` like in Mawu. Around this time I also had my first deep dive on ASCII after rewatching "The Martian" and thus decided on making my own file format.
I wrote v0 of the `.xff` specification in just a few days, and then started working on the implementation of v0.
After a few weeks of work and running into several issues and design oversights (as expected), I started working on v1.
V1 has morphed the `.xff` specification from a simple, to a more complex format akin to a JSON variation capable of storing arbitrary data in a binary format.
This library is also going to be a major part of my own tech stack.
As `xff` is meant to be a jack of all trades, it is important that it can be used in a wide range of use-cases.

## Naming
This library's namesake is the ancient Babylonian god Nabu, the god of literacy, rational arts and scribes.
As the inventor of writing, Nabu is a fitting namesake for a tool designed to create and interpret a new form of written data.

## Contents
- [Motivation](#motivation)
- [Naming](#naming)
- [Contents](#contents)
- [Roadmap](#roadmap)
- [Implemented Features](#implemented-features)
- [`.xff` specifications](#xff-specifications)
- [Usage](#usage)
    - [Importing](#importing)
    - [Usage of serde](#usage-of-serde)
    - [XffValue](#xffvalue)
- [Testing](#testing)
    
## Roadmap

- Configuration wizard
    - For writing and reading `.xff` files containing all data needed for a project to configure itself
    - Wait for v1 maybe?

- Change from reading the entire file to using `BufReader` to read files larger than the actual memory available.

- Finish README
    - Stabilise all code examples
        - Only v0 doc was removed, v1 needs to be added
- Finish doc
    - Finish lib doc


## Implemented Features

- Performance

- Error rework
    - actual casting of unique and meaningful errors

## `.xff` specification
All specifications are in the `specifications` directory.

- [V0](specifications/v0.md).
- [V1](specifications/v1.md).

V2 is not yet finalized, but my musings about it can be found [here](specifications/v2.md).

## Usage

### Importing
Nabu can be imported from GitHub directly:
```toml
[dependencies]
nabu = { git = "https://github.com/Xqhare/nabu" }
```

### Serde
Serde is a shorthand for serializing and deserializing. 
This module contains all the functions needed for serializing and deserializing `.xff` files.

#### Usage of serde
No matter what the extension of the path you provide, it will be converted to ".xff".
For example, if you provide "example.txt", it will be converted to "example.xff".

```rust
# use nabu::serde::remove_file;
use nabu::serde::{read, write};
use nabu::xff::value::XffValue;
let path = "xff-example-data/serde-example.txt";
let path_2 = "xff-example-data/serde-example.xff";
let data = {
    vec![
        XffValue::from("hello mom"),
    ]
};
let tmp = write(path, data.clone());
assert!(tmp.is_ok());
let tmp_2 = read(path_2);
assert!(tmp_2.is_ok());
let ok = tmp_2.unwrap();
assert_eq!(ok[0], data[0]);
# remove_file(path_2).unwrap();
```

### XffValue
A XffValue is the type used by Nabu to store and manipulate data.
There are basic types such as `String`, `Number`, `Boolean`, `Null` and `Data`, along with the `Array` and `Object` types.

An `Array` is a list of `XffValue`s, and an `Object` is a list of key-value pairs of `String`s and `XffValue`s.

```rust
use nabu::xff::value::{Data, Number, XffValue};
let data = XffValue::String("hello mom".to_string());
let data_2 = XffValue::Number(Number::from(-42));
let data_4 = XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
```

There are many implementations of the `From` trait for the `XffValue` enum.
This example contains all types that can be converted to `XffValue`.
Starting off with converting to a `XffValue` directly.
```rust
use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};

let string_0 = XffValue::from("hello mom");
let string_1 = XffValue::from("hello mom".to_string());

let number_0 = XffValue::from(42);
let number_1 = XffValue::from(-42);
let number_2 = XffValue::from(42.2);
let number_3 = XffValue::from(-42.2);
let number_4 = XffValue::from(f64::MAX);
let number_5 = XffValue::from(usize::MAX);
let number_6 = XffValue::from(isize::MAX);
let number_7 = XffValue::from(u8::MAX);
let number_8 = XffValue::from(i8::MAX);

let data_0 = XffValue::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
let data_1 = XffValue::from(vec![152, 142, 202, 33, 54, 5, 86, 197, 38, 209]);


let number_9 = XffValue::Number(Number::from(-42));
let number_10 = XffValue::Number(Number::from(42));
let number_11 = XffValue::Number(Number::from(42.2));
let number_12 = XffValue::Number(Number::from(-42.2));
let number_13 = XffValue::Number(Number::from(f64::MAX));
let number_14 = XffValue::Number(Number::from(usize::MAX));
let number_15 = XffValue::Number(Number::from(isize::MAX));
let number_16 = XffValue::Number(Number::from(u8::MAX));
let number_17 = XffValue::Number(Number::from(i8::MAX));

let data_2 = XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
let data_3 = XffValue::Data(Data::from(vec![152, 142, 202, 33, 54, 5, 86, 197, 38, 209]));


let string_2 = XffValue::String("hello mom".to_string());

let number_18 = XffValue::Number(Number::Unsigned(42));
let number_19 = XffValue::Number(Number::Integer(-42));
let number_20 = XffValue::Number(Number::Float(42.2));

```

This list contains all types that can be converted to `XffValue` by using `XffValue::from()`.
- `XffValue` 
    - `&str`, `String`
    - `usize`, `u8`, `u16`, `u32`, `u64`
    - `isize`, `i8`, `i16`, `i32`, `i64`
    - `f32`, `f64`
    - `Vec<u8>`, `Vec<CommandCharacter>`

- `XffValue::String`
    - `XffValue::String("hello mom".to_string())`
- `XffValue::Number`
    - `usize`, `u8`, `u16`, `u32`, `u64`
    - `isize`, `i8`, `i16`, `i32`, `i64`
    - `f32`, `f64`
- `XffValue::Data`
    - `Vec<u8>` the byte-array of the data to be stored

The `XffValue` also has several associated functions.
- `XffValue::String`
- `XffValue::Number`
    - `as_u8()` converts the number into an ASCII encoded byte-stream
- `XffValue::Data`

### Testing
Nabu can be tested with the following commands:
```bash
cargo test
```

Or:
```bash
cargo test --all-features -- --include-ignored
```

> [!note]
> Ignored tests require the `--all-features` flag as they are feature dependent.
