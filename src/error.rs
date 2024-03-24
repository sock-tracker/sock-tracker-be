use std::fmt;

/// Main crate Result alias
pub type Result<T> = core::result::Result<T, Error>;

/// Main crate error
#[derive(Debug)]
pub enum Error {
    // Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> core::result::Result<(), fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
