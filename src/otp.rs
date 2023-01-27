use crate::args::{Decrypt, Encrypt};
use crate::fs::{open_file, read, remove_file, write, Mode};

use rand::Rng;

/// Encrypts a file using the options wrapped in an [Encrypt].
///
/// ## Panic
/// Will panic if [Encrypt].out1 or [Encrypt].out2 is `None`
pub fn encrypt(e: &Encrypt) -> Result<(), (String, String)> {
    assert!(e.out1.is_some());
    assert!(e.out2.is_some());

    let mut f_in = open_file(&e.file, Mode::Open)?;
    let mut f_out1 = open_file(e.out1.as_ref().unwrap(), Mode::Create)?;
    let mut f_out2 = open_file(e.out2.as_ref().unwrap(), Mode::Create)?;

    let mut buf_in = vec![0u8; e.buffer as usize];
    let mut buf_out_1 = vec![0u8; e.buffer as usize];
    let mut buf_out_2 = vec![0u8; e.buffer as usize];

    let mut rng = rand::thread_rng();

    loop {
        let bytes = read(&mut f_in, &mut buf_in)?;

        if bytes == 0 {
            break;
        }

        for i in 0..bytes {
            buf_out_1[i] = rng.gen_range(0..=255);
            buf_out_2[i] = buf_in[i] ^ buf_out_1[i];
        }

        let _ = write(&mut f_out1, &buf_out_1[..bytes])?;
        let _ = write(&mut f_out2, &buf_out_2[..bytes])?;
    }

    if e.rm {
        remove_file(&e.file)?;
    }

    Ok(())
}

/// Decrypts a file using the options wrapped in an [Decrypt].
///
/// ## Panic
/// Will panic if [Decrypt].in1 or [Decrypt].in2 is `None`
pub fn decrypt(d: &Decrypt) -> Result<(), (String, String)> {
    assert!(d.in1.is_some());
    assert!(d.in1.is_some());

    let mut f_in_1 = open_file(d.in1.as_ref().unwrap(), Mode::Open)?;
    let mut f_in_2 = open_file(d.in2.as_ref().unwrap(), Mode::Open)?;
    let mut f_out = open_file(&d.file, Mode::Create)?;

    let mut buf_in_1: Vec<u8> = vec![0u8; d.buffer as usize];
    let mut buf_in_2: Vec<u8> = vec![0u8; d.buffer as usize];
    let mut buf_out: Vec<u8> = vec![0u8; d.buffer as usize];

    loop {
        let bytes_1 = read(&mut f_in_1, &mut buf_in_1)?;
        let bytes_2 = read(&mut f_in_2, &mut buf_in_2)?;

        if bytes_1 != bytes_2 {
            println!("The two input files must be of the same size!");
            std::process::exit(1);
        }

        if bytes_1 == 0 {
            break;
        }

        for i in 0..bytes_1 {
            buf_out[i] = buf_in_1[i] ^ buf_in_2[i];
        }

        let _ = write(&mut f_out, &buf_out[..bytes_1])?;
    }

    if d.rm {
        remove_file(d.in1.as_ref().unwrap())?;
        remove_file(d.in2.as_ref().unwrap())?;
    }

    Ok(())
}
