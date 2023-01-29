use crate::args::{Decrypt, Encrypt};
use crate::fs::Mode;
use crate::fs::{open_file, read, remove_file, write};

use rand::Rng;

/// Encrypts data using the one-time-pad.
///
/// ## Panic
/// Will panic if either of `buf_out1` or `buf_out2` is smaller than `buf_in`.
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
pub fn decrypt(buf_in1: &[u8], buf_in2: &[u8], buf_out: &mut [u8]) {
    assert!(buf_in1.len() == buf_in2.len());
    assert!(buf_out.len() >= buf_in1.len());

    for i in 0..buf_in1.len() {
        buf_out[i] = buf_in1[i] ^ buf_in2[i];
    }
}

/// Encrypts a file using the options wrapped in an [Encrypt].
///
/// ## Panic
/// Will panic if [Encrypt].out1 or [Encrypt].out2 is `None`
pub fn encrypt_file(e: &Encrypt) -> Result<(), (String, String)> {
    assert!(e.out1.is_some());
    assert!(e.out2.is_some());

    let mut f_in = open_file(&e.file, Mode::Open)?;
    let mut f_out1 = open_file(e.out1.as_ref().unwrap(), Mode::Create)?;
    let mut f_out2 = open_file(e.out2.as_ref().unwrap(), Mode::Create)?;

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

/// Decrypts a file using the options wrapped in an [Decrypt].
///
/// ## Panic
/// Will panic if [Decrypt].in1 or [Decrypt].in2 is `None`.
pub fn decrypt_file(d: &Decrypt) -> Result<(), (String, String)> {
    assert!(d.in1.is_some());
    assert!(d.in2.is_some());

    let mut f_in1 = open_file(d.in1.as_ref().unwrap(), Mode::Open)?;
    let mut f_in2 = open_file(d.in2.as_ref().unwrap(), Mode::Open)?;
    let mut f_out = open_file(&d.file, Mode::Create)?;

    let mut buf_in1 = vec![0u8; d.buffer as usize];
    let mut buf_in2 = vec![0u8; d.buffer as usize];
    let mut buf_out = vec![0u8; d.buffer as usize];

    loop {
        let bytes_1 = read(&mut f_in1, &mut buf_in1)?;
        let bytes_2 = read(&mut f_in2, &mut buf_in2)?;

        if bytes_1 != bytes_2 {
            return Err((
                d.file.clone(),
                "The two input files must be of the same size!".to_string(),
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
        remove_file(d.in1.as_ref().unwrap())?;
        remove_file(d.in2.as_ref().unwrap())?;
    }

    Ok(())
}
