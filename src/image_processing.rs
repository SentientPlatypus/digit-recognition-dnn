use image::{self, imageops::*, Pixel, Pixels, Luma};
use serde_json::Value;
use core::num;
use std::collections::HashMap;
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
  correct_value: i64
}

pub struct data_set {
  images: Vec<number_img>
}

impl data_set {
  pub fn generate(path:String) -> data_set{
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read data");
    
    //DATA IS JSON STRING.
    type JsonMap = HashMap<String, serde_json::Value>;
    let data_json:HashMap<String, Vec<JsonMap>> = serde_json::from_str(&data).expect("Failed to parse json");
    let mut img_vector:Vec<number_img> = Vec::new();

    for image_data in &data_json["data"] {
      let mut pixels:Vec<f64> = Vec::new();
      for pixel in image_data["vector"].as_array().expect("failed to parse as array") {
        pixels.push(pixel.as_f64().expect("failed to turn into f64"))
      }
      img_vector.push(
        number_img { 
          pixel_brightness: pixels, 
          correct_value: image_data["y"].as_i64().expect("failed to turn into i64")
        }
      )
    }
    println!("SUCCESS ");
    data_set { images:img_vector}
  }
}

