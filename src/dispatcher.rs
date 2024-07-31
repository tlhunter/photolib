use clap::ArgMatches;

use crate::{config, metadata::PhotoLibXmp};
use std::path::Path;

pub fn dispatch(cmd: ArgMatches) {
    match cmd.subcommand() {
        Some(("init", _matches)) => {
            config::create_config_file();
        }

        Some(("info", new_matches)) => {
            let path: &String = new_matches.get_one("path").expect("path is required");
            let image = crate::metadata::PhotoLibMetadata::new(path);
            println!("{}", image.to_string());

            // let meta = PhotoLibXmp::new("/home/tlhunter/Photographs/Potrero Hill/2024-02-22 Marine One a7ii 40mm/DSC00151.ARW.xmp");
            // println!("{:?}", meta);
        }

        Some(("new", new_matches)) => {
            let path: &String = new_matches.get_one("path").expect("path is required");
            let path = Path::new(path);
            let alias: &String = match new_matches.get_one("alias") {
                None => &path
                    .components()
                    .last()
                    .expect("unable to get new library alias")
                    .as_os_str()
                    .to_str()
                    .expect("unable to parse new library alias")
                    .into(),
                Some(given_alias) => given_alias,
            };
            dbg!(path);
            dbg!(alias);
            // add to config.ini file
        }

        _ => unreachable!(),
    }
}
