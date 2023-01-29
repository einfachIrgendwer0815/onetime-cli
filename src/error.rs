use std::io;

pub enum Error {
    IoError(IoError),
    InvalidInput(String),
}

pub struct IoError {
    pub file: String,
    pub error: io::Error,
}
