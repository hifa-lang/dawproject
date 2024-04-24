#![doc = include_str!("../README.md")]

mod models;
mod read;
mod repositories;
pub mod utils;
mod write;

/// This structure contains the metadata and project of DAWproject.
pub use models::{Dawproject, DawprojectWithZip};
pub use read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use repositories::project::Project;
pub use repositories::*;
pub use write::{DawprojectWriteError, DawprojectWriter};
