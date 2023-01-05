//! # Onetime-cli
//!
//!Encrypt / decrypt files using the one-time-pad.
//! 
//! ## Usage
//!
//! The simplest way to encrypt a file called `secret.txt` is:
//!
//! ```bash
//! onetime-cli encrypt secret.txt
//! ```
//!
//! which will generate two new files `secret.txt.otp.0` and `secret.txt.otp.1`. You can then delete `secret.txt`.
//!
//!
//!
//! To decrypt `secret.txt`, run:
//!
//! ```bash
//! onetime-cli decrypt secret.txt
//! ```
//!
//! which will use the two `secret.txt.otp.*` files to decrypt `secret.txt`. You can then delete these two files.
//!
//!
//!
//! To see more possible cli arguments, run:
//!
//! ```bash
//! onetime-cli --help
//! ```

pub mod args;

mod otp;
mod fs;

pub use otp::{decrypt, encrypt};