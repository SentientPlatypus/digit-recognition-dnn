pub mod network;
pub mod layer;
pub mod neuron;
pub mod activation_functions;
pub mod image_processing;
use image_processing::data_set;
use image_processing::number_img;
use network::Network;


use crate::activation_functions::functions::{
    sigmoid,
    relu
};

fn main() {
    let my_dataset:data_set = data_set::generate(String::from("data/data.json"));

    let img:&number_img = &my_dataset.images[0];
    
    let mut classification_network:Network = Network::new(vec![img.pixel_brightness.len(), 8, 8, 10]);
    classification_network.set_inputs(&img.pixel_brightness)
}



// fn main() {
//     let mut network: network::Network = network::Network::new(vec![28 * 28, 16, 16, 8]);
//     let img_gray_vec: Vec<u8> = image_processing::convert_to_grayscale();
//     network.set_inputs(img_gray_vec);
// }

