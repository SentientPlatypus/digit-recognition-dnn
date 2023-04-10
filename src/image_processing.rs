use itertools::Itertools;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct NumberImg {
  pub pixel_brightness:Vec<f64>,
  pub correct_value: i64,
  pub true_vec: Vec<f64>
}

fn generate_true_vec(id:usize) -> Vec<f64> {
  let mut output:Vec<f64> = Vec::new();
  for n in 0 as usize..=9 {
    if n == id {
      output.push(1.0);
    } else {
        output.push(0.0);
    }
  }
  output
}

pub struct Dataset {
  pub images: Vec<NumberImg>
}

impl Dataset {

  ///Returns a dataset with ALL the images
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
          correct_value: image_data["y"].as_i64().expect("failed to turn into i64"),
          true_vec: generate_true_vec(image_data["y"].as_i64().expect("failed to turn into i64") as usize)
        }
      )
    }
    println!("SUCCESS ");
    Dataset { images:img_vector}
  }

  ///Creates a Dataset of only the first image in the json file
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
        correct_value: img_data["y"].as_i64().expect("failed to turn into i64"),
        true_vec: generate_true_vec(img_data["y"].as_i64().expect("failed to turn into i64") as usize)
      }
    );

    Dataset { images: img_vector }

  }


  /// Retains only the images with a correct_value of the numbers in to_retain
  /// 
  pub fn filter_by_output(&mut self, to_retain:Vec<i64>) {
    self.images.retain(|x| to_retain.contains(&x.correct_value))
  }


  pub fn shuffle(&mut self) {
    ///shuffles the dataset.
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    self.images.shuffle(&mut rng);
  }

  pub fn random_choice(&self) -> &NumberImg{
    ///gets a random image from the dataset.
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    self.images.choose(&mut rng).expect("failed to get random image")
  }

}

