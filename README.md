# Nabu

> [!note]
> This is a hobby project. It is not intended nor ready to be used in production.

Nabu is a rust library for reading and writing `.xff` files.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` files are my attempt at a general purpose file format, with Nabu as the translation layer, giving the opportunity for downstream projects to create their own files suiting their needs.
I am also trying out the `Feature` flags for the first time, to extend the usability of `.xff` files for a wide range of use-cases without needing to roll a bespoke library.

As with all my projects, this documentation contains everything you never wanted to know about `.xff` files, Nabu and how to work with them.
Just read the examples below and have fun.

On a technical note, the byte-structure of `.xff` does not change, however a `.xff` file written with the default `serde` module will be different from a `.xff` file written with `key_value_store` in the way the data inside is structured and interpreted.
This means that any `.xff` file in existence can be read using the `serde` module without data loss, but there may be some loss of interpreted data (the logging wizard for example builds blocks of valid `xff` and appends them into the same file. Reading it with the `serde` module would return all values in it in order, however the logs contained would need to be decoded from that.).

This modularity is important, as it allows `.xff` files to be read and written in a way that is compatible with different use-cases. I found the non-guaranteed usability of the same extension in different use-cases shockingly prevalent, especially with older standards like CSV.
I do also recognise that this approach of splintering `.xff` into several extensions more or less exclusive to each other may not be the best idea, I see it as necessary for the implementation of the vision I have for `.xff` and honestly have no better Idea and expect an absolute maximum of one user ever, me.

## 1. Motivation
After finishing [Mawu](https://github.com/Xqhare/mawu), I wanted to dive deeper into file structures and working with bytes directly, instead of `&str` and later `chars` like in Mawu. Around this time I also had my first deep dive on ASCII after rewatching "The Martian" and thus decided on making my own file format.
I wrote v0 of the `.xff` specification in just a few days, and then started working on the implementation.
This library is also a major part of my own tech stack.
As `xff` is meant to be a jack of all trades, it is important that it can be used in a wide range of use-cases.

## 2. Naming
This library's namesake is the ancient Babylonian god Nabu, the god of literacy, rational arts and scribes.
As the inventor of writing, Nabu is a fitting namesake for a tool designed to create and interpret a new form of written data.

## 3. Contents
- [1. Motivation](#motivation)
- [2. Naming](#naming)
- [3. Contents](#contents)
- [4. Roadmap](#roadmap)
- [5. Implemented Features](#implemented-features)
- [6. `.xff` specifications](#.xff-specifications)
- [7. Usage](#usage)
    - [7.1. Importing](#importing)
    - [7.2. Default Modules](#default-modules)
        - [7.2.1. Usage of serde](#usage-of-serde)
    - [7.3. XffValue](#xffvalue)
    - [7.4. Feature Flags](#feature-flags)
        - [7.4.1. Logging Wizard](#logging-wizard)
        - [7.4.2. Config Wizard](#config-wizard)
        - [7.4.3. Key Value Store](#key-value-store)
            - [7.4.3.1. Key Value Core](#key-value-core)
                - [7.4.3.1.1. Key Value Core Usage](#key-value-core-usage)
            - [7.4.3.2. Key Value Store](#key-value-store)
                - [7.4.3.2.1. Key Value Store Usage](#key-value-store-usage)

## 4. Roadmap
- Configuration wizard
    - For writing and reading `.xff` files containing all data needed for a project to configure itself


## 5. Implemented Features
- Key-value store
    - For working with persistent data stored in `.xff` files as simple key-value pairs
- Logging wizard
    - For writing and reading `.xff` files containing all data needed for a project to log its behaviour

## 6. `.xff` specifications
All specifications are in the `specifications` directory. 
V0 can be found [here](specifications/v0.md).

## 7. Usage
All paths shown in example code are valid paths inside this repository.

### 7.1. Importing
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

### 7.2. Default Modules
The only module active by default is `serde`, which is used for serialization and deserialization of `.xff` files.
It represents the core of Nabu and by itself is very bare-bones.

This means that using only this module is enough to read and write `.xff` files, and implement any further functionality you want.

#### 7.2.1. Usage of serde
No matter what the extension of the path you provide, it will be converted to .xff
For example, if you provide "example.txt", it will be converted to "example.xff"
This behaviour only ever changes when a function expects a `Path` instead of a `&str` or similar

```rust
use nabu::serde::{read, write, remove_file};
use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};
let path = "xff-example-data/serde-example.txt";
let path_2 = "xff-example-data/serde-example.xff";
let data = {
    vec![
        XffValue::String("hello mom".to_string()),
    ]
};
let tmp = write(path, data.clone());
assert!(tmp.is_ok());
let tmp_2 = read(path_2);
assert!(tmp_2.is_ok());
let ok = tmp_2.unwrap();
assert_eq!(ok[0], data[0]);
remove_file(path_2).unwrap();
```

### 7.3. XffValue
XffValue is the main data type used in Nabu to store and manipulate data.
Basic types are `String`, `Number`, `CommandCharacter` and `Data`, all representing data stored in `.xff` files.
There is also `ArrayCmdChar` which is an array of `CommandCharacter`, only used in a few features.
```rust
use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};
let data = XffValue::String("hello mom".to_string());
let data_2 = XffValue::Number(Number::from(-42));
let data_3 = XffValue::CommandCharacter(CommandCharacter::LineFeed);
let data_4 = XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
let data_5 = XffValue::ArrayCmdChar(vec![CommandCharacter::Escape, CommandCharacter::StartOfText]);
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

