use assert_cmd::Command;
use md5_rs::Context;
use scopeguard::defer;
use std::ffi::OsString;
use std::fs::File;
use std::fs::{copy, create_dir, create_dir_all, metadata, read_dir, remove_dir_all, remove_file};
use std::io::{ErrorKind, Read};

const CARGO_BIN_NAME: &str = "onetime-cli";

const FILES_DIR: &str = "./tests/files";
const FILES_ORIG_DIR: &str = "./tests/files_original";

fn copy_files(input_dir: &str, dest_dir: &str) -> std::io::Result<()> {
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
                    Ok(_) => {}
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => (),
                        _ => {
                            return Err(e);
                        }
                    },
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

    let orig_dir_name = format!("{FILES_ORIG_DIR}/{input_dir}");
    let dest_dir = format!("{FILES_DIR}/{dest_dir}");

    clear_files(&dest_dir);

    if let Err(e) = create_dir_all(&dest_dir) {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => {
                panic!("{dest_dir}: {e}");
            }
        }
    };

    traverse_dir(&orig_dir_name, &OsString::from(dest_dir))?;

    Ok(())
}

fn clear_files(dir_name: &str) {
    let dir_name = format!("{FILES_DIR}/{dir_name}");

    if let Err(e) = remove_dir_all(dir_name) {
        match e.kind() {
            ErrorKind::NotFound => (),
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

fn assert_path_exists(path: &str) {
    if let Err(e) = metadata(path) {
        panic!("{path}: {e}");
    }
}

fn assert_path_not_found(path: &str) {
    if let Err(e) = metadata(path) {
        match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("{path}: {e}"),
        }
    } else {
        panic!("{path} exists");
    }
}

#[test]
fn test_encrypt_decrypt() {
    copy_files("files_1", "test_encrypt_decrypt").unwrap();

    defer! {
        clear_files("test_encrypt_decrypt")
    }

    let original_md5 = get_md5_sum("./tests/files/test_encrypt_decrypt/file1.txt").unwrap();

    // Encrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt")
        .arg("encrypt")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully encrypted file1.txt\n")
        .stderr("");

    remove_file("./tests/files/test_encrypt_decrypt/file1.txt").unwrap();

    // Decrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt")
        .arg("decrypt")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully decrypted file1.txt\n")
        .stderr("");

    let md5_now = get_md5_sum("./tests/files/test_encrypt_decrypt/file1.txt").unwrap();

    assert_eq!(original_md5, md5_now);
}

#[test]
fn test_encrypt_decrypt_with_manually_set_input_and_output_files() {
    copy_files(
        "files_1",
        "test_encrypt_decrypt_with_manually_set_input_and_output_files",
    )
    .unwrap();

    defer! {
        clear_files("test_encrypt_decrypt_with_manually_set_input_and_output_files")
    }

    let original_md5 = get_md5_sum(
        "./tests/files/test_encrypt_decrypt_with_manually_set_input_and_output_files/file1.txt",
    )
    .unwrap();

    // Encrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt_with_manually_set_input_and_output_files")
        .arg("encrypt")
        .args(["--out1-suffix", "encrypted.part0"])
        .args(["--out2-suffix", "encrypted.part1"])
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully encrypted file1.txt\n")
        .stderr("");

    remove_file(
        "./tests/files/test_encrypt_decrypt_with_manually_set_input_and_output_files/file1.txt",
    )
    .unwrap();
    assert_path_exists(&format!("{FILES_DIR}/test_encrypt_decrypt_with_manually_set_input_and_output_files/file1.txt.encrypted.part0"));
    assert_path_exists(&format!("{FILES_DIR}/test_encrypt_decrypt_with_manually_set_input_and_output_files/file1.txt.encrypted.part1"));

    // Decrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt_with_manually_set_input_and_output_files")
        .arg("decrypt")
        .args(["--in1-suffix", "encrypted.part1"])
        .args(["--in2-suffix", "encrypted.part0"])
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully decrypted file1.txt\n")
        .stderr("");

    let md5_now = get_md5_sum(
        "./tests/files/test_encrypt_decrypt_with_manually_set_input_and_output_files/file1.txt",
    )
    .unwrap();

    assert_eq!(original_md5, md5_now);
}

#[test]
fn test_delete_after_encryption() {
    copy_files("files_1", "test_delete_after_encryption").unwrap();

    defer! {
        clear_files("test_delete_after_encryption")
    }

    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_delete_after_encryption")
        .arg("encrypt")
        .arg("--rm")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully encrypted file1.txt\n")
        .stderr("");

    assert_path_not_found(&format!(
        "{FILES_DIR}/test_delete_after_encryption/file1.txt"
    ));
    assert_path_exists(&format!(
        "{FILES_DIR}/test_delete_after_encryption/file1.txt.otp.0"
    ));
    assert_path_exists(&format!(
        "{FILES_DIR}/test_delete_after_encryption/file1.txt.otp.1"
    ));
}

#[test]
fn test_delete_after_decryption() {
    copy_files("files_2", "test_delete_after_decryption").unwrap();

    defer! {
        clear_files("test_delete_after_decryption")
    }

    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_delete_after_decryption")
        .arg("decrypt")
        .arg("--rm")
        .arg("file2.txt")
        .assert();

    assert
        .success()
        .stdout("Successfully decrypted file2.txt\n")
        .stderr("");

    assert_path_not_found(&format!(
        "{FILES_DIR}/test_delete_after_decryption/file2.txt.otp.0"
    ));
    assert_path_not_found(&format!(
        "{FILES_DIR}/test_delete_after_decryption/file2.txt.otp.1"
    ));
    assert_path_exists(&format!(
        "{FILES_DIR}/test_delete_after_decryption/file2.txt"
    ));
}

#[test]
fn test_encrypt_decrypt_quiet() {
    copy_files("files_1", "test_encrypt_decrypt_quiet").unwrap();

    defer! {
        clear_files("test_encrypt_decrypt_quiet")
    }

    let original_md5 = get_md5_sum("./tests/files/test_encrypt_decrypt_quiet/file1.txt").unwrap();

    // Encrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt_quiet")
        .arg("-q")
        .arg("encrypt")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("")
        .stderr("");

    remove_file("./tests/files/test_encrypt_decrypt_quiet/file1.txt").unwrap();

    // Decrypt command
    let mut cmd = Command::cargo_bin(CARGO_BIN_NAME).unwrap();
    let assert = cmd
        .current_dir("./tests/files/test_encrypt_decrypt_quiet")
        .arg("-q")
        .arg("decrypt")
        .arg("file1.txt")
        .assert();

    assert
        .success()
        .stdout("")
        .stderr("");

    let md5_now = get_md5_sum("./tests/files/test_encrypt_decrypt_quiet/file1.txt").unwrap();

    assert_eq!(original_md5, md5_now);
}
