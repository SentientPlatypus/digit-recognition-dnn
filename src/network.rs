use crate::{layer::{Layer, self}, neuron::Neuron};
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
        for layer_index in 0..self.layers.len()-1 
        {
            for neuron_index in 0..self.layers[layer_index].neurons.len() 
            {
                let neuron:Neuron = self.layers[layer_index].neurons[neuron_index];
                for to_neuron in self.layers[layer_index + 1].neurons 
                {
                    to_neuron.weights = 
                }
            }
        }   
    }
}
