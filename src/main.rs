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
    let network = network::Network::build(vec![28 * 28, 16, 16, 8]);

}

