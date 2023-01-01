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
}
