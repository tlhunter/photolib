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
  extern crate rexiv2;

  #[test]
  fn raw_test_2() {
      let path = "/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.ARW";
      let meta = rexiv2::Metadata::new_from_path(path).unwrap();
      println!("{:?}", meta);
      println!("Exposure: {:?}", meta.get_tag_multiple_strings("Exif.Photo.ExposureTime"));
      println!("FStop: {:?}", meta.get_tag_multiple_strings("Exif.Photo.FNumber"));
      println!("Focal: {:?}", meta.get_tag_multiple_strings("Exif.Photo.FocalLength"));
      println!("Lens: {:?}", meta.get_tag_multiple_strings("Exif.Photo.LensModel"));
      println!("Make: {:?}", meta.get_tag_multiple_strings("Exif.Image.Make"));
      println!("Model: {:?}", meta.get_tag_multiple_strings("Exif.Image.Model"));
      println!("Date: {:?}", meta.get_tag_multiple_strings("Exif.Image.DateTime"));
      // println!("{:?}", meta.get_exif_tags());
  }
}