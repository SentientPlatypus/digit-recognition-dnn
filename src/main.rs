pub mod network;
pub mod layer;
pub mod neuron;
pub mod activation_functions;
pub mod image_processing;
use image_processing::Dataset;
use image_processing::NumberImg;
use network::Network;

fn main() {
    let my_dataset:Dataset = Dataset::generate_first(String::from("data/data.json"));

    let img:&NumberImg = &my_dataset.images[0];
    
    let mut classification_network:Network = Network::new(vec![img.pixel_brightness.len(), 8, 8, 10]);
    classification_network.set_inputs(&img.pixel_brightness);

    classification_network.feedforward();
    println!("{:?}", classification_network.get_network_output());
    println!("{:#?}", classification_network.layers.last().expect("failed to get last layer"));
}



// fn main() {
//     let mut network: network::Network = network::Network::new(vec![28 * 28, 16, 16, 8]);
//     let img_gray_vec: Vec<u8> = image_processing::convert_to_grayscale();
//     network.set_inputs(img_gray_vec);
// }

