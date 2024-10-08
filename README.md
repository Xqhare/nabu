# Nabu

> [!note]
> This is a hobby project. It is not intended nor ready to be used in production.

Nabu is a rust library for reading and writing `.xff` files.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` is a general purpose file format, with Nabu acting as a serializer and deserializer, as well as an IO abstraction.

As with all my projects, this documentation contains everything you never wanted to know about `.xff` files or Nabu.

This README documents the usage of the most recent version of `.xff`: Version 1.

If you would like to use version 0, please refer to the [releases page](https://github.com/Xqhare/nabu/releases/tag/v.0.6.2). 
There you can find the documentation and code for version 0. 
While all code is still present the usage has changed slightly. Mainly the return value is now no longer a `Vec<XffValue>`, instead it is a single `XffValue::Array`.

All features present in the codebase are used in version 0 only.

## Purpose
Nabu was written to satiate my want of being able to embed binary data inside a JSON like data structure.
I also tried to make it easy to detect malformed data, and to make it slightly harder to manipulate the file by hand.

## Motivation
After finishing [Mawu](https://github.com/Xqhare/mawu), I wanted to dive deeper into file structures and working with bytes directly, instead of `&str` and later `chars` like in Mawu. Around this time I also had my first deep dive on ASCII after rewatching "The Martian" and thus decided on making my own file format.
I wrote v0 of the `.xff` specification in just a few days, and then started working on the implementation of v0.
After a few weeks of work and running into several issues and design oversights (as expected), I started work on v1.
V1 has morphed the `.xff` specification from a simple, to a more complex format akin to a JSON variation capable of storing arbitrary data in a binary format.
As `xff` is meant to be a jack of all trades, it is important that it can be used in a wide range of use-cases.
Because I like creating problems for myself, the `.xff` specification contains several error detection features. If these are of any use to anyone (or actually work as intended), only time will tell.

## Naming
As with all my projects, Nabu is named after an ancient god.

This library's namesake is the ancient Babylonian god Nabu, the god of literacy, rational arts and scribes.
As the inventor of writing, Nabu is a fitting namesake for a tool designed to create and interpret a new form of written data.

I am still undecided if NABU will also be a recursive acronym.
The only candidate is 'Nabu's Archival Binary Utility' as of now. I don't really like it though.

## Contents
- [Purpose](#purpose)
- [Motivation](#motivation)
- [Naming](#naming)
- [Contents](#contents)
- [Roadmap](#roadmap)
- [Features](#features)
- [`.xff` specification](#xff-specification)
- [Usage](#usage)
    - [Importing](#importing)
    - [A Hello World of sorts](#a-hello-world-of-sorts)
    - [Serde](#serde)
        - [Usage of serde](#usage-of-serde)
    - [XffValue](#xffvalue)
        - [From](#from)
        - [Associated Functions](#associated-functions)
        - [Notes on value types](#notes-on-value-types)
            - [Object](#object)
            - [Array](#array)
- [Errors](#errors)
    - [IO Errors](#ioerror)
    - [InternalError](#internalerror)
- [Testing](#testing)
    
## Roadmap

## Features

- Storage of a variety of data types
    - Basic data types
        - Strings, Numbers, Boolean's, Null
    - Arrays, Objects
    - Arbitrary data
- Performant
    - 100MB are read in approximately 3 seconds
- Meaningful errors
- Fully documented
- High test coverage

## `.xff` specification
To use Nabu it is not needed to have read the specification, but it is recommended.

All specifications are in the `specifications` directory. 

- [V0](specifications/v0.md).
- [V1](specifications/v1.md).

V2 is not yet finalized, but my musings about it can be found [here](specifications/v2.md).

## Usage

### Importing
Nabu may be imported from GitHub directly:
```toml
[dependencies]
nabu = { git = "https://github.com/Xqhare/nabu" }
```

Please make sure to run `cargo update` to pull the latest version of Nabu from GitHub.

Nabu contains the `serde` module, along with a `XffValue` type, as well as all variants of the `XffValue` type.

A quick overview:
```rust
// All functionality needed to read, parse and write `.xff` files
use nabu::serde::{read, write, remove_file};
// All types needed to store and manipulate entries stored in `.xff` files
use nabu::XffValue;
use nabu::{Array, Object, Data, Number};
```

### A Hello World of sorts
While I highly recommend reading the rest of the documentation, here is a example and future quick reference of how to use Nabu, covering all types and the usage of serde:
```rust
use nabu::serde::{read, write, remove_file};
use nabu::XffValue;
use nabu::{Array, Object, Data, Number};

