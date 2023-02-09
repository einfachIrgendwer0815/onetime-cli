use std::path::PathBuf;

/// Configuration passed to [encrypt_file()](crate::encrypt_file) or [decrypt_file()](crate::decrypt_file)
pub struct Config {
    /// File to be encrypted
    pub file: PathBuf,

    /// Suffix for the name of the first input/output file
    pub suffix1: String,

    /// Suffix for the name of the second input/output file
    pub suffix2: String,

    /// Buffer size in bytes
    pub buffer: u32,

    /// Delete input file after encryption
    pub rm: bool,
}

impl Config {
    pub fn new(file: &str) -> Self {
        Self {
            file: PathBuf::from(file),
            ..Config::default()
        }
    }

    pub fn new_with_suffixes(file: &str, suffix1: &str, suffix2: &str) -> Self {
        Self {
            file: PathBuf::from(file),
            suffix1: suffix1.to_string(),
            suffix2: suffix2.to_string(),
            ..Config::default()
        }
    }

    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            suffix1: "otp.0".to_string(),
            suffix2: "otp.1".to_string(),
            buffer: 1048576,
            rm: false,
        }
    }
}
