// use std::env;
mod commands;
mod config;

fn main() {
    let cmd = commands::get_commands();
    dbg!(cmd);
    config::create_config_file();
}
