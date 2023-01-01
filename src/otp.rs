use crate::args::{Decrypt, Encrypt};

use std::fs::File;
use std::io::{Read, Result, Write};

use rand::Rng;

pub fn encrypt(e: &Encrypt) -> Result<()> {
    let mut f_in = File::open(&e.file)?;
    let mut f_out1 = File::create(e.out1.as_ref().unwrap())?;
    let mut f_out2 = File::create(e.out2.as_ref().unwrap())?;

    let mut buf_in = vec![0u8; e.buffer as usize];
    let mut buf_out_1 = vec![0u8; e.buffer as usize];
    let mut buf_out_2 = vec![0u8; e.buffer as usize];

    let mut rng = rand::thread_rng();

    loop {
        let bytes = f_in.read(&mut buf_in)?;

        if bytes == 0 {
            break;
        }

        for i in 0..bytes {
            buf_out_1[i] = rng.gen_range(0..=255);
            buf_out_2[i] = buf_in[i] ^ buf_out_1[i];
        }

        let _ = f_out1.write(&buf_out_1[..bytes])?;
        let _ = f_out2.write(&buf_out_2[..bytes])?;
    }

    Ok(())
}

pub fn decrypt(d: &Decrypt) -> Result<()> {
    let mut f_in_1 = File::open(d.in1.as_ref().unwrap())?;
    let mut f_in_2 = File::open(d.in2.as_ref().unwrap())?;
    let mut f_out = File::create(&d.file)?;

    let mut buf_in_1: Vec<u8> = vec![0u8; d.buffer as usize];
    let mut buf_in_2: Vec<u8> = vec![0u8; d.buffer as usize];
    let mut buf_out: Vec<u8> = vec![0u8; d.buffer as usize];

    loop {
        let bytes_1 = f_in_1.read(&mut buf_in_1)?;
        let bytes_2 = f_in_2.read(&mut buf_in_2)?;

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

        let _ = f_out.write(&buf_out[..bytes_1])?;
    }

    Ok(())
}
