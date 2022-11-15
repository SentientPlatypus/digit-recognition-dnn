use itertools::Itertools;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


pub struct NumberImg {
  pub pixel_brightness:Vec<f64>,
  pub correct_value: i64
}

pub struct Dataset {
  pub images: Vec<NumberImg>
}

impl Dataset {
  pub fn generate_full(path:String) -> Dataset{
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read data");
    
    //DATA IS JSON STRING.
    type JsonMap = HashMap<String, serde_json::Value>;
    let data_json:HashMap<String, Vec<JsonMap>> = serde_json::from_str(&data).expect("Failed to parse json");
    let mut img_vector:Vec<NumberImg> = Vec::new();

    for image_data in &data_json["data"] {    
      let pixels:Vec<f64> = image_data["vector"]
        .as_array()
        .expect("failed to parse as array")
        .into_iter()
        .map(|x| x.as_f64().expect("failed to turn into f64"))
        .collect_vec();

      img_vector.push(
        NumberImg { 
          pixel_brightness: pixels, 
          correct_value: image_data["y"].as_i64().expect("failed to turn into i64")
        }
      )
    }
    println!("SUCCESS ");
    Dataset { images:img_vector}
  }

  pub fn generate_first(path:String) -> Dataset {
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read data");
    //DATA IS JSON STRING.
    type JsonMap = HashMap<String, serde_json::Value>;
    let data_json:HashMap<String, Vec<JsonMap>> = serde_json::from_str(&data).expect("Failed to parse json");

    let mut img_vector:Vec<NumberImg> = Vec::new();

    let img_data = &data_json["data"][0];

    let pixels:Vec<f64> = img_data["vector"]
      .as_array()
      .expect("failed to parse as array")
      .into_iter()
      .map(|x:&Value| x.as_f64().expect("failed to turn into f64"))
      .collect_vec();


    img_vector.push(
      NumberImg { 
        pixel_brightness: pixels, 
        correct_value: img_data["y"].as_i64().expect("failed to turn into i64")
      }
    );

    Dataset { images: img_vector }

  }
}

