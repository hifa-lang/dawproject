# dawproject

[![Crate](https://img.shields.io/crates/v/dawproject.svg)](https://crates.io/crates/dawproject)
[![API](https://docs.rs/dawproject/badge.svg)](https://docs.rs/dawproject)

A port of [DAWproject](https://github.com/bitwig/dawproject) in Rust.

## Usage

You can read and write `.dawproject` file.

```rust
use dawproject::{DawprojectReader, DawprojectWriter};

// read dawproject file
let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();
reader.read_dawproject().unwrap();
// use it wherever you want
let dawproject = reader.build_dawproject().unwrap();
// Write
let mut writer = DawprojectWriter::create("tests/data/copied_canon.dawproject").unwrap();
writer.write_dawproject(&dawproject).unwrap();
```

## install

```bash
git submodule update --init
```