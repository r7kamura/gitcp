#[derive(Debug)]
pub enum Error {
    InvalidSourceError { source: String },
    IoError(std::io::Error),
    FsExtraError(fs_extra::error::Error),
    RequestError(reqwest::Error),
    ResponseStatusError(reqwest::StatusCode),
    TemporaryDirectoryError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidSourceError { source } => write!(f, "Invalid source: `{}`", source),
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::FsExtraError(e) => write!(f, "Move items error: {}", e),
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::ResponseStatusError(e) => write!(f, "Response status error: {}", e),
            Error::TemporaryDirectoryError(..) => write!(f, "Tempfile creation error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidSourceError { .. } => None,
            Error::IoError(e) => Some(e),
            Error::FsExtraError(e) => Some(e),
            Error::RequestError(e) => Some(e),
            Error::ResponseStatusError(_) => None,
            Error::TemporaryDirectoryError(e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(error: fs_extra::error::Error) -> Self {
        Error::FsExtraError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}
