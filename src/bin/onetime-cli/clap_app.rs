use std::path::PathBuf;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, ArgAction,
    ArgMatches, Command,
};

use onetime_cli::Config;

pub enum Subcommand {
    Encrypt(Config),
    Decrypt(Config),
    None,
}

impl From<ArgMatches> for Subcommand {
    fn from(value: ArgMatches) -> Self {
        match value.subcommand() {
            Some(("encrypt", args)) | Some(("decrypt", args)) => {
                let cfg = Config {
                    file: PathBuf::from(args.get_one::<String>("file").unwrap()),
                    suffix1: args.get_one::<String>("suffix1").unwrap().to_string(),
                    suffix2: args.get_one::<String>("suffix2").unwrap().to_string(),
                    buffer: *args.get_one::<u32>("buffer").unwrap(),
                    rm: args.get_flag("remove_input"),
                    quiet: value.get_flag("quiet"),
                };

                match value.subcommand_name().unwrap() {
                    "encrypt" => Subcommand::Encrypt(cfg),
                    "decrypt" => Subcommand::Decrypt(cfg),
                    _ => unreachable!("there are no other possible subcommands"),
                }
            }
            _ => Self::None,
        }
    }
}

pub fn build_clap_app() -> Command {
    let mut cmd = Command::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .after_help("Use --help for more detailed help information.")
        .after_long_help("Use -h for shorter help information.");

    cmd = build_subcommand_encrypt(cmd);
    cmd = build_subcommand_decrypt(cmd);
    cmd = build_main_args(cmd);
    cmd
}

fn build_main_args(cmd: Command) -> Command {
    cmd
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Don't print anything to stdout")
        )
}

fn build_subcommand_encrypt(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new("encrypt")
            .about("Encrypt a file")
            .arg(
                Arg::new("file")
                    .value_name("FILE")
                    .action(ArgAction::Set)
                    .required(true)
                    .help("File to be encrypted"),
            )
            .arg(
                Arg::new("suffix1")
                    .long("out1-suffix")
                    .value_name("suffix")
                    .default_value("otp.0")
                    .help("Suffix for the name of the first output file")
                    .long_help(
                        "Suffix for the name of the first output file.\n\n\
                            Example:\n  \
                            Let's say the input file's name is 'secret.txt'. Accordingly,\n  \
                            the first output file would be named 'secret.txt.<SUFFIX>'. So,\n  \
                            assuming the specified suffix is 'encrypted_1', the resulting full\n  \
                            file name would be 'secret.txt.encrypted_1'. Note, that the suffix\n  \
                            must NOT start with a dot.",
                    ),
            )
            .arg(
                Arg::new("suffix2")
                    .long("out2-suffix")
                    .value_name("suffix")
                    .default_value("otp.1")
                    .help("Suffix for the name of the second output file")
                    .long_help("The same as --out1-suffix, but for the second output file."),
            )
            .arg(
                Arg::new("buffer")
                    .short('b')
                    .value_name("size")
                    .value_parser(value_parser!(u32))
                    .default_value("1048576")
                    .action(ArgAction::Set)
                    .help("Buffer size in bytes"),
            )
            .arg(
                Arg::new("remove_input")
                    .long("rm")
                    .action(ArgAction::SetTrue)
                    .help("Delete input file after encryption"),
            ),
    )
}

fn build_subcommand_decrypt(cmd: Command) -> Command {
    cmd
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt a file")
                .arg(
                    Arg::new("file")
                        .value_name("FILE")
                        .action(ArgAction::Set)
                        .required(true)
                        .help("Output file name. This is the name of the decrypted file.")
                )
                .arg(
                    Arg::new("suffix1")
                        .long("in1-suffix")
                        .value_name("suffix")
                        .default_value("otp.0")
                        .help("Suffix for the name of the first input file")
                        .long_help(
                            "Suffix for the name of the first input file.\n\n\
                            Example:\n  \
                            Let's say the output file's name is 'secret.txt'. Accordingly,\n  \
                            the first input file's name is expected to be 'secret.txt.<SUFFIX>'.\n  \
                            So, assuming the specified suffix is 'encrypted_1', the resulting full\n  \
                            file name would be 'secret.txt.encrypted_1'. Note, that the suffix\n  \
                            must NOT start with a dot."
                        )
                )
                .arg(
                    Arg::new("suffix2")
                        .long("in2-suffix")
                        .value_name("suffix")
                        .default_value("otp.1")
                        .help("Suffix for the name of the second input file")
                        .long_help(
                            "The same as --in1-suffix, but for the second input file."
                        )
                )
                .arg(
                    Arg::new("buffer")
                        .short('b')
                        .value_name("size")
                        .value_parser(value_parser!(u32))
                        .default_value("1048576")
                        .action(ArgAction::Set)
                        .help("Buffer size in bytes")
                )
                .arg(
                    Arg::new("remove_input")
                        .long("rm")
                        .action(ArgAction::SetTrue)
                        .help("Delete input file after encryption")
                )
        )
}
