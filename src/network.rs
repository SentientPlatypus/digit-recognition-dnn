use crate::{layer::{Layer}, neuron::Neuron};




struct Network {
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
        network
    }
}
