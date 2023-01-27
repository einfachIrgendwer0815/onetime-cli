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

    /// Output file one. Defaults to "<FILE>.otp.0"
    #[arg(short = '1', long)]
    pub out1: Option<String>,

    /// Output file two. Defaults to "<FILE>.otp.1"
    #[arg(short = '2', long)]
    pub out2: Option<String>,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    pub buffer: u32,

    /// Delete input file after encryption
    #[arg(short, long, default_value = "false")]
    pub rm: bool,
}

impl Encrypt {
    /// Fills `self.out1` and `self.out2` with default values
    /// if they are `None`. So afterwards, `self.out1` and `self.out2`
    /// are always `Some`.
    pub fn set_out_files(&mut self) {
        if self.out1.is_none() {
            self.out1 = Some(self.file.clone() + ".otp.0");
        }
        if self.out2.is_none() {
            self.out2 = Some(self.file.clone() + ".otp.1")
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct Decrypt {
    /// Output file name. This is the name of the decrypted file.
    pub file: String,

    /// Input file one. Defaults to "<FILE>.otp.0"
    #[arg(short = '1', long)]
    pub in1: Option<String>,

    /// Input file one. Defaults to "<FILE>.otp.1"
    #[arg(short = '2', long)]
    pub in2: Option<String>,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    pub buffer: u32,

    /// Delete input files after decryption
    #[arg(short, long, default_value = "false")]
    pub rm: bool,
}

impl Decrypt {
    /// Fills `self.in1` and `self.in2` with default values
    /// if they are `None`. So afterwards, `self.in1` and `self.in2`
    /// are always `Some`.
    pub fn set_in_files(&mut self) {
        if self.in1.is_none() {
            self.in1 = Some(self.file.clone() + ".otp.0");
        }
        if self.in2.is_none() {
            self.in2 = Some(self.file.clone() + ".otp.1")
        }
    }
}
