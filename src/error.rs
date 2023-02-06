use std::{
    fmt::{Debug, Display},
    io,
};

/// Custom error type
pub enum Error {
    IoError(IoError),
    InvalidInput(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::IoError(io_e) => f.write_fmt(format_args!("{io_e}")),
            Error::InvalidInput(e) => f.write_fmt(format_args!("Invalid input: {e}")),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::IoError(io_e) => f.write_fmt(format_args!("IoError ({io_e:?})")),
            Error::InvalidInput(e) => f.write_fmt(format_args!("InvalidInput ({e:?})")),
        }
    }
}

/// Variant of [Error] representing an error that occurred during file I/O
pub struct IoError {
    pub file: String,
    pub error: io::Error,
}

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("File: \"{}\"; {}", self.file, self.error))
    }
}

impl Debug for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "IoError {{ file: \"{}\", error: {:?} }}",
            self.file, self.error
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_io_error_display() {
        let e = Error::IoError(IoError {
            file: "picture.png".to_string(),
            error: io::Error::from(io::ErrorKind::NotFound),
        });

        assert_eq!(&format!("{e}"), "File: \"picture.png\"; entity not found")
    }

    #[test]
    fn test_invalid_input_error_display() {
        let e = Error::InvalidInput("param1 is None".to_string());

        assert_eq!(&format!("{e}"), "Invalid input: param1 is None")
    }

    #[test]
    fn test_io_error_debug() {
        let e = Error::IoError(IoError {
            file: "picture.png".to_string(),
            error: io::Error::from(io::ErrorKind::NotFound),
        });

        assert_eq!(
            &format!("{e:?}"),
            "IoError (IoError { file: \"picture.png\", error: Kind(NotFound) })"
        )
    }

    #[test]
    fn test_invalid_input_error_debug() {
        let e = Error::InvalidInput("param1 is None".to_string());

        assert_eq!(&format!("{e:?}"), "InvalidInput (\"param1 is None\")")
    }
}
