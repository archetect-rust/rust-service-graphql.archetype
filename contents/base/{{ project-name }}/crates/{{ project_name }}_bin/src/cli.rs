use clap::{Arg, ArgAction, ArgMatches, command, Command, value_parser};
use clap::builder::{BoolishValueParser, EnumValueParser, PathBufValueParser};

use crate::traces::TraceFormat;

pub fn arg_matches() -> ArgMatches {
    command!()
        .name("{{ project-name }}-gateway")
        .subcommand(
            Command::new("migrate")
                .subcommand_required(true)
                .about("Database Migrations")
                .subcommand(Command::new("up").about("Apply migrations"))
                .subcommand(
                    Command::new("down")
                        .about("Roll back migrations.  Rolls back a single migration at a time, by default.")
                        .arg(
                            Arg::new("all")
                                .help("Rollback ALL migrations.  This will effectively destroy your entire database!")
                                .long("all")
                                .action(ArgAction::Set)

                        ),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Configuration Operations")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("defaults").about("Displays the default settings"))
                .subcommand(Command::new("merged").about("Displays the effective settings from all merged sources."))
                .subcommand(
                    Command::new("generate")
                        .about("Generate the effective settings in an adjacent yml file, overwriting existing config."),
                ),
        )
        .arg(
            Arg::new("config-file")
                .help("Specifies additional configuration to merge.")
                .long("config-file")
                .short('c')
                .value_parser(PathBufValueParser::new())
                .action(ArgAction::Set)
                .env("CONFIG_FILE")
        )
        .arg(
            Arg::new("host")
                .help("The host the server listens on.")
                .long("host")
                .action(ArgAction::Set)
                .env("SERVER_HOST")
        )
        .arg(
            Arg::new("log-sql")
                .help("Turns sql logging on or off.")
                .long("log-sql")
                .action(ArgAction::SetTrue)
                .value_parser(BoolishValueParser::new())
                .env("PERSISTENCE_DATABASE_LOG_SQL")
        )
        .arg(
            Arg::new("service-port")
                .help("Service Port")
                .short('p')
                .long("service-port")
                .action(ArgAction::Set)
                .value_parser(value_parser!(i64).range(1024..65535))
                .env("SERVER_SERVICE_PORT")
        )
        .arg(
            Arg::new("temp-db")
                .help("Initialize and migrate an ephemeral database")
                .long("temp-db")
                .value_parser(BoolishValueParser::new())
                .action(ArgAction::SetTrue)
                .env("PERSISTENCE_TEMP_DB")
        )
        .arg(
            Arg::new("migrate")
                .help("Whether or not to automatically migrate the database")
                .long("migrate")
                .action(ArgAction::SetTrue)
                .value_parser(BoolishValueParser::new())
                .env("PERSISTENCE_MIGRATE")

        )
        .arg(
            Arg::new("database-url")
                .help("Database URL")
                .long("database-url")
                .action(ArgAction::Set)
                .env("PERSISTENCE_DATABASE_URL")
        )
        .arg(
            Arg::new("tracing-format")
                .help("Specify logging format")
                .long("tracing-format")
                .value_parser(EnumValueParser::<TraceFormat>::new())
                .action(ArgAction::Set)
                .env("TRACING_FORMAT")
        )
        .arg(
            Arg::new("tracing-filter")
                .help("Specify logging and tracing level filters")
                .long("tracing-filter")
                .action(ArgAction::Set)
                .env("TRACING_FILTER")
        )
    {%- for application_key in applications %}
    {%- set application = applications[application_key] %}
        .arg(
            Arg::new("{{ application['project-name'] }}")
                .help("{{ application['project-title'] }} Endpoint")
                .long("{{ application['project-name'] }}")
                .action(ArgAction::Set)
                .env("CORE_{{ application['PROJECT_NAME'] }}_URL")
        )
    {%- endfor %}
        .get_matches()
}