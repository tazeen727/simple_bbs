use std::fmt;
use diesel::result::Error as DieselError;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    DatabaseError(DieselError),
    ValidationError {
        msg: String
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match self {
            DatabaseError(e) => Some(e),
            ValidationError {..} => None,
        }
    }
}

impl<'e> From<DieselError> for Error {
    fn from(e: DieselError) -> Error {
        Error::DatabaseError(e)
    }
}