let command_character_array_0 = XffValue::from(vec![CommandCharacter::Escape, CommandCharacter::StartOfText]);


let string_2 = XffValue::String("hello mom".to_string());

let number_9 = XffValue::Number(Number::from(-42));
let number_10 = XffValue::Number(Number::from(42));
let number_11 = XffValue::Number(Number::from(42.2));
let number_12 = XffValue::Number(Number::from(-42.2));
let number_13 = XffValue::Number(Number::from(f64::MAX));
let number_14 = XffValue::Number(Number::from(usize::MAX));
let number_15 = XffValue::Number(Number::from(isize::MAX));
let number_16 = XffValue::Number(Number::from(u8::MAX));
let number_17 = XffValue::Number(Number::from(i8::MAX));


let command_character_0 = XffValue::CommandCharacter(CommandCharacter::from(26));
let command_character_1 = XffValue::CommandCharacter(CommandCharacter::from(27));
let command_character_2 = XffValue::CommandCharacter(CommandCharacter::from(28));
let command_character_3 = XffValue::CommandCharacter(CommandCharacter::from(29));
let command_character_4 = XffValue::CommandCharacter(CommandCharacter::from(30));
let command_character_5 = XffValue::CommandCharacter(CommandCharacter::from(31));

let data_2 = XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
let data_3 = XffValue::Data(Data::from(vec![152, 142, 202, 33, 54, 5, 86, 197, 38, 209]));


let number_18 = XffValue::Number(Number::Unsigned(42));
let number_19 = XffValue::Number(Number::Integer(-42));
let number_20 = XffValue::Number(Number::Float(42.2));

let command_character_6 = XffValue::CommandCharacter(CommandCharacter::LineFeed);

