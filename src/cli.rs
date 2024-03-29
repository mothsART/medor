use clap::{Arg, Command};

pub fn build_cli(name: &'static str, version: &'static str) -> Command {
    Command::new(name)
        .bin_name(name)
        .version(version)
        .author("Ferry Jérémie ferryjeremie@free.fr")
        .about("search and launch application")
        .arg_required_else_help(true)
        .arg(
            Arg::new("app")
                .short('a')
                .help("search app name")
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .action(clap::ArgAction::SetTrue)
                .help("debug mode (show SQL requests)"),
        )
}
