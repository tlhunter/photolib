// use std::env;
mod commands;
mod config;
mod dispatcher;
mod metadata;

fn main() {
    let cmd = commands::get_commands();
    dispatcher::dispatch(cmd);
}