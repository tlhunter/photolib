// use std::env;
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("photolib")
        .about("photo library manager")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)

        .subcommand(
            Command::new("init")
                .about("Initialize a new photo library at the specified location.")
                .arg(
                    Arg::new("path")
                        .help("The path to initialize a library in. Safe to run in a directory that already contains photos.")
                        .action(ArgAction::Set)
                        .required(true)
                )
        )

        .subcommand(
            Command::new("check")
                .about("Check for library errors such as mismatched dates.")
                .arg(
                    Arg::new("namespace")
                        .help("Library, Library/Collection, or Library/Collection/Shoot")
                        .action(ArgAction::Set)
                        .required(false)
                )
        )

        .subcommand(
            Command::new("import")
                .about("Copies or moves files to a library")
                .arg(
                    Arg::new("source")
                        .help("A path to the source files to copy or move to the library")
                        .action(ArgAction::Set)
                        .required(true)
                )
                .arg(
                    Arg::new("move")
                        .short('m')
                        .long("move")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .help("By default files are copied. By setting this flag they are moved.")
                )
        )

        .subcommand(
            Command::new("dupes")
                .about("Check for photos duplicated across different shoots")
                .arg(
                    Arg::new("namespace")
                        .help("Library, Library/Collection, or Library/Collection/Shoot")
                        .action(ArgAction::Set)
                        .required(false)
                )
        )

        .subcommand(
            Command::new("report")
                .about("Provide library statistics such as focal length frequency")
                .arg(
                    Arg::new("namespace")
                        .help("Library, Library/Collection, or Library/Collection/Shoot")
                        .action(ArgAction::Set)
                        .required(false)
                )
        )

        .subcommand(
            Command::new("fix")
                .about("Prompt user to fix mismatched dates, delete duplicates, etc.")
                .arg(
                    Arg::new("namespace")
                        .help("Library, Library/Collection, or Library/Collection/Shoot")
                        .action(ArgAction::Set)
                        .required(false)
                )
        )

        .get_matches();
    dbg!(matches);
}
