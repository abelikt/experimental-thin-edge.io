use std::convert::TryInto;
use std::path::PathBuf;

/// Represents a path to a file or directory.
///
/// We need this newtype in order to implement `TryInto<String>`.
/// `PathBuf` does not implement `TryInto<String>`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct FilePath(PathBuf);

#[derive(thiserror::Error, Debug)]
#[error("FilePath to String conversion failed: {0:?}")]
pub struct FilePathToStringConversionFailure(std::ffi::OsString);

impl<T> From<T> for FilePath
where
    PathBuf: From<T>,
{
    fn from(input: T) -> Self {
        Self(PathBuf::from(input))
    }
}

impl Into<PathBuf> for FilePath {
    fn into(self) -> PathBuf {
        self.0
    }
}

impl TryInto<String> for FilePath {
    type Error = FilePathToStringConversionFailure;

    fn try_into(self) -> Result<String, FilePathToStringConversionFailure> {
        self.0
            .into_os_string()
            .into_string()
            .map_err(FilePathToStringConversionFailure)
    }
}