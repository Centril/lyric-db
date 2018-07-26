use std::convert;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DatabaseError {
    Io(io::Error),
    InvalidAttribute((String, String)),
    InvalidTag(String),
    MissingAttribute((String, String)),
    Empty,
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        use self::DatabaseError;
        match self {
            DatabaseError::Io(e) => e.description(),
            DatabaseError::InvalidAttribute(_) => "Invalid attribute",
            DatabaseError::InvalidTag(_) => "Invalid tag",
            DatabaseError::Empty => "Database file is empty",
            DatabaseError::MissingAttribute(_) => "Missing attribute",
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DatabaseError;
        match self {
            DatabaseError::Io(e) => write!(f, "io error: {}", e.description()),
            DatabaseError::InvalidAttribute((a, tag)) => {
                write!(f, "Invalid attribute {} in tag {}", a, tag)
            }
            DatabaseError::InvalidTag(tag) => write!(f, "Invalid tag {}", tag),
            DatabaseError::Empty => write!(f, "Database file is empty"),
            DatabaseError::MissingAttribute((a, tag)) => {
                write!(f, "Missing attribute {} in {}", a, tag)
            }
        }
    }
}

impl convert::From<io::Error> for DatabaseError {
    fn from(err: io::Error) -> DatabaseError {
        DatabaseError::Io(err)
    }
}
