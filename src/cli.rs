use clap::{Arg, Command};

pub fn build_cli(name: &'static str, version: &'static str) -> Command<'static> {
    Command::new(name)
        .bin_name(name)
        .version(version)
        .author("Ferry Jérémie ferryjeremie@free.fr")
        .about("search and launch  application")
        .arg_required_else_help(true)
        .arg(
            Arg::new("app")
                .short('a')
                .help("search app name")
                .required(true)
                .takes_value(true),
        )
}
