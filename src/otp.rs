use crate::args::{Decrypt, Encrypt};
use crate::fs::Mode;
use crate::fs::{open_file, read, remove_file, write};
use crate::Error;

use rand::Rng;

/// Encrypts data using the one-time-pad.
///
/// ## Panic
/// Will panic if either of `buf_out1` or `buf_out2` is smaller than `buf_in`.
///
/// ## Example
/// ```
/// use onetime_cli::encrypt;
///
/// let data: [u8; 10] = [1,2,3,4,5,6,7,8,9,10];
/// let mut out1 = [0u8; 10];
/// let mut out2 = [0u8; 10];
///
/// encrypt(&data, &mut out1, &mut out2);
/// // The encrypted parts are stored in `out1` and `out2`.
///
/// println!("{:?}", out1);
/// println!("{:?}", out2);
/// ```
pub fn encrypt(buf_in: &[u8], buf_out1: &mut [u8], buf_out2: &mut [u8]) {
    assert!(buf_out1.len() >= buf_in.len());
    assert!(buf_out2.len() >= buf_in.len());

    let mut rng = rand::thread_rng();

    for i in 0..buf_in.len() {
        buf_out1[i] = rng.gen_range(0..=255);
        buf_out2[i] = buf_in[i] ^ buf_out1[i];
    }
}

/// Decrypts data using the one-time-pad.
///
/// `buf_in1` and `buf_in2` must have the same size.
///
/// ## Panic
/// Will panic if:
///   * `buf_in1` and `buf_in2` don't have the same size.
///   * `buf_out` is smaller than the `buf_in`s.
///
/// ## Example
/// ```
/// use onetime_cli::decrypt;
///
/// let in1: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let in2: [u8; 10] = [26, 89, 3, 93, 78, 12, 60, 23, 4, 71];
/// let mut data = [0u8; 10];
///
/// decrypt(&in1, &in2, &mut data);
/// // The decrypted data is stored in `data`.
///
/// assert_eq!(data, [27, 91, 0, 89, 75, 10, 59, 31, 13, 77]);
/// ```
pub fn decrypt(buf_in1: &[u8], buf_in2: &[u8], buf_out: &mut [u8]) {
    assert!(buf_in1.len() == buf_in2.len());
    assert!(buf_out.len() >= buf_in1.len());

    for i in 0..buf_in1.len() {
        buf_out[i] = buf_in1[i] ^ buf_in2[i];
    }
}

/// Encrypts a file using the options wrapped in an [Encrypt].
///
/// ## Error
///
/// Returns an error if any of the I/O operations fail.
///
/// ## Panic
/// Will panic if [Encrypt].out1 or [Encrypt].out2 is `None`
///
/// ## Example
/// ```
/// use onetime_cli::args::Encrypt;
/// use onetime_cli::encrypt_file;
/// use onetime_cli::Error;
///
/// let e = Encrypt {
///     file: "secret.txt".to_string(),
///     out1_suffix: "otp.0".to_string(),
///     out2_suffix: "otp.1".to_string(),
///     buffer: 1048576, // this is the default
///     rm: false // keep input files
/// };
///
/// let res = encrypt_file(&e);
///
/// match res {
///     Ok(_) => {
///         println!("Successfully encrypted secret.txt");
///     },
///     Err(e) => {
///         match e {
///             Error::IoError(io_e) => {
///                 eprintln!("Encrypting {} failed. {}", io_e.file, io_e.error);
///             }
///             Error::InvalidInput(i) => {
///                 eprintln!("{i}");
///             }
///         }
///     },
/// };
/// ```
pub fn encrypt_file(e: &Encrypt) -> Result<(), Error> {
    let mut f_in = open_file(&e.file, Mode::Open)?;

    let f_out1_name = format!("{}.{}", e.file, e.out1_suffix);
    let f_out2_name = format!("{}.{}", e.file, e.out2_suffix);
    let mut f_out1 = open_file(&f_out1_name, Mode::Create)?;
    let mut f_out2 = open_file(&f_out2_name, Mode::Create)?;

    let mut buf_in = vec![0u8; e.buffer as usize];
    let mut buf_out1 = vec![0u8; e.buffer as usize];
    let mut buf_out2 = vec![0u8; e.buffer as usize];

    loop {
        let bytes = read(&mut f_in, &mut buf_in)?;

        if bytes == 0 {
            break;
        }

        encrypt(
            &buf_in[..bytes],
            &mut buf_out1[..bytes],
            &mut buf_out2[..bytes],
        );

        write(&mut f_out1, &buf_out1[..bytes])?;
        write(&mut f_out2, &buf_out2[..bytes])?;
    }

    if e.rm {
        remove_file(&e.file)?;
    }

    Ok(())
}

/// Decrypts a file using the options wrapped in a [Decrypt].
///
/// ## Error
///
/// Returns an error if:
///  - any of the I/O operations fail
///  - the two input files differ in length
///
/// ## Panic
/// Will panic if [Decrypt].in1 or [Decrypt].in2 is `None`.
///
/// ## Example
/// ```
/// use onetime_cli::args::Decrypt;
/// use onetime_cli::decrypt_file;
/// use onetime_cli::Error;
///
/// let d = Decrypt {
///     file: "secret.txt".to_string(),
///     in1_suffix: "otp.0".to_string(),
///     in2_suffix: "otp.1".to_string(),
///     buffer: 1048576, // this is the default
///     rm: false // keep input files
/// };
///
/// let res = decrypt_file(&d);
///
/// match res {
///     Ok(_) => {
///         println!("Successfully decrypted secret.txt");
///     },
///     Err(e) => {
///         match e {
///             Error::IoError(io_e) => {
///                 eprintln!("Decrypting {} failed. {}", io_e.file, io_e.error);
///             }
///             Error::InvalidInput(i) => {
///                 eprintln!("{i}");
///             }
///         }
///     },
/// };
/// ```
pub fn decrypt_file(d: &Decrypt) -> Result<(), Error> {
    let f_in1_name = format!("{}.{}", d.file, d.in1_suffix);
    let f_in2_name = format!("{}.{}", d.file, d.in2_suffix);
    let mut f_in1 = open_file(&f_in1_name, Mode::Open)?;
    let mut f_in2 = open_file(&f_in2_name, Mode::Open)?;
    let mut f_out = open_file(&d.file, Mode::Create)?;

    let mut buf_in1 = vec![0u8; d.buffer as usize];
    let mut buf_in2 = vec![0u8; d.buffer as usize];
    let mut buf_out = vec![0u8; d.buffer as usize];

    loop {
        let bytes_1 = read(&mut f_in1, &mut buf_in1)?;
        let bytes_2 = read(&mut f_in2, &mut buf_in2)?;

        if bytes_1 != bytes_2 {
            return Err(Error::InvalidInput(
                "The two input files differ in size!".to_string(),
            ));
        }

        if bytes_1 == 0 {
            break;
        }

        decrypt(
            &buf_in1[..bytes_1],
            &buf_in2[..bytes_1],
            &mut buf_out[..bytes_1],
        );

        write(&mut f_out, &buf_out[..bytes_1])?;
    }

    if d.rm {
        remove_file(&f_in1_name)?;
        remove_file(&f_in2_name)?;
    }

    Ok(())
}