let path = "xff-example-data/hello-world.xff";

let mut object: Object = Object::new();

object.insert("String", XffValue::from("Hi mom!"));
object.insert("Number", XffValue::from(usize::MAX));
object.insert("Number", XffValue::from(-42));
object.insert("Number", XffValue::from(42.69));
object.insert("Boolean", XffValue::from(true));
object.insert("Null", XffValue::from(XffValue::Null));

let mut array: Array = Array::new();
array.push(XffValue::from("Hello mom!"));
array.push(XffValue::from(usize::MAX));

object.insert("Array", XffValue::from(array));

object.insert("Data", XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));

let value = XffValue::from(object);

let write = write(path, value.clone());
assert!(write.is_ok());
let read = read(path);
assert!(read.is_ok());
let ok = read.unwrap();
assert_eq!(ok, value);
let remove = remove_file(path);
assert!(remove.is_ok());
```


### Serde
`Serde` is a shorthand for serializing and deserializing. 
This module contains all the functions needed for serializing and deserializing `.xff` files, as well as a convenience function for deleting files.

#### Usage of serde
No matter what the extension of the path you provide, it will be converted to ".xff".
For example, if you provide "example.txt", it will be converted to "example.xff".

```rust
# use nabu::serde::remove_file;
use nabu::serde::{read, write};
use nabu::XffValue;
let path = "xff-example-data/serde-example.txt";
let path_2 = "xff-example-data/serde-example.xff";

let data = XffValue::String("hello mom".to_string());

let write = write(path, data.clone());
assert!(write.is_ok());
let read = read(path_2);
assert!(read.is_ok());
let ok = read.unwrap();
assert_eq!(ok, data);
# remove_file(path_2).unwrap();
```

### XffValue
A XffValue is the type used by Nabu to store and manipulate data.
There are basic types such as `String`, `Number`, `Boolean`, `Null` and `Data`, along with the `Array` and `Object` types.

An `Array` is a list of `XffValue`s, and an `Object` is a list of key-value pairs of `String`s and `XffValue`s.

```rust
use nabu::{Data, Number, XffValue};
let data = XffValue::String("hello mom".to_string());
let data_2 = XffValue::Number(Number::from(-42));
let data_4 = XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
```

The types are explained along with the `XffValue` enum, instead of in their own chapters, as I found it easier to understand.

### From

There are many implementations of the `From` trait for the `XffValue` enum, this is a comprehensive list:

- `XffValue::from()`
    - `&str`, `String` -> `XffValue::String`
    - `usize`, `u8`, `u16`, `u32`, `u64` -> `XffValue::Number`
    - `isize`, `i8`, `i16`, `i32`, `i64` -> `XffValue::Number`
    - `f32`, `f64` -> `XffValue::Number`
    - `Number` -> `XffValue::Number`
    - `bool` -> `XffValue::Boolean`
    - `Vec<u8>` -> `XffValue::Data`
    - `Data` -> `XffValue::Data`
    - `Vec<XffValue>` -> `XffValue::Array`
    - `Array` -> `XffValue::Array`
    - `HashMap<S, V>`, `BTreeMap<S, V>` or `Vec<(S, V)>` where `S` can be converted to `String` and `V` to `XffValue` -> `XffValue::Object`
    - `Object` -> `XffValue::Object`

Along with a comprehensive example:
```rust
use nabu::{XffValue, Data, Number};

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

