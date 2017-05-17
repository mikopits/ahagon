use std::error::Error as StdError;
use std::fmt;

use github_rs::github::GithubError;

#[derive(Debug)]
pub enum Error {
    Github(GithubError),
    Io(::std::io::Error),
    Toml(::toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Github(ref e) => fmt::Display::fmt(e, f),
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::Toml(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Github(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Toml(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Github(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Toml(ref e) => Some(e),
        }
    }
}

impl From<GithubError> for Error {
    fn from(err: GithubError) -> Error {
        Error::Github(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<::toml::de::Error> for Error {
    fn from(err: ::toml::de::Error) -> Error {
        Error::Toml(err)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
