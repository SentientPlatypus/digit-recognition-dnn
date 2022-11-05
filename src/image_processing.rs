use image::{self, imageops::*, Pixel, Pixels, Luma};
use std::error::Error;
use csv::ReaderBuilder;


pub fn convert_to_grayscale() -> Vec<u8> {
  let img = match image::open("num.jpeg")
  {
    Ok(t) => {
      t
    }
    Err(_) => {
      panic!("Failed in opening");
    }
  };
  let gray_img = img.to_luma8();

  let mut pixel_vector = Vec::new();
  gray_img.pixels().for_each(|pixel| {
    pixel_vector.push(pixel.0[0]);
  });
  pixel_vector
}


struct number_img {
  pixel_brightness:Vec<f64>,
  correct_value: u8
}

struct data_set {
  images: Vec<number_img>
}

impl data_set {
  pub fn generate(path:String) {
    let mut rdr = Reader::from_path("foo.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
  }
}

