use clap::{Arg, ArgAction, ArgMatches, Command, ValueEnum};
use clap::builder::EnumValueParser;
use strum_macros::Display;

pub fn arg_matches() -> ArgMatches {
    clap::command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install application")
        )
        .subcommand(
            Command::new("build")
                .about("Build the application")
                .arg(
                    Arg::new("architecture")
                        .help("Build Architecture")
                        .long_help("The target runtime architecture for the build")
                        .long("architecture")
                        .short('a')
                        .value_parser(EnumValueParser::<BuildArchitecture>::new())
                        .action(ArgAction::Set)
                        .env("BUILD_ARCHITECTURE")
                )
                .arg(
                    Arg::new("mode")
                        .help("Build Mode")
                        .action(ArgAction::Set)
                        .long("mode")
                        .value_parser(EnumValueParser::<BuildMode>::new())
                        .default_value("release")
                        .env("BUILD_MODE")
                )
        )
        .subcommand(
            Command::new("docker")
                .about("Docker Commands")
                .subcommand(
                    Command::new("build")
                        .about("Builds a Docker Container")
                        .arg(
                            Arg::new("mode")
                                .help("Build Mode")
                                .action(ArgAction::Set)
                                .long("mode")
                                .value_parser(EnumValueParser::<BuildMode>::new())
                                .default_value("release")
                                .env("BUILD_MODE")
                        )
                        .arg(
                            Arg::new("style")
                                .help("Build Style")
                                .long_help("Whether to dockerize an externally-built application, or one built within a container")
                                .long("style")
                                .value_name("build style")
                                .value_parser(EnumValueParser::<BuildStyle>::new())
                                .default_value("external")
                                .action(ArgAction::Set)
                                .env("BUILD_STYLE")
                        )
                        .arg(
                            Arg::new("architecture")
                                .help("Build Architecture")
                                .long_help("The target runtime architecture for the build")
                                .long("architecture")
                                .short('a')
                                .value_parser(EnumValueParser::<BuildArchitecture>::new())
                                .action(ArgAction::Set)
                                .env("BUILD_ARCHITECTURE")
                        )
                    ,
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a the current Docker container")
                )
                .subcommand(
                    Command::new("run")
                        .about("Run a previously-built container")
                )
                .subcommand(
                    Command::new("image")
                        .about("Outputs information about the docker image")
                        .subcommand(
                            Command::new("name")
                                .about("Outputs the image name")
                        )
                        .subcommand(
                            Command::new("version")
                                .about("Outputs the image name")
                        )
                        .subcommand(
                            Command::new("full")
                                .about("Outputs the fully-qualified image name and version")
                        )
                )
                .subcommand(Command::new("push").about("Pushes a Docker Container")),
        )
        .get_matches()
}

#[derive(Copy, Clone, Debug, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BuildMode {
    Release,
    Debug,
}

#[derive(Copy, Clone, Debug, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BuildStyle {
    External,
    Container,
}

#[derive(Copy, Clone, Debug, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BuildArchitecture {
    #[strum(serialize = "aarch64-unknown-linux-gnu")]
    ARM64,
}
