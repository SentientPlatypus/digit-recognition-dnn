pub mod network;
pub mod layer;
pub mod neuron;
pub mod activation_functions;
pub mod image_processing;
use image_processing::Dataset;
use image_processing::NumberImg;
use network::Network;
use textplots::{Chart, Plot, Shape};




fn main() {
    let mut my_dataset:Dataset = Dataset::generate_full(String::from("data/data.json"));
    my_dataset.shuffle();
    my_dataset.filter_by_output(vec![0, 1]);


    let mut classification_network:Network = Network::new(
        vec![784, 684, 584, 484, 392, 1],
        true
    );





    // classification_network.set_inputs(&my_dataset.images[0].pixel_brightness);
    // classification_network.feedforward();
    // println!("{:#?}", classification_network.layers.last().expect("Failed to get output layer"));
    // println!("Correct value: {:#?}", my_dataset.images[0].correct_value);
    // println!("predicted value: {:#?}", classification_network.get_network_output());
    // println!("{:#?}", classification_network.get_network_cost(my_dataset.images[0].correct_value));

    classification_network.sgd(
        &mut my_dataset, 
        0.0001, 
        100,
        0.1,
        10,
        10,
        10,
        true
    );
}

