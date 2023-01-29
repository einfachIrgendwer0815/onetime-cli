use clap::Parser;

use onetime_cli::args::{Action, Args};
use onetime_cli::Error;

pub const RED_ERROR_TEXT: &str = "\x1b[1;91mError\x1b[0m";
pub const BOLD_START: &str = "\x1b[01m";
pub const STYLE_END: &str = "\x1b[0m";

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encrypt(mut e) => {
            e.set_out_files();

            match onetime_cli::encrypt_file(&e) {
                Ok(_) => {
                    println!("Successfully encrypted {}", &e.file);
                }
                Err(e) => {
                    match e {
                        Error::IoError(io_e) => {
                            eprintln!(
                                "{RED_ERROR_TEXT}: File: {0}; {BOLD_START}{1}{STYLE_END}",
                                io_e.file, io_e.error
                            );
                        }
                        Error::InvalidInput(s) => {
                            eprintln!("{RED_ERROR_TEXT}: {s}");
                        }
                    }
                    std::process::exit(1)
                }
            }
        }
        Action::Decrypt(mut d) => {
            d.set_in_files();

            match onetime_cli::decrypt_file(&d) {
                Ok(_) => {
                    println!("Successfully decrypted {}", &d.file);
                }
                Err(e) => {
                    match e {
                        Error::IoError(io_e) => {
                            eprintln!(
                                "{RED_ERROR_TEXT}: File: {0}; {BOLD_START}{1}{STYLE_END}",
                                io_e.file, io_e.error
                            );
                        }
                        Error::InvalidInput(s) => {
                            eprintln!("{RED_ERROR_TEXT}: {s}");
                        }
                    }
                    std::process::exit(1)
                }
            }
        }
    }
}
