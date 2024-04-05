#![doc = include_str!("../README.md")]

mod read;
mod repositories;

pub use read::{DawprojectReadError, DawprojectReader};
/// Metadata of the `metadata.xml` file.
pub use repositories::metadata::MetaData;
/// Project of the `project.xml` file.
pub use repositories::project::Project;
pub use repositories::*;
