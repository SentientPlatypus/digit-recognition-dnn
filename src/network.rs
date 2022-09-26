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

    pub fn build() -> Network {
        let layers:Vec<Layer> = Vec::new();
        let network:Network = Network { layers: layers };
        network
    }
}
