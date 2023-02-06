use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, ArgAction,
    ArgMatches, Command,
};

use onetime_cli::args::{Decrypt, Encrypt};

pub enum Subcommand {
    Encrypt(Encrypt),
    Decrypt(Decrypt),
    None,
}

impl From<ArgMatches> for Subcommand {
    fn from(value: ArgMatches) -> Self {
        match value.subcommand() {
            Some(("encrypt", args)) => Self::Encrypt(Encrypt {
                file: args.get_one::<String>("file").unwrap().to_string(),
                out1_suffix: args.get_one::<String>("out1_suffix").unwrap().to_string(),
                out2_suffix: args.get_one::<String>("out2_suffix").unwrap().to_string(),
                buffer: *args.get_one::<u32>("buffer").unwrap(),
                rm: args.get_flag("remove_input"),
            }),
            Some(("decrypt", args)) => Self::Decrypt(Decrypt {
                file: args.get_one::<String>("file").unwrap().to_string(),
                in1_suffix: args.get_one::<String>("in1_suffix").unwrap().to_string(),
                in2_suffix: args.get_one::<String>("in2_suffix").unwrap().to_string(),
                buffer: *args.get_one::<u32>("buffer").unwrap(),
                rm: args.get_flag("remove_input"),
            }),
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
                Arg::new("out1_suffix")
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
                Arg::new("out2_suffix")
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
                    Arg::new("in1_suffix")
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
                    Arg::new("in2_suffix")
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
