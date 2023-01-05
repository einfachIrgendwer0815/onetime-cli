use clap::Parser;

use onetime_cli::args::{Action, Args};


pub const RED_ERROR_TEXT: &str = "\x1b[1;91mError\x1b[0m";
pub const BOLD_START: &str = "\x1b[01m";
pub const STYLE_END: &str = "\x1b[0m";


fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encrypt(mut e) => {
            if e.out1.is_none() {
                e.out1 = Some(e.file.clone() + ".otp.0");
            }
            if e.out2.is_none() {
                e.out2 = Some(e.file.clone() + ".otp.1")
            }
            
            match onetime_cli::encrypt(&e) {
                Ok(_) => {
                    println!("Successfully encrypted {}", &e.file);
                }
                Err((f, e)) => {
                    eprintln!("{}: File: '{}'; {}{}{}", RED_ERROR_TEXT, f, BOLD_START, e, STYLE_END);
                }
            }
        }
        Action::Decrypt(mut d) => {
            if d.in1.is_none() {
                d.in1 = Some(d.file.clone() + ".otp.0");
            }
            if d.in2.is_none() {
                d.in2 = Some(d.file.clone() + ".otp.1");
            }

            match onetime_cli::decrypt(&d) {
                Ok(_) => {
                    println!("Successfully decrypted {}", &d.file);
                }
                Err((f, e)) => {
                    eprintln!("{}: File: '{}'; {}{}{}", RED_ERROR_TEXT, f, BOLD_START, e, STYLE_END);
                }
            }
        }
    }
}
