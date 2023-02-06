use onetime_cli::Error;

const RED_ERROR_TEXT: &str = "\x1b[1;91mError\x1b[0m";
const BOLD_START: &str = "\x1b[01m";
const STYLE_END: &str = "\x1b[0m";

pub trait ErrorHandler<T> {
    fn unwrap_or_exit(self, code: i32) -> T;
}

impl<T> ErrorHandler<T> for Result<T, Error> {
    fn unwrap_or_exit(self, code: i32) -> T {
        match self {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}", err_to_text(e));
                std::process::exit(code);
            }
        }
    }
}

pub fn err_to_text(e: Error) -> String {
    match e {
        Error::IoError(io_e) => {
            format!(
                "{RED_ERROR_TEXT}: File: {0}; {BOLD_START}{1}{STYLE_END}",
                io_e.file, io_e.error
            )
        }
        Error::InvalidInput(s) => {
            format!("{RED_ERROR_TEXT}: {s}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use onetime_cli::IoError;
    use std::io;

    #[test]
    fn test_print_err_with_io_error() {
        let e = Error::IoError(IoError {
            file: "picture.png".to_string(),
            error: io::Error::from(io::ErrorKind::NotFound),
        });

        assert_eq!(
            err_to_text(e),
            format!("{RED_ERROR_TEXT}: File: picture.png; {BOLD_START}entity not found{STYLE_END}"),
        );
    }

    #[test]
    fn test_print_err_with_input_error() {
        let e = Error::InvalidInput("param1 is None".to_string());

        assert_eq!(err_to_text(e), format!("{RED_ERROR_TEXT}: param1 is None"));
    }
}
