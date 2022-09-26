use crate::{layer::{Layer}, neuron::Neuron};
use rand::Rng;




struct Network {
    layers: Vec<Layer>,
}

impl Network {
    
    //initializes network
    pub fn initialize(&mut self){
        (0..self.layers.len()-2).for_each(|layer_index: usize| 
        {
            (0..self.layers[layer_index].neurons.len()).for_each(|neuron_index: usize| 
            {
                self.layers[layer_index + 1].neurons.iter_mut().for_each(|to_neuron: &mut Neuron| 
                {
                    to_neuron.generate_random_weights();
                });
            });
        });   
    }

    pub fn build(layer_sizes:Vec<usize>) -> Network {
        let layers:Vec<Layer> = Vec::new();
        let mut network:Network = Network { layers: layers };
        (0..layer_sizes).for_each(|n:usize|
        {
            network.layers.push(
                Layer::build(n)
            )
        });
        network
    }
}
