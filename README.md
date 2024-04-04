# dawproject

[![Crate](https://img.shields.io/crates/v/dawproject.svg)](https://crates.io/crates/dawproject)
[![API](https://docs.rs/dawproject/badge.svg)](https://docs.rs/dawproject)

A port of the [DAWproject](https://github.com/bitwig/dawproject) in Rust.

# Usage

```rust
use dawproject::DawprojectReader;
use std::fs::File;

// read dawproject file
let file = File::open("tests/data/canon.dawproject").unwrap();
let mut reader =
    DawprojectReader::new(file).unwrap();
reader.read_metadata().unwrap();
reader.read_project().unwrap();
// use it wherever you want
let metadata = reader.metadata();
let project = reader.project();
```
