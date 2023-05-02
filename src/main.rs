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
    my_dataset.shuffle();


    // my_dataset.filter_by_output(vec![0, 1]);
    // let mut classification_network:Network = Network::new(
    //     vec![784, 684, 584, 484, 392, 100,  1],
    //     true
    // );
    
    // THERES A SOFTMAX LAYER TOO, IDIOT!!!
    let mut classification_network:Network = Network::new(
        vec![784, 684, 584, 484, 392, 10, 10],
        false
    );



    classification_network.sgd(
        &mut my_dataset, 
        0.001, 
        30,
        0.01,
        10,
        10,
    );

    println!("{:#?}",classification_network.layers.last().expect("failed to get last layer mb mb"));


    let mut sum:f64 = 0.0;
    for neuron in 0..classification_network.layers[6].neurons.len() {
        println!("{:#?}", classification_network.layers[6].neurons[neuron].act());
        sum += classification_network.layers[6].neurons[neuron].act();
    }

    println!("SUM: {}", sum);


    for neuron in 0..classification_network.layers[5].neurons.len() {
        println!("{:#?}", classification_network.layers[5].neurons[neuron].act())
    }
    // classification_network.to_file(String::from("data/network.json"));
}