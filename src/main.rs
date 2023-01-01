use clap::Parser;

use onetime_cli::args::{Action, Args};

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
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
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
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
