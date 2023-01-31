# Changelog

All notable changes to this project will be documented in this file.

This project uses semantic versioning (SemVer).


## Unreleased

### Added
  - Exit with code 1 on error
  - Custom error representation (onetime_cli::Error)
  - Parameters `--out1-suffix` and `--out2-suffix` of subcommand `encrypt`
  - Parameters `--in1-suffix` and `--in2-suffix` of subcommand `decrypt`

### Changed
  - Improved error messages

### [BREAKING] changes
  - Moved previous functionality of onetime_cli::encrypt to onetime_cli::encrypt_file
  - Moved previous functionality of onetime_cli::decrypt to onetime_cli::decrypt_file
  - onetime_cli::encrypt now encrypts data from an array using the one-time-pad
  - onetime_cli::decrypt now decrypts data into an array using the one-time-pad
  - Return type of the functions onetime_cli::{encrypt_file, decrypt_file} changed to Result<(), onetime_cli::Error>

### Removed
  - Parameters `-1`/`--out1` and `-2`/`--out2` of subcommand `encrypt`
  - Parameters `-1`/`--in1` and `-2`/`--in2` of subcommand `decrypt`
  - `-r` as alias for `--rm`. `--rm` is not removed, only `-r`.


## v0.2.0 -- 2023-01-02

### Added
  - `--rm` (`-r`) to remove input files after encryption/decryption


## v0.1.0 -- 2023-01-01

### Added
  - Encrypt files with `onetime-cli encrypt ...`
    * Set names for output files with `--out1` (`-1`) and `--out2` (`-2`)
  - Decrypt files with `onetime-cli decrypt ...`
    * Set names for input files with `--in1` (`-1`) and `--in2` (`-2`)

  - Set buffer size
