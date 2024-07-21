use clap::ArgMatches;

use crate::config;
use std::{default, path::Path};

pub fn dispatch(cmd: ArgMatches) {
    match cmd.subcommand() {
        Some(("init", _matches)) => {
            config::create_config_file();
        }

        Some(("new", new_matches)) => {
            let path: &String = new_matches.get_one("path").expect("path is required");
            let path = Path::new(path);
            let default_alias = path
                .components()
                .last()
                .expect("fuck")
                .as_os_str()
                .to_str()
                .expect("shit");
            let alias: &String = match new_matches.get_one("alias") {
                None => &default_alias.into(),
                Some(a) => a,
            };
            dbg!(path);
            dbg!(alias);
        }

        _ => unreachable!(),
    }
}
