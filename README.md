# dawproject

[![Crate](https://img.shields.io/crates/v/dawproject.svg)](https://crates.io/crates/dawproject)
[![API](https://docs.rs/dawproject/badge.svg)](https://docs.rs/dawproject)

Dawproject is a format for exchanging user data between digital audio workstations (DAWs).
This project is port of [DAWproject](https://github.com/bitwig/dawproject) in Rust.

## Usage

You can easily read and write `.dawproject` files.

```rust
use dawproject::{DawprojectReader, DawprojectWriter};

// read dawproject file
let mut reader = DawprojectReader::open("assets/tests/canon.dawproject").unwrap();
reader.read_dawproject().unwrap();
// use it wherever you want
let dawproject = reader.build_dawproject().unwrap();
// Write
let mut writer = DawprojectWriter::create("assets/tests/copied_canon.dawproject").unwrap();
writer.write_dawproject(&dawproject).unwrap();
```

## Documentations

- [DAWproject](https://github.com/bitwig/dawproject)
  - Bitwig DAWproject repository
- [DAWPROJECT XML Reference](https://htmlpreview.github.io/?https://github.com/bitwig/dawproject/blob/main/Reference.html)
  - Bitwig DawProject XML Reference
- [Project Document](https://docs.rs/dawproject)
  - This project document

## Build Source

```bash
git submodule update --init
```
