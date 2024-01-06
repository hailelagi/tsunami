use std::error::Error as StdError;
use std::fmt;
// use std::io;
use crate::value::Term;
use std::result;
use std::str;

/// A crate private constructor for `Error`.
pub(crate) fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

/// A type alias for `Result<T, ets::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when using ETS.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of an error.
#[derive(Debug)]
pub enum ErrorKind {
    Argument {
        form: &'static str,
        reason: &'static str,
        value: Term,
    },
    VariableUnbound {
        n: usize,
    },
    WrongDialect {
        form: &'static str,
    },
    CalledInGuard {
        form: &'static str,
    },
    Generic(String),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

// impl From<io::Error> for Error {
//     fn from(err: io::Error) -> Error {
//         new_error(ErrorKind::Io(err))
//     }
// }

// impl From<Error> for io::Error {
//     fn from(err: Error) -> io::Error {
//         io::Error::new(io::ErrorKind::Other, err)
//     }
// }

impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Argument { reason, .. } => reason,
            ErrorKind::WrongDialect { .. } => "wrong dialect",
            ErrorKind::Generic(..) => "generic",
            // ErrorKind::Io(ref err) => err.description(),
            // ErrorKind::Utf8 { ref err, .. } => err.description(),
            // ErrorKind::UnequalLengths { .. } => "record of different length found",
            // ErrorKind::Seek => "headers unavailable on seeked CSV reader",
            // ErrorKind::Serialize(ref err) => err,
            // ErrorKind::Deserialize { ref err, .. } => err.description(),
            _ => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self.0 {
            ErrorKind::Argument { .. } => None,
            ErrorKind::WrongDialect { .. } => None,
            ErrorKind::Generic(..) => None,
            // ErrorKind::Io(ref err) => Some(err),
            // ErrorKind::Utf8 { ref err, .. } => Some(err),
            // ErrorKind::UnequalLengths { .. } => None,
            // ErrorKind::Seek => None,
            // ErrorKind::Serialize(_) => None,
            // ErrorKind::Deserialize { ref err, .. } => Some(err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Argument {
                form,
                reason,
                value,
            } => write!(f, "Special form '{}' called {} in {}.", form, reason, value),
            ErrorKind::WrongDialect { form } => {
                write!(f, "Special form '{}' called in wrong dialect.", form)
            }
            ErrorKind::CalledInGuard { form } => {
                write!(f, "Special form '{}' called in guard.", form)
            }
            ErrorKind::Generic(ref reason) => write!(f, "{}", reason),
            _ => unreachable!(),
        }
    }
}