There are also several implementations of the `From` trait for the different types:

- `Number`
    - `usize`, `u8`, `u16`, `u32`, `u64` -> `Number::Unsigned`
    - `isize`, `i8`, `i16`, `i32`, `i64` -> `Number::Integer`
    - `f32`, `f64` -> `Number::Float`

- `Data`
    - `Vec<D>` where `D` can be converted to `u8` -> `Data`

- `Array`
    - `Vec<V>` where `V` can be converted to `XffValue` -> `Array`

- `Object`
    - `HashMap<S, V>`, `BTreeMap<S, V>` or `Vec<(S, V)>` where `S` can be converted to `String` and `V` to `XffValue` -> `Object`

### Associated Functions

`XffValue` has several associated functions:
- `into_{type}`
    - Returns an option if the value is of the requested type.
- `is_{type}`
    - Returns a bool if the value is of the requested type.
- `is_true`, `is_false` and `is_null`
    - Return true if the assertion is true

A quick example using `Number`, but it is applicable to any type:
```rust
use nabu::{XffValue, Number};

let number = XffValue::Number(Number::from(42));

assert!(number.is_number());
assert!(!number.is_string());

let inner_number = number.into_number().unwrap();
assert_eq!(inner_number, Number::from(42));
let inner_value_wrong_type = number.into_data();
assert_eq!(inner_value_wrong_type, None);
```

`Number` has the associated functions:

- `into_usize`, `into_isize`, `into_f64`
- `is_unsigned`, `is_integer`, `is_float`
- `as_string` -> This converts any number into a string
- `as_u8()` -> converts the number into an ASCII encoded byte-stream

`Object` has the associated functions:

- `new` -> creates a new empty `Object`
- `into_btree_map`, `into_hash_map` -> converts the object into a `BTreeMap` or `HashMap`

The underlying data can be interacted with directly by using:

- `is_empty`
- `clear`
- `insert`
- `remove`
- `get`
- `contains_key`
- `iter`
- `len`

`Array` has the associated functions:

- `new` -> creates a new empty `Array`
- `into_vec` -> converts the array into a `Vec`

The underlying data can be interacted with directly by using:

- `is_empty`
- `clear`
- `push`
- `pop`
- `get`
- `contains`
- `iter`
- `len`
- `insert`
- `remove`

`Data` has the associated functions:

- `is_empty`
- `clear`
- `len`
- `into_vec`

#### Notes on value types
All types are printable.
The default returned by `XffValue::default()` is `XffValue::Null`.

##### `Object`
Any `Object` can be indexed with strings. This returns a reference by key.
```rust
use nabu::{Object, XffValue};

let mut object = Object::new();

object.insert("Key", "hello mom");
object.insert("Key2", -42);

let value = &object["Key"];
assert_eq!(value, &XffValue::from("hello mom"));

let value2 = &object["Key2"];
assert_eq!(value2, &XffValue::from(-42));
```

##### `Array`
Any `Array` can be indexed with integers. This returns a reference by index.
```rust
use nabu::{Array, XffValue};

let mut array = Array::new();

array.push("hello mom");

let value = &array[0];
assert_eq!(value, &XffValue::from("hello mom"));
```

### Errors
Nabu will return one of two larger groups of errors:

1. `IOError`
2. `InternalError`

#### `IOError`
These errors are just the standard IO errors.
Read and write permissions and the such.

#### `InternalError`
These errors are errors that are caused while parsing or encoding a `.xff` file and are not expected to be encountered in normal use.
These errors are generally not recoverable as they point to a malformed file.

In error messages that contain a position value, the position is given in bytes from the start of the `.xff` file.

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
> Ignored tests require the `--all-features` flag as some are feature dependent.
