use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action, 
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Encrypt a file
    Encrypt(Encrypt),

    /// Decrypt a file
    Decrypt(Decrypt),
}

#[derive(clap::Args, Debug)]
struct Encrypt {
    /// File to be encrypted
    file: String,

    /// Output file one. Defaults to "<FILE>.otp.0"
    #[arg(short = '1', long)]
    out1: Option<String>,
    
    /// Output file two. Defaults to "<FILE>.otp.1"
    #[arg(short = '2', long)]
    out2: Option<String>,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    buffer: u32,
}

#[derive(clap::Args, Debug)]
struct Decrypt {
    /// Output file name. This is the name of the decrypted file.
    file: String,

    /// Input file one. Defaults to "<FILE>.otp.0"
    #[arg(short = '1', long)]
    in1: Option<String>, 
    
    /// Input file one. Defaults to "<FILE>.otp.1"
    #[arg(short = '2', long)]
    in2: Option<String>,

    /// Buffer size in bytes
    #[arg(long, default_value = "1048576")]
    buffer: u32,
}


fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
