use std::fs::File;
use std::io::{Read, Write};

use crate::{Error, IoError};

pub enum Mode {
    Open,
    Create,
}

pub fn open_file(path: &str, mode: Mode) -> Result<File, Error> {
    let res = match mode {
        Mode::Open => File::open(path),
        Mode::Create => File::create(path),
    };

    match res {
        Ok(f) => Ok(f),
        Err(e) => Err(Error::IoError(IoError {
            file: format!("{path:?}"),
            error: e,
        })),
    }
}

pub fn read(file: &mut File, buf: &mut [u8]) -> Result<usize, Error> {
    match file.read(buf) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(Error::IoError(IoError {
            file: format!("{file:?}"),
            error: e,
        })),
    }
}

pub fn write(file: &mut File, buf: &[u8]) -> Result<usize, Error> {
    match file.write(buf) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(Error::IoError(IoError {
            file: format!("{file:?}"),
            error: e,
        })),
    }
}

pub fn remove_file(path: &str) -> Result<(), Error> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IoError(IoError {
            file: format!("{path:?}"),
            error: e,
        })),
    }
}
