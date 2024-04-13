use crate::utils::consts::{METADATA_PATH, PROJECT_PATH};
use crate::{Dawproject, MetaData, Project};
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DawprojectWriteError {
    #[error("zip error")]
    ZipError(#[from] zip::result::ZipError),
    #[error("metadata.xml serialize error: {0}")]
    MetadataSerializeError(String),
    #[error("project.xml serialize error: {0}")]
    ProjectSerializeError(String),
    #[error("std io error")]
    StdIoError(#[from] std::io::Error),
}

/// Write `.dawproject` file.
pub struct DawprojectWriter<W: Write + Seek> {
    zip_writer: zip::ZipWriter<W>,
    // audio_files: Vec<AudioFile>,
}

impl<W> DawprojectWriter<W>
where
    W: Write + Seek,
{
    pub fn new(writer: W) -> Result<Self, DawprojectWriteError> {
        let zip_writer = zip::ZipWriter::new(writer);

        Ok(DawprojectWriter { zip_writer })
    }

    pub fn write_dawproject(
        &mut self,
        dawproject: &Dawproject,
    ) -> Result<(), DawprojectWriteError> {
        self.write_metadata(&dawproject.metadata)?;
        self.write_project(&dawproject.project)?;
        Ok(())
    }

    fn write_metadata(&mut self, metadata: &MetaData) -> Result<(), DawprojectWriteError> {
        self.zip_writer
            .start_file(METADATA_PATH, Default::default())
            .map_err(DawprojectWriteError::ZipError)?;
        let xml_str = yaserde::ser::to_string_with_config(
            metadata,
            &yaserde::ser::Config {
                perform_indent: true,
                write_document_declaration: true,
                indent_string: Some("    ".into()),
            },
        )
        .map_err(DawprojectWriteError::MetadataSerializeError)?;
        self.zip_writer
            .write_all(xml_str.as_bytes())
            .map_err(DawprojectWriteError::StdIoError)?;
        // TODO: refactor to this
        // yaserde::ser::serialize_with_writer(
        //     metadata,
        //     self.zip_writer,
        //     &yaserde::ser::Config::default(),
        // )
        // .map_err(DawprojectWriteError::MetadataSerializeError)?;
        Ok(())
    }
    fn write_project(&mut self, project: &Project) -> Result<(), DawprojectWriteError> {
        self.zip_writer
            .start_file(PROJECT_PATH, Default::default())
            .map_err(DawprojectWriteError::ZipError)?;
        let xml_str =
            yaserde::ser::to_string_with_config(project, &yaserde::ser::Config::default())
                .map_err(DawprojectWriteError::ProjectSerializeError)?;
        self.zip_writer
            .write_all(xml_str.as_bytes())
            .map_err(DawprojectWriteError::StdIoError)?;
        // yaserde::ser::serialize_with_writer(
        //     project,
        //     self.zip_writer,
        //     &yaserde::ser::Config::default(),
        // )
        // .map_err(DawprojectWriteError::MetadataSerializeError)?;
        Ok(())
    }

    pub fn raw_copy_file(&mut self, file: zip::read::ZipFile) -> Result<(), DawprojectWriteError> {
        self.zip_writer
            .raw_copy_file(file)
            .map_err(DawprojectWriteError::ZipError)?;
        Ok(())
    }

    pub fn write_file<R: Read>(
        &mut self,
        name: &str,
        buf: &[u8],
    ) -> Result<(), DawprojectWriteError> {
        self.zip_writer
            .start_file(name, Default::default())
            .map_err(DawprojectWriteError::ZipError)?;
        self.zip_writer.write_all(buf)?;
        Ok(())
    }

    pub fn finish(&mut self) -> Result<W, DawprojectWriteError> {
        self.zip_writer
            .finish()
            .map_err(DawprojectWriteError::ZipError)
    }
}

impl DawprojectWriter<BufWriter<File>> {
    /// Create a `.dawproject` file.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, DawprojectWriteError> {
        let f = File::create(path).map_err(DawprojectWriteError::StdIoError)?;
        let writer = BufWriter::new(f);
        Self::new(writer)
    }
}
