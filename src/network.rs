use crate::{layer::{Layer, self, LayerKind}, neuron::{Neuron, self}};
use crate::activation_functions::functions::{
    sigmoid,
    relu,
    derivative_sigmoid
};
use::math::mean;


pub struct Network {
    pub layers: Vec<Layer>,
}



impl Network {
    pub fn len(&self) -> usize {
        self.layers.len()
    }


    pub fn add_layer(&mut self, in_features:usize ,out_features:usize, kind:LayerKind) {
        self.layers.push(Layer::new(in_features, out_features, kind))
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

    pub fn new(layer_sizes:Vec<usize>) -> Network {
        let layers:Vec<Layer> = Vec::new();
        let mut network:Network = Network { layers: layers };
        for size_index in 0..layer_sizes.len() - 1
        {
            let last_index = layer_sizes.len() - 1;
            let kind:LayerKind = match size_index {
                0 => LayerKind::hidden_layer,
                _last_index=> LayerKind::output_layer,
                _=> LayerKind::hidden_layer
            };

            let mut prev_index:usize = 0 as usize;
            if size_index != 0 {
                prev_index = size_index - 1;
            } 

            network.layers.push(
                Layer::new(layer_sizes[prev_index],layer_sizes[size_index], kind)
            )
        }
        network.initialize();
        network
    }

    pub fn feedforward(&mut self) {
        for layer_index in 1..&self.layers.len()-1 {
            for neuron_index in 0..&self.layers[layer_index].neurons.len()-1 {
                self.layers[layer_index].neurons[neuron_index].activation = {
                    let mut sum:f64 = 0.0;
                    for weight_index in 0..self.layers[layer_index].neurons[neuron_index].weights.len()-1 {
                        sum += (self.layers[layer_index].neurons[neuron_index].weights[weight_index] * 
                            &self.layers[layer_index - 1].neurons[weight_index].activation);
                    }
                    sum += self.layers[layer_index].neurons[neuron_index].bias();
                    self.layers[layer_index].neurons[neuron_index].sum = sum;
                    sigmoid(sum)
                }
            }
        }
    }



    pub fn get_network_output(&self) -> i64 {
        match self.layers.last() {
            Some(lyr) => {
                let mut max_value:&Neuron = &lyr.neurons[0];
                for neuron_index in 1..lyr.neurons.len() - 1 {
                    if lyr.neurons[neuron_index].act() > max_value.act() {
                        max_value = &lyr.neurons[neuron_index];
                    }
                }
                return max_value.id() as i64;
            }
            None => {
                panic!("failed to retrieve last layer")
            }
        }
    }

    pub fn set_inputs(&mut self, pixels:&Vec<f64>) {
        let input_layer = &mut self.layers[0];
        for neuron_index in 0..input_layer.len() {
            input_layer.neurons[neuron_index].set_act(f64::from(pixels[neuron_index]));
        }
    }

}

