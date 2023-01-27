use std::fs::File;
use std::io::{Read, Write};


pub enum Mode {
    Open,
    Create,
}


pub fn open_file(path: &str, mode: Mode) -> Result<File, (String, String)> {
    let res = match mode {
        Mode::Open => {
            File::open(path)
        }
        Mode::Create => {
            File::create(path)
        }
    };

    match res {
        Ok(f) => { Ok(f) }
        Err(e) => {
            Err((format!("{path:?}"), e.to_string()))
        }
    }
}


pub fn read(file: &mut File, buf: &mut [u8]) -> Result<usize, (String, String)> {
    match file.read(buf) {
        Ok(bytes) => { Ok(bytes) }
        Err(e) => {
            Err((format!("{file:?}"), e.to_string()))
        }
    }
}


pub fn write(file: &mut File, buf: &[u8]) -> Result<usize, (String, String)> {
    match file.write(buf) {
        Ok(bytes) => { Ok(bytes) }
        Err(e) => {
            Err((format!("{file:?}"), e.to_string()))
        }
    }
}


pub fn remove_file(path: &str) -> Result<(), (String, String)> {
    match std::fs::remove_file(path) {
        Ok(_) => { Ok(()) },
        Err(e) => {
            Err((format!("{path:?}"), e.to_string()))
        }
    }
}