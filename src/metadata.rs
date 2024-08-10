extern crate rexiv2;
use regex::Regex;
use std::fs;

/**
 * Regarding dates: AFAICT, EXIF dates are in the local time zone that the photo
 * was taken in, and a timezone offset is also available. photolib doesn't really
 * ever need to know when the timezone is since A) it's not like a person is taking
 * photos simultaneously in multiple TZ and B) we don't ever need to compare times or
 * otherwise order entries by precise dates.
 */

pub struct PhotoLibMetadata {
  date: String, // YYYY-MM-DD
  // timezone: Option<String>,
  camera: String,
  lens: Option<String>, // The Lumix GX1 doesn't provide this
  focal: String,
  aperture: String,
  shutter: String,
  iso: String,
  rating: String,
}

impl PhotoLibMetadata {
  pub fn new(path: &str) -> PhotoLibMetadata {
    let meta = rexiv2::Metadata::new_from_path(path).unwrap();

    let camera_make = meta.get_tag_string("Exif.Image.Make").unwrap();

    // a camera might only provide the make but no model? TODO test case
    let camera = match meta.get_tag_string("Exif.Image.Model").ok() {
      Some(camera_model) => format!("{camera_make} {camera_model}"),
      None => camera_make,
    };

    PhotoLibMetadata {
      date: convert_exif_date(meta.get_tag_string("Exif.Image.DateTime").unwrap()),
      // timezone: meta.get_tag_string("Exif.Photo.OffsetTime").ok(),
      camera,
      lens: meta.get_tag_string("Exif.Photo.LensModel").ok(),
      focal: meta.get_tag_string("Exif.Photo.FocalLength").unwrap(),
      aperture: meta.get_tag_string("Exif.Photo.FNumber").unwrap(),
      shutter: meta.get_tag_string("Exif.Photo.ExposureTime").unwrap(),
      iso: meta.get_tag_string("Exif.Photo.ISOSpeedRatings").unwrap(),
      rating: meta.get_tag_string("Exif.Image.Rating").unwrap_or_default(),
    }
  }

  pub fn to_string(&self) -> String {
    return format!(
      "Date: {}\nRating: {}\nCamera: {}\nLens: {}\nDetails: {} {} {} {}",
      self.date,
      self.rating,
      self.camera,
      match &self.lens {
        None => "Unknown".to_owned(),
        Some(lens) => lens.to_string(),
      },
      self.get_flength_display(),
      self.get_aperture_display(),
      self.get_shutter_display(),
      self.get_iso_display()
    );
  }

  // focal length is in form of "500/10" meaning 500 / 10 and should be displayed as 50mm
  pub fn get_flength_display(&self) -> String {
    let focal = exif_string_division(self.focal.to_string());
    return format!("{focal}mm");
  }

  pub fn get_aperture_display(&self) -> String {
    let aperture = exif_string_division(self.aperture.to_string());
    return format!("f/{aperture}");
  }

  // bypassing exif_string_division as 1/200 looks better than 0.005
  // TODO: Need to simplify fractions as GX1 provides 10/2000 
  pub fn get_shutter_display(&self) -> String {
    // let val = exif_string_division(self.shutter.to_string());
    let shutter = self.shutter.to_string();
    return format!("{shutter}s");
  }

  pub fn get_iso_display(&self) -> String {
    let iso = &self.iso;
    return format!("ISO{iso}");
  }
}

/*
pub struct PhotoRating {
  rejected: bool,
  rated: bool,
  rating: u8, // should normally be 1 - 5, 0 means the other checks should have been used
}

impl PhotoRating {
  pub fn new(rating: Result<String>) -> PhotoRating {
    let Ok(rating) = rating.parse::<i8>() else {
      return PhotoRating{
        rejected: false,
        rated: false,
        rating: 0,
      };
    };

    if rating <= -1 {
      return PhotoRating{
        rejected: true,
        rated: true,
        rating: 0,
      };
    } else if rating == 0 {
      return PhotoRating{
        rejected: false,
        rated: false,
        rating: 0,
      };
    } else if rating > 5 {
      return PhotoRating{
        rejected: false,
        rated: true,
        rating: 5,
      };
    } else {
      return PhotoRating{
        rejected: false,
        rated: true,
        rating: rating as u8,
      };
    }
  }
  pub fn to_string(&self) -> String {
    if !self.rated {
      return "unrated".to_owned();
    } else if self.rejected {
      return "rejected".to_owned();
    } else {
      return format!(
        "{} / 5",
        self.rating
      );
    }
  }
}
  */

fn exif_string_division(input: String) -> String {
    match input.split_once('/') {
      None => "Unknown".to_owned(),
      Some(parts) => {
        let numerator = parts.0.parse::<f32>().unwrap();
        let denominator = parts.1.parse::<f32>().unwrap();
        return (numerator / denominator).to_string();
      },
    }
}

// YYYY:MM:DD HH:MM:SS -> YYYY-MM-DD
fn convert_exif_date(input: String) -> String {
  let input = input.replacen(':', "-", 2);
  let converted = (input.split_once(' ').unwrap()).0;
  return converted.to_string();
}

// TODO: this should be parsed as XML instead of using regex
// -1 = user actively chose to reject this photo. photolib considers this delete worthy.
//  0 = unrated?
//  1 = user decided this photo is meh
//  5 = user decided this photo deserves the highest rating
pub fn extract_rating_from_xmp(path: &str)-> Option<i8> {
  let contents = fs::read_to_string(path).unwrap();
  let re = Regex::new(r#"xmp:Rating="(?<rating>.+)""#).unwrap();
  let Some(captures) = re.captures(&contents) else {
    return None;
  };
  let rating: [&str; 1] = captures.extract().1;
  let Ok(rating) = rating[0].parse::<i8>() else {
    return None;
  };

  return Some(rating);
}

#[cfg(test)]
mod tests {
  use crate::metadata::PhotoLibMetadata;
  use crate::metadata::extract_rating_from_xmp;

  #[test]
  fn raw_test_sony_arw() {
    println!("{}", PhotoLibMetadata::new("/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.ARW").to_string());
  }

  #[test]
  fn raw_test_sony_jpeg() {
    println!("{}", PhotoLibMetadata::new("/home/tlhunter/Photographs/San Francisco/2024-02-24 South SF Flickr Walk a7rIV/TLH01595.JPG").to_string());
  }

  #[test]
  fn raw_test_lumix_rw2() {
    println!("{}", PhotoLibMetadata::new("/home/tlhunter/Photographs/P1120849.RW2").to_string());
  }

  #[test]
  fn xmp_test() {
    println!("Rating: {}", extract_rating_from_xmp("/home/tlhunter/Photographs/Potrero Hill/2024-02-22 Marine One a7ii 40mm/DSC00151.ARW.xmp").unwrap());
  }
}
