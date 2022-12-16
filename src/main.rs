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
    my_dataset.filter_by_output(vec![0, 1]);
    let _first_img:&NumberImg = &my_dataset.images[0];


    let mut classification_network:Network = Network::new(
        vec![784, 684, 584, 484, 392, 1],
        true
    );




    let random_numberimg = my_dataset.random_choice();
    classification_network.set_inputs(&random_numberimg.pixel_brightness);
    classification_network.feedforward();
    println!("Correct value: {:#?}", random_numberimg.correct_value);
    println!("predicted value: {:#?}", classification_network.get_network_output());
    println!("{:#?}", classification_network.get_network_cost(random_numberimg.correct_value));
    println!("{:#?}", classification_network.layers.last().expect("Failed to get output layer"));
    classification_network.backpropagate(random_numberimg, 0.001, 0.1, 1);
    classification_network.feedforward();
    println!("{:#?}", classification_network.layers[4]);


    // classification_network.sgd(
    //     &mut my_dataset, 
    //     0.001, 
    //     50,
    //     0.1,
    //     10,
    //     10,
    // );
}

