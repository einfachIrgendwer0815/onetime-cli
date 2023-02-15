mod clap_app;
mod error_handler;

use clap_app::{build_clap_app, Subcommand};
use error_handler::ErrorHandler;

fn main() {
    let args = build_clap_app().get_matches();

    match Subcommand::from(args) {
        Subcommand::Encrypt(e) => {
            onetime_cli::encrypt_file(&e).unwrap_or_exit(1);

            if !e.quiet {
                println!("Successfully encrypted {}", e.file.to_string_lossy());
            }
        }
        Subcommand::Decrypt(d) => {
            onetime_cli::decrypt_file(&d).unwrap_or_exit(1);

            if !d.quiet {
                println!("Successfully decrypted {}", d.file.to_string_lossy());
            }
        }
        Subcommand::None => {
            let mut cmd = clap_app::build_clap_app();
            println!("{}", cmd.render_help());
        }
    }
}
