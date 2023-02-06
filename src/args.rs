pub struct Encrypt {
    /// File to be encrypted
    pub file: String,

    /// Suffix for the name of the first output file
    pub out1_suffix: String,

    /// Suffix for the name of the second output file
    pub out2_suffix: String,

    /// Buffer size in bytes
    pub buffer: u32,

    /// Delete input file after encryption
    pub rm: bool,
}

pub struct Decrypt {
    /// Output file name. This is the name of the decrypted file.
    pub file: String,

    /// Suffix for the name of the first input file
    pub in1_suffix: String,

    /// Suffix for the name of the second input file
    pub in2_suffix: String,

    /// Buffer size in bytes
    pub buffer: u32,

    /// Delete input files after decryption
    pub rm: bool,
}
