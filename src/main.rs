pub mod network;
pub mod layer;
pub mod neuron;
pub mod dendrite;
pub mod activation_functions;
pub mod image_processing;

use crate::activation_functions::functions::{
    sigmoid,
    relu
};





fn main() {
    let mut network: network::Network = network::Network::build(vec![28 * 28, 16, 16, 8]);
    let img_gray_vec: Vec<u8> = image_processing::convert_to_grayscale();
    network.set_inputs(img_gray_vec);
}

