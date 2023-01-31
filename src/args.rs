use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// Encrypt a file
    Encrypt(Encrypt),

    /// Decrypt a file
    Decrypt(Decrypt),
}

#[derive(clap::Args, Debug)]
pub struct Encrypt {
    /// File to be encrypted
    pub file: String,

    /// Suffix for the name of the first output file
    #[arg(
        long,
        value_name = "SUFFIX",
        default_value = "otp.0",
        long_help = "Suffix for the name of the first output file.\n\n\
        Example:\n  \
        Let's say the input file's name is 'secret.txt'. Accordingly,\n  \
        the first output file would be named 'secret.txt.<SUFFIX>'. So,\n  \
        assuming the specified suffix is 'encrypted_1', the resulting full\n  \
        file name would be 'secret.txt.encrypted_1'. Note, that the suffix\n  \
        must NOT start with a dot."
    )]
    pub out1_suffix: String,

    /// Suffix for the name of the second output file
    #[arg(
        long,
        value_name = "SUFFIX",
        default_value = "otp.1",
        long_help = "The same as --out1-suffix, but for the second output file."
    )]
    pub out2_suffix: String,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    pub buffer: u32,

    /// Delete input file after encryption
    #[arg(short, long, default_value = "false")]
    pub rm: bool,
}

#[derive(clap::Args, Debug)]
pub struct Decrypt {
    /// Output file name. This is the name of the decrypted file.
    pub file: String,

    /// Suffix for the name of the first input file
    #[arg(
        long,
        value_name = "SUFFIX",
        default_value = "otp.0",
        long_help = "Suffix for the name of the first input file.\n\n\
        Example:\n  \
        Let's say the output file's name is 'secret.txt'. Accordingly,\n  \
        the first input file's name is expected to be 'secret.txt.<SUFFIX>'.\n  \
        So, assuming the specified suffix is 'encrypted_1', the resulting full\n  \
        file name would be 'secret.txt.encrypted_1'. Note, that the suffix\n  \
        must NOT start with a dot."
    )]
    pub in1_suffix: String,

    /// Suffix for the name of the second input file
    #[arg(
        long,
        value_name = "SUFFIX",
        default_value = "otp.1",
        long_help = "The same as --in1-suffix, but for the second input file."
    )]
    pub in2_suffix: String,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    pub buffer: u32,

    /// Delete input files after decryption
    #[arg(short, long, default_value = "false")]
    pub rm: bool,
}
