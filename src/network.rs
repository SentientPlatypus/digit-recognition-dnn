use crate::{layer::{Layer, self}, neuron::{Neuron, self}};
use crate::activation_functions::functions::{
    sigmoid,
    relu,
    derivative_sigmoid
};
use::math::mean;


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
                    self.layers[layer_index].neurons[neuron_index].n_sum = sum;
                    sigmoid(sum)
                }
            }
        }
    }

    pub fn get_output_neuron(&self) -> &Neuron {
        match self.layers.last() {
            Some(lyr) => {
                lyr.get_max_neuron()
            },
            None => {
                panic!("failed to get output layer");
            }
        }
    }

    pub fn get_cost_vector(&self, layer_id:usize, target_id:usize, desired_output:f64) -> Vec<f64> {
        let mut cost_vec:Vec<f64> = Vec::new();
        for neuron in &self.layers[layer_id].neurons  {
            if neuron.n_id == target_id {
                cost_vec.push((desired_output - neuron.n_value).powf(2.0));
            } else {
                cost_vec.push((0.0 - neuron.n_value).powf(2.0));
            }
        }
        cost_vec
    }

    pub fn get_mse_error(&self, cost_vector:Vec<f64>) -> f64 {
        mean::arithmetic(&cost_vector[..])
    }

    pub fn determine_weight_change(&mut self, desired_output_id:usize, desired_output:f64, layer_id:usize) {
        for neuron_index in 0..&self.layers[layer_id].neurons.len() - 1 {
            for weight_index in 0..&self.layers[layer_id].neurons[neuron_index].weights.len() - 1 {
                let prev_activation = &self.layers[layer_id - 1].neurons[weight_index].n_value;
                let output_size = match self.layers.last() {
                    Some(lyr) => lyr.len(),
                    None => panic!("failed to get output layer")
                };
                {
                    let neuron = &self.layers[layer_id].neurons[neuron_index];
                    self.layers[layer_id].neurons[neuron_index].weights[weight_index] -= (2.0 * (neuron.n_value - neuron.n_bias)) * derivative_sigmoid(neuron.n_sum) * prev_activation;     
                }
            }       
        }
    }

    pub fn set_inputs(&mut self, pixels:Vec<u8>) {
        let input_layer = &mut self.layers[0];
        for neuron_index in 0..input_layer.len() {
            input_layer.set_neuron_val(f64::from(pixels[neuron_index]), neuron_index);
        }
    }

}
