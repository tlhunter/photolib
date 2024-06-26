use std::{fs::File, io::Write};

use xdg::BaseDirectories;

fn get_base_directories() -> BaseDirectories {
    return BaseDirectories::with_prefix("photolib").unwrap();
}

pub fn create_config_file() {
    let config_path = get_base_directories()
        .place_config_file("config.ini")
        .expect("cannot create configuration directory");
    println!("{:?}", config_path);
    let mut config_file = File::create(config_path).unwrap();
    write!(&mut config_file, "configured = 1").unwrap();
}
