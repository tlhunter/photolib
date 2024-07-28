extern crate rexiv2;

pub struct PhotoLibMetadata {
  date: String,
  // date_offset: Option<String>,
  camera: String,
  lens: Option<String>, // The Lumix GX1 doesn't provide this
  flength: String,
  fstop: String,
  exposure: String,
  iso: String,
}

impl PhotoLibMetadata {
  pub fn new(path: &str) -> PhotoLibMetadata {
    let meta = rexiv2::Metadata::new_from_path(path).unwrap();

    let camera_make = meta.get_tag_string("Exif.Image.Make").unwrap();
    let camera_model = meta.get_tag_string("Exif.Image.Model").ok();

    // a camera might only provide the make but no model? TODO test case
    let camera = match camera_model {
      Some(model) => format!("{camera_make} {model}"),
      None => camera_make,
    };

    PhotoLibMetadata {
      date: meta.get_tag_string("Exif.Image.DateTime").unwrap(),
      // date_offset: meta.get_tag_string("Exif.Photo.OffsetTime").ok(),
      camera,
      lens: meta.get_tag_string("Exif.Photo.LensModel").ok(),
      flength: meta.get_tag_string("Exif.Photo.FocalLength").unwrap(),
      fstop: meta.get_tag_string("Exif.Photo.FNumber").unwrap(),
      exposure: meta.get_tag_string("Exif.Photo.ExposureTime").unwrap(),
      iso: meta.get_tag_string("Exif.Photo.ISOSpeedRatings").unwrap(),
    }
  }

  // focal length is in form of "500/10" meaning 500 / 10 and should be displayed as 50mm
  pub fn get_flength_display(&self) -> String {
    let val = exif_string_division(self.flength.to_string());
    return format!("{val}mm");
  }

  pub fn get_fstop_display(&self) -> String {
    let val = exif_string_division(self.fstop.to_string());
    return format!("f/{val}");
  }

  pub fn get_exposure_display(&self) -> String {
    let val = exif_string_division(self.exposure.to_string());
    return format!("{val}s");
  }

  pub fn get_iso_display(&self) -> String {
    let iso = &self.iso;
    return format!("ISO{iso}");
  }
}

fn exif_string_division(input: String) -> String {
    match input.split_once('/') {
      None => "Unknown".to_owned(),
      Some(parts) => {
        let numerator = parts.0.parse::<i32>().unwrap();
        let denominator = parts.1.parse::<i32>().unwrap();
        return (numerator / denominator).to_string();
      },
    }
}


#[cfg(test)]
mod tests {
  use crate::metadata::PhotoLibMetadata;

  #[test]
  fn raw_test_sony_arw() {
      let meta = PhotoLibMetadata::new("/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.ARW");
      println!("Date: {:?}", meta.date);
      println!("Camera: {:?}", meta.camera);
      println!("Lens: {:?}", meta.lens);
      println!("Details: {:?} f/{:?} {:?}s ISO{:?}",
        meta.get_flength_display(),
        meta.get_fstop_display(),
        meta.get_exposure_display(),
        meta.get_iso_display()
      );
  }

  #[test]
  fn raw_test_sony_jpeg() {
      let meta = PhotoLibMetadata::new("/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.JPG");
      println!("Date: {:?}", meta.date);
      println!("Camera: {:?}", meta.camera);
      println!("Lens: {:?}", meta.lens);
      println!("Details: {:?} f/{:?} {:?}s ISO{:?}",
        meta.get_flength_display(),
        meta.get_fstop_display(),
        meta.get_exposure_display(),
        meta.get_iso_display()
      );
  }
  #[test]
  fn raw_test_lumix_rw2() {
      let meta = PhotoLibMetadata::new("/home/tlhunter/Photographs/P1120849.RW2");
      println!("Date: {:?}", meta.date);
      println!("Camera: {:?}", meta.camera);
      println!("Lens: {:?}", meta.lens);
      println!("Details: {:?} f/{:?} {:?}s ISO{:?}",
        meta.get_flength_display(),
        meta.get_fstop_display(),
        meta.get_exposure_display(),
        meta.get_iso_display()
      );
  }
}
