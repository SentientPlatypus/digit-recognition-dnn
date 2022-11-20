pub mod network;
pub mod layer;
pub mod neuron;
pub mod activation_functions;
pub mod image_processing;
use image_processing::Dataset;
use image_processing::NumberImg;
use network::Network;

fn main() {
    let mut my_dataset:Dataset = Dataset::generate_full(String::from("data/data.json"));
    my_dataset.filter_by_output(vec![0, 1]);


    let mut classification_network:Network = Network::new(
        vec![784, 684, 584, 484, 392, 1],
        true
    );


    println!("{:#?}", classification_network.layers.last().expect("Failed to get output layer"));
    
    classification_network.sgd(
        &my_dataset, 
        0.0001, 
        20,
        0.1
    );

}

