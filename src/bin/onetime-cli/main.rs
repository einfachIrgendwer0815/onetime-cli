use clap::Parser;

use onetime_cli::args::{Action, Args};

mod error_handler;
use error_handler::ErrorHandler;

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encrypt(e) => {
            onetime_cli::encrypt_file(&e).unwrap_or_exit(1);
            println!("Successfully encrypted {}", e.file);
        }
        Action::Decrypt(d) => {
            onetime_cli::decrypt_file(&d).unwrap_or_exit(1);
            println!("Successfully decrypted {}", d.file);
        }
    }
}
