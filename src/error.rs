use std::ffi;
use std::fmt;
use std::fmt::{Debug, Display};

/// UInput error.
#[derive(Debug)]
pub enum Error {
    /// System errors.
    Nix(nix::Error),

    /// Errors with internal nulls in names.
    Nul(ffi::NulError),

    #[cfg(feature = "udev")]
    /// Errors coming from udev.
    Udev(udev::Error),

    /// Errors from io
    IoError(std::io::Error),

    /// The uinput file could not be found.
    NotFound,
}

impl From<ffi::NulError> for Error {
    fn from(value: ffi::NulError) -> Self {
        Error::Nul(value)
    }
}

impl From<nix::Error> for Error {
    fn from(value: nix::Error) -> Self {
        Error::Nix(value)
    }
}

#[cfg(feature = "udev")]
impl From<udev::Error> for Error {
    fn from(value: udev::Error) -> Self {
        Error::Udev(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Nix(err) => Display::fmt(err, f),

            Error::Nul(err) => Display::fmt(err, f),

            #[cfg(feature = "udev")]
            Error::Udev(err) => Display::fmt(err, f),

            Error::IoError(err) => Display::fmt(err, f),

            Error::NotFound => f.write_str("Device not found."),
        }
    }
}

impl std::error::Error for Error {}
