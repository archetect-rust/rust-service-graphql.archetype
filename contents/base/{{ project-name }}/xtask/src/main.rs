mod cli;

use std::process;

use anyhow::{anyhow, Result};
use clap::{ArgMatches, Command};
use crate::cli::{BuildArchitecture, BuildStyle, BuildMode};

const APPLICATION_NAME_FULL: &str = concat!("naxgrp.jfrog.io/nax-platform-docker/applications/{{ project-name }}");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args = cli::arg_matches();

    match args.subcommand() {
        Some(("build", args)) => handle_build_command(args),
        Some(("docker", args)) => handle_docker_commands(args),
        Some(("install", args)) => handle_install_command(args),
        Some((command, _)) => anyhow::bail!("Unexpected command: {command}"),
        None => anyhow::bail!("Expected subcommand"),
    }
}

fn handle_install_command(_args: &ArgMatches) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command.args(["install", "--path", "crates/{{ project_name }}_bin"]);

    Ok(())
}

fn handle_build_command(args: &ArgMatches) -> Result<()> {
    let mode = args.get_one::<BuildMode>("mode");
    let architecture = args.get_one::<BuildArchitecture>("architecture");

    let mut command = process::Command::new("cargo");
    command.arg("build");

    if let Some(BuildMode::Release) = mode {
        command.arg("--release");
    }

    if let Some(architecture) = architecture {
        command.arg("--target")
            .arg(format!("{architecture}"));
    };

    command.status()?;

    Ok(())
}

fn handle_docker_commands(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("build", args)) => docker_build(args),
        Some(("remove", args)) => docker_rmi(args),
        Some(("run", args)) => docker_run(args),
        Some(("image", args)) => docker_image(args),
        _ => Ok(()),
    }
}

fn docker_build(args: &ArgMatches) -> Result<()> {
    let mode = args.get_one::<BuildMode>("mode").expect("Required by Clap");
    let style = args.get_one::<BuildStyle>("style").expect("Required by Clap");
    let architecture = args.get_one::<BuildArchitecture>("architecture");

    println!("Build Profile: {mode}");
    println!("Package From: {style}");

    let mut command = process::Command::new("docker");

    command
        .arg("build")
        .arg("--platform")
        .arg("linux/arm64")
        .arg("-t")
        .arg(format!("{}:{}", APPLICATION_NAME_FULL, VERSION))
        .arg("-t")
        .arg(format!("{}:{}", APPLICATION_NAME_FULL, "latest"))
        .arg("--build-arg")
        .arg(format!("profile={mode}"))
        .arg("--build-arg")
        .arg(format!("style={style}"))
        .arg("--file")
        .arg(format!(".platform/docker/{style}/Dockerfile"))
        .arg(".")
        .arg("--progress=plain")
        .arg("--no-cache")
    ;

    if let Some(architecture) = architecture {
        command
            .arg("--build-arg")
            .arg(format!("architecture={architecture}"));
    }

    command.status()?;

    Ok(())
}

fn docker_run(_args: &ArgMatches) -> Result<()> {
    process::Command::new("docker")
        .arg("run")
        .arg("--platform")
        .arg("linux/arm64")
        .arg("-it")
        .arg(APPLICATION_NAME_FULL)
        .status()?;
    Ok(())
}

fn docker_image(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("name", _args)) => println!("{APPLICATION_NAME_FULL}"),
        Some(("version", _args)) => println!("{VERSION}"),
        _ => println!("{APPLICATION_NAME_FULL}:{VERSION}"),
    }

    Ok(())
}

fn docker_rmi(_args: &ArgMatches) -> Result<()> {
    process::Command::new("docker")
        .arg("rmi")
        .arg(APPLICATION_NAME_FULL)
        .spawn()?
        .wait()?;
    process::Command::new("docker")
        .arg("rmi")
        .arg(format!("{APPLICATION_NAME_FULL}:{VERSION}"))
        .spawn()?
        .wait()?;

    Ok(())
}
