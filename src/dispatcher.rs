use clap::ArgMatches;

use crate::config;

pub fn dispatch(cmd: ArgMatches) {
    match cmd.subcommand() {
        Some(("init", _matches)) => {
            config::create_config_file();
        }

        Some(("new", new_matches)) => {
            let path: &String = new_matches.get_one("path").expect("path is required");
            dbg!(path);
        }

        _ => unreachable!(),
    }
}
