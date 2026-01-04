//! `error` encapsulates error and result types used by the application.

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse,
    Template(askama::Error),
    Io(std::io::Error),
    Wiki,
    Other(String),
}

impl From<gray_matter::Error> for Error {
    fn from(_err: gray_matter::Error) -> Self {
        Self::Parse
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Self::Template(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
