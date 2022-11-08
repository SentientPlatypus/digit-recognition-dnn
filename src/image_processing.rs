use image::{self, imageops::*, Pixel, Pixels, Luma};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use std::io::BufReader;
use std::path::Path;

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

pub struct data_set {
  images: Vec<number_img>
}

impl data_set {
  pub fn generate(path:String) {
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read data");
    
    //DATA IS JSON STRING.

    let data_json:serde_json::Value = serde_json::from_str(&data).expect("Failed to parse json");
    print!("{}",data_json["data"][0])

  }
}

