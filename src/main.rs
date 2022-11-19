pub mod network;
pub mod layer;
pub mod neuron;
pub mod activation_functions;
pub mod image_processing;
use image_processing::Dataset;
use image_processing::NumberImg;
use network::Network;

fn main() {
    let my_dataset:Dataset = Dataset::generate_full(String::from("data/data.json"));
    let img:&NumberImg = &my_dataset.images[0];

    let mut classification_network:Network = Network::new(
        vec![img.pixel_brightness.len(), 8, 8, 10]
    );
    
    classification_network.sgd(
        &my_dataset, 
        0.0001, 
        20,
        0.1
    );

}

