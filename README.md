# Onetime-cli

[![Crates.io][crates_img]][crates_lnk]
[![Docs.rs][docs_img]][docs_lnk]
[![Crates.io][dwn_img]][crates_lnk]
[![Github.com][issues_img]][issues_lnk]
[![Github.com][license_img]][license_lnk]

[crates_img]:https://img.shields.io/crates/v/onetime-cli
[crates_lnk]:https://crates.io/crates/onetime-cli

[docs_img]:https://img.shields.io/docsrs/onetime-cli/latest
[docs_lnk]:https://docs.rs/onetime-cli

[dwn_img]:https://img.shields.io/crates/d/onetime-cli

[license_img]:https://img.shields.io/crates/l/onetime-cli
[license_lnk]:https://github.com/einfachIrgendwer0815/onetime-cli/blob/main/LICENSE

[issues_img]:https://img.shields.io/github/issues/einfachIrgendwer0815/onetime-cli
[issues_lnk]:https://github.com/einfachIrgendwer0815/onetime-cli/issues

Encrypt / decrypt files using the one-time-pad.

## Install

If you have cargo installed, run:
```bash
cargo install onetime-cli
```

Otherwise you can download an executable from the [Release section](https://github.com/einfachIrgendwer0815/onetime-cli/releases).



## Usage

The simplest way to encrypt a file called `secret.txt` is:
```bash
onetime-cli encrypt secret.txt
```
which will generate two new files `secret.txt.otp.0` and `secret.txt.otp.1`. You can then delete `secret.txt`.



To decrypt `secret.txt`, run:
```bash
onetime-cli decrypt secret.txt
```
which will use the two `secret.txt.otp.*` files to decrypt `secret.txt`. You can then delete these two files.



To see more possible cli arguments, run:
```bash
onetime-cli --help
```
