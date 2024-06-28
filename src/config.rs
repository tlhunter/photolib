use ini::configparser::ini::Ini;

use xdg::BaseDirectories;

fn get_base_directories() -> BaseDirectories {
    return BaseDirectories::with_prefix("photolib").unwrap();
}

pub fn create_config_file() {
    // TODO: If already configured then don't clobber config file
    let config_path = get_base_directories()
        .place_config_file("config.ini")
        .expect("cannot create configuration directory");
    let mut conf = Ini::new();
    conf.set("libraries", "default", Some(String::from("~/Photos")));
    conf.write(config_path.to_str().unwrap()).unwrap();
}

pub fn add_library(name: String, path: String) {}

pub fn remove_library(name: String) {}
