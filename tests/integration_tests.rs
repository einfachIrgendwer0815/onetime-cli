use std::ffi::OsString;
use std::fs::{read_dir, remove_file, remove_dir_all, create_dir, copy};
use std::fs::File;
use std::io::{Read, ErrorKind};
use assert_cmd::Command;
use md5_rs::Context;


const CARGO_BIN_NAME: &str = "onetime-cli";

const FILES_DIR: &str = "./tests/files";
const FILES_ORIG_DIR: &str = "./tests/files_original";

const FILE1_PATH: &str = "./tests/files/file1.txt";


fn copy_files(dir_name: &str) -> std::io::Result<()> {
    fn traverse_dir(dir: &str, dest_dir: &OsString) -> std::io::Result<()> {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let path_str = path.to_str().unwrap();
                let new_dest_dir = {
                    let dir_name = path.file_name().unwrap();

                    let mut dir = dest_dir.to_owned();
                    dir.push("/");
                    dir.push(dir_name);

                    dir
                };

                match create_dir(&new_dest_dir) {
                    Ok(_) => {},
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::AlreadyExists => (),
                            _ => {
                                return Err(e);
                            }
                        }
                    }
                };

                traverse_dir(path_str, &new_dest_dir)?
            } else if path.is_file() {
                let mut new_file = dest_dir.to_owned();
                new_file.push("/");
                new_file.push(path.file_name().unwrap());

                copy(path, new_file)?;
            }
        }

        Ok(())
    }

    let dir_name = FILES_ORIG_DIR.to_owned() + "/" + dir_name;

    traverse_dir(&dir_name, &OsString::from(FILES_DIR))?;

    Ok(())
}

fn clear_files() {
    if let Err(e) = remove_dir_all(FILES_DIR) {
        match e.kind() {
            ErrorKind::NotFound => (),
            _ => {
                panic!("{}", e);
            }
        }
    }
    
    if let Err(e) = create_dir(FILES_DIR) {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => {
                panic!("{}", e);
            }
        }
    }
}

fn get_md5_sum(path: &str) -> Result<[u8; 16], std::io::Error> {
    let mut ctx = Context::new();

    let mut file = File::open(path)?;
    let mut buffer = [0u8; 4096];

    loop {
        let bytes = file.read(&mut buffer)?;

        if bytes == 0 {
            break;
        }

        ctx.read(&buffer[..bytes]);
    }

    Ok(ctx.finish())
}

#[test]
fn test_encrpyt_decrypt() {
    clear_files();
    copy_files("test_encrypt_decrypt").unwrap();

    let original_md5 = get_md5_sum(FILE1_PATH).unwrap();

    // Encrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir(FILES_DIR)
        .arg("encrypt")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully encrypted file1.txt\n")
        .stderr("");

    remove_file(FILE1_PATH).unwrap();

    // Decrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir(FILES_DIR)
        .arg("decrypt")
        .arg("file1.txt")
        .assert();
    
    assert
        .success()
        .stdout("Successfully decrypted file1.txt\n")
        .stderr("");

    let md5_now = get_md5_sum(FILE1_PATH).unwrap();

    assert_eq!(original_md5, md5_now);
}