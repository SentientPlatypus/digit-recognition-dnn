use crate::{layer::{Layer, self}, neuron::Neuron};
use itertools::Itertools;
use crate::activation_functions::functions::{
    sigmoid,
    relu
};


pub struct Network {
    layers: Vec<Layer>,
}



impl Network {
    pub fn len(&self) -> usize {
        self.layers.len()
    }
    //initializes network
    pub fn initialize(&mut self){
        (0..self.len()-2).for_each(|layer_index: usize| 
        {
            (0..self.layers[layer_index].len()).for_each(|neuron_index: usize| 
            {
                self.layers[layer_index + 1].neurons.iter_mut().for_each(|to_neuron: &mut Neuron| 
                {
                    to_neuron.generate_random_weight(neuron_index);
                });
            });
        });   
    }

    pub fn build(layer_sizes:Vec<usize>) -> Network {
        let layers:Vec<Layer> = Vec::new();
        let mut network:Network = Network { layers: layers };
        for size in layer_sizes
        {
            network.layers.push(
                Layer::build(size)
            )
        }
        network.initialize();
        network
    }

    pub fn feedforward(&mut self) {
        for layer_index in 1..&self.layers.len()-1 {
            for neuron_index in 0..&self.layers[layer_index].neurons.len()-1 {
                self.layers[layer_index].neurons[neuron_index].n_value = {
                    let mut sum:f64 = 0.0;
                    for weight_index in 0..self.layers[layer_index].neurons[neuron_index].weights.len()-1 {
                        sum += (self.layers[layer_index].neurons[neuron_index].weights[weight_index] * &self.layers[layer_index - 1].neurons[weight_index].n_value);
                    }
                    sum += self.layers[layer_index].neurons[neuron_index].n_bias;
                    sigmoid(sum)
                }
            }
        }
    }
    /* pub fn feedforward(&mut self) {
    //     for layer_index in 1..&self.layers.len()-1 {
    //         for neuron in &mut self.layers[layer_index].neurons {
    //             neuron.n_value = {
    //                 let mut sum:f64 = 0.0;
    //                 for weight_index in 0..neuron.weights.len()-1 {
    //                     sum += (neuron.weights[weight_index] * &self.layers.tuplewindows()[layer_index - 1].neurons[weight_index].n_value);
    //                 }
    //                 sum += neuron.n_bias;
    //                 sigmoid(sum)
    //             }
    //         }
    //     }
    // }
    */

}
