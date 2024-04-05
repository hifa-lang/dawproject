# dawproject

[![Crate](https://img.shields.io/crates/v/dawproject.svg)](https://crates.io/crates/dawproject)
[![API](https://docs.rs/dawproject/badge.svg)](https://docs.rs/dawproject)

A port of the [DAWproject](https://github.com/bitwig/dawproject) in Rust.

# Usage

```rust
use dawproject::DawprojectReader;

// read dawproject file
let mut reader = DawprojectReader::open("tests/data/canon.dawproject").unwrap();
reader.read_metadata().unwrap();
reader.read_project().unwrap();
// use it wherever you want
let metadata = reader.metadata();
let project = reader.project();
```