let command_character_array_1 = XffValue::ArrayCmdChar(vec![CommandCharacter::Escape, CommandCharacter::StartOfText]);
```

This list contains all types that can be converted to `XffValue` by using `XffValue::from()`.
- `XffValue` 
    - `&str`, `String`
    - `usize`, `u8`, `u16`, `u32`, `u64`
    - `isize`, `i8`, `i16`, `i32`, `i64`
    - `f32`, `f64`
    - `Vec<u8>`, `Vec<CommandCharacter>`

- `XffValue::String`
- `XffValue::Number`
    - `usize`, `u8`, `u16`, `u32`, `u64`
    - `isize`, `i8`, `i16`, `i32`, `i64`
    - `f32`, `f64`
- `XffValue::CommandCharacter`
    - `u8` meaning the ASCII encoded byte of a command character
- `XffValue::Data`
    - `Vec<u8>` meaning the byte-array of the data to be stored
- `XffValue::ArrayCmdChar`

The `XffValue` also has several associated functions.
- `XffValue::String`
- `XffValue::Number`
    - `as_u8()` converts the number into an ASCII encoded byte-stream
- `XffValue::CommandCharacter`
    - `as_u8()` converts the command character into an ASCII encoded byte
- `XffValue::Data`
- `XffValue::ArrayCmdChar`

### 7.4 Feature Flags
Nabu provides an ever-increasing number of feature flags that can be enabled to extend its capabilities.
These flags have been carefully designed for full interoperability. Meaning no mutually exclusive features are present at all.

Some features have more than one flag.

All possible flags are:
```toml
[dependencies]
nabu = { git = "https://github.com/Xqhare/nabu", features = ["logging_wizard", "config_wizard", "key_value_store", "key_value_core"] }
```

> [!note]
> Most features rely on their own `.xff` extension, meaning that `.xff` files created using the Key Value Store would error if loaded as a config Wizard. 

#### 7.4.1. Logging Wizard
Nabu provides a logging feature that can be enabled by the `logging_wizard` feature flag.

This feature can be used one of two ways.
Either by using `LoggingWizard::new()`, or by using `LoggingWizard::from_file`.

Using `LoggingWizard::new()` is recommended for general logging use, as it is designed to minimise disk operations and free memory after writing to disk.
`LoggingWizard::from_file()` is recommended for interpretation of `.xff` log files. The entire file is loaded into memory as entries inside a `LoggingWizard` struct and can then be used for processing.

Please note that `LoggingWizard::from_file()` and `LoggingWizard::new()` do not save their state to disk automatically. To save the state, use `.save()` or add a log using `.add_log_and_save()`. Using the latter method, the created log will be written to disk immediately, with the memory being freed right after. This method is recommended, especially if you do not expect a large amount of logs to be generated, and don't want to deal with creating a saving logic.
To help to create a saving logic, the `LoggingWizard` struct contains a `logs_len` field that contains the amount of logs in the `logs` field and could be used to check if the `LoggingWizard` has reached a specific length to then call `.save()`.

##### 7.4.1.1. Structure
The `LoggingWizard` struct holds all the logs that have been added to it. The logs it holds are of type `Log`, these hold the data points of the logs as `LogData`.
The structure, as well as all functions are listed below.

- `LoggingWizard`
    - `logs: Vec<Log>`
    - `logs_len: usize`
    - `save()`
    - `add_log(log: Log)`
    - `add_log_and_save(log: Log)`
- `Log`
    - `logs: Vec<LogData>`
    - `new()` / `default()`
    - `add_log_data(log_data: LogData)`
- `LogData`
    - `name: String`
    - `value: XffValue`
    - `optional_metadata: BTreeMap<String, String>`
    - `new(name: String, value: XffValue, optional_metadata: Option<BTreeMap<String, String>>)` / `create(name: String, value: XffValue, optional_metadata: Option<BTreeMap<String, String>>)`
    - `add_optional_metadata(key: String, value: String)`

##### 7.4.1.2. `from_file()` Usage
```rust
use nabu::{logging_wizard::{LoggingWizard, Log}, xff::value::XffValue};
        let wizard = LoggingWizard::from_file("xff-example-data/logging-wizard-main-example.xff");
        assert!(wizard.is_ok());
        for log in wizard.unwrap().logs {
            // Do stuff
            println!("{:?}", log);
        }
```

##### 7.4.1.3. `new()` Usage
```rust
use nabu::{logging_wizard::{LoggingWizard, Log, LogData}, xff::value::XffValue};
        let mut wizard = LoggingWizard::new("xff-example-data/logging-wizard-main-example.xff");
        let mut log = Log::new();
        log.add_log_data(LogData::new("data point name", XffValue::String("value".to_string()), None));
        wizard.add_log(log);
        let out = wizard.save();
        assert!(out.is_ok());

```

Alternatively, using the `add_log_and_save` method:
```rust
use nabu::{logging_wizard::{LoggingWizard, Log, LogData}, xff::value::XffValue};
        let mut wizard = LoggingWizard::new("xff-example-data/logging-wizard-main-example.xff");
        let mut log = Log::new();
        log.add_log_data(LogData::new("data point name", XffValue::String("value".to_string()), None));
        wizard.add_log_and_save(log);
