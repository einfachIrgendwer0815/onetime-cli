use clap::Parser;

use onetime_cli::args::{Action, Args};

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
                Err((f, e)) => {
                    eprintln!("{RED_ERROR_TEXT}: File: '{f}'; {BOLD_START}{e}{STYLE_END}");
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
                Err((f, e)) => {
                    eprintln!("{RED_ERROR_TEXT}: File: '{f}'; {BOLD_START}{e}{STYLE_END}");
                    std::process::exit(1)
                }
            }
        }
    }
}
