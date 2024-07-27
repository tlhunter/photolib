// use std::env;
mod commands;
mod config;
mod dispatcher;

fn main() {
    let cmd = commands::get_commands();
    dispatcher::dispatch(cmd);
}

#[cfg(test)]
mod tests {
  #[test]
  fn raw_test() {
    // so this library doesn't support RAW files from my 5 year old camera.
    // probably going to need to use this C++ library instead.
    // gonna have to tumble down the rust rabbit hole pretty deep.
    // https://github.com/felixc/rexiv2/blob/main/SETUP.md
    let path = "/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.ARW";
    let image = rawloader::decode_file(path).unwrap();
    dbg!(image.width);
    assert_eq!(image.width, 123);
  }
}