```

#### 7.4.2. Config Wizard

#### 7.4.3. Key Value Store
The key value store is a simple key value store for in place storage and manipulation of data.
It consists of two feature flags, `key_value_core` and `key_value_store`.
`key_value_core` provides the core functionality and can be enabled separately from `key_value_store`, however `key_value_store` can not be enabled separately from `key_value_core` as it depends on it.

Using `key_value_store` is recommended, especially if you want to store it as a `.xff` file.

##### 7.4.3.1. Key Value Core
`key_value_core` is a small feature flag containing `read_core()` and `write_core()`.

`read_core()` takes a path and returns a `BTreeMap<String, XffValue>`.
A `BTreeMap<String, XffValue>` can be saved to a `.xff` file using `write_core()`.
All interaction with the `BTreeMap` and its values is handled by the caller.

###### 7.4.3.1.1. Key Value Core Usage
```rust
use nabu::features::key_value::core::{read_core, write_core};
use nabu::xff::value::{XffValue, Number, Data};
use std::{collections::BTreeMap, path::Path};

# #[cfg(feature = "key_value_core")]
# fn main() {
let mut map: BTreeMap<String, XffValue> = Default::default();
map.insert("key0".to_string(), XffValue::String("Hello Mom!".to_string()));
map.insert("key1".to_string(), XffValue::Number(Number::from(-42)));
map.insert("key2".to_string(), XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
let write = write_core(&Path::new("xff-example-data/xff-core-example-file.xff"), map.clone());
assert!(write.is_ok());
let read = read_core(&Path::new("xff-example-data/xff-core-example-file.xff"));
assert!(read.is_ok());
# }
```

##### 7.4.3.2. Key Value Store
`key_value_store` is a feature flag containing `NabuDB`, a struct for in place storage and manipulation of data.
`NabuDB` is a one-stop solution for key value store holding arbitrary data.
While it is possible to simply save ASCII text and control characters, it is also possible to store arbitrary data like a picture.
Please note that in `V0` no metadata is stored with the data in the `.xff` file directly, only the data itself.
Storing this metadata is left to the user.

All possible interactions with it are shown in the example code below.

###### 7.4.3.2.1. Key Value Store Usage
```rust
use std::collections::BTreeMap;
use nabu::key_value_store::new_nabudb;
use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};

# #[cfg(feature = "key_value_store")]
# fn main() {
let path = "xff-example-data/nabuDB_main_example.xff";
let mut db = new_nabudb(path).unwrap();
db.clear();

db.insert("key0".to_string(), XffValue::String("value0".to_string()));
db.insert("key1".to_string(), XffValue::Number(Number::from(-42)));
db.insert("key2".to_string(), XffValue::CommandCharacter(CommandCharacter::LineFeed));
db.insert("key3".to_string(), XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));

db.save();

let read = new_nabudb(path).unwrap();
assert_eq!(read.get("key0").unwrap(), db.get("key0").unwrap());
assert_eq!(read.get("key1").unwrap(), db.get("key1").unwrap());
assert_eq!(read.get("key2").unwrap(), db.get("key2").unwrap());
assert_eq!(read.get("key3").unwrap(), db.get("key3").unwrap());

db.set_auto_save(true);
db.insert("key4".to_string(), XffValue::String("value4".to_string()));
let read = new_nabudb(path).unwrap();
assert_eq!(db.get("key4").unwrap(), read.get("key4").unwrap());

if db.contains_key("key4") {
    let _ = db.remove("key4");
}

println!("All keys:");
for key in db.keys() {
    println!("{}", key);
}

let map: BTreeMap<String, XffValue> = db.to_map();

let get_key_0 = db.get("key0");
assert_eq!(get_key_0.unwrap(), &XffValue::String("value0".to_string()));

let (key, value) = db.get_key_value("key0").unwrap();
assert_eq!(key, &"key0".to_string());
assert_eq!(value, &XffValue::String("value0".to_string()));

println!("All key-values:");
for (key, value) in db.iter() {
    println!("{}: {:?}", key, value);
}

let len = db.len();

assert_eq!(db.get("key0").unwrap(), &XffValue::String("value0".to_string()));
assert_eq!(db.get("key1").unwrap(), &XffValue::Number(Number::from(-42)));
assert_eq!(db.get("key2").unwrap(), &XffValue::CommandCharacter(CommandCharacter::LineFeed));
assert_eq!(db.get("key3").unwrap(), &XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
assert_eq!(len, 4);

db.clear();
let read = new_nabudb(path).unwrap();
assert!(read.len() == 0);
# }
```
