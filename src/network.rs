use crate::{layer::{Layer}, neuron::Neuron};
use rand::Rng;




struct Network {
    layers: Vec<Layer>,
    net_total_layers: u8,
    net_inputs: Vec<i8>,
    net_outputs: Vec<i8>,
    net_layers:Vec<u8>,
    net_learning_rate: i8
}

impl Network {
    
    //initializes network
    fn initialize(&mut self){
        for layer_index in 0..self.layers.len()-2 
        {
            for neuron_index in 0..self.layers[layer_index].neurons.len() 
            {
                let neuron: &mut Neuron = &mut self.layers[layer_index].neurons[neuron_index];
                self.layers[layer_index + 1].neurons.iter_mut().for_each(|to_neuron| {
                    to_neuron.generate_random_weights();
                });
            }
        }   
    }
}
