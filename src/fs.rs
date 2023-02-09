use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::{Error, IoError};

pub enum Mode {
    Open,
    Create,
}

pub fn open_file(path: &Path, mode: Mode) -> Result<File, Error> {
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

pub fn remove_file(path: &Path) -> Result<(), Error> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IoError(IoError {
            file: format!("{path:?}"),
            error: e,
        })),
    }
}

pub fn extend_extension(path: &Path, extension: &str) -> PathBuf {
    let mut path = path.to_owned();

    let extension = match path.extension() {
        None => OsString::from(extension),
        Some(e) => {
            let mut ext = e.to_owned();

            ext.push(".");
            ext.push(extension);

            ext
        }
    };

    path.set_extension(extension);

    path
}
