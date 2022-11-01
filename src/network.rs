use crate::{layer::{Layer, self, LayerKind}, neuron::{Neuron, self}};
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


    pub fn add_layer(&self, in_features:usize ,out_features:usize, kind:LayerKind) {
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
                last_index=> LayerKind::output_layer,
                _=> LayerKind::hidden_layer
            };

            network.layers.push(
                Layer::new(layer_sizes[size_index - 1],layer_sizes[size_index], kind)
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
                        sum += (self.layers[layer_index].neurons[neuron_index].weights[weight_index] * &self.layers[layer_index - 1].neurons[weight_index].activation);
                    }
                    sum += self.layers[layer_index].neurons[neuron_index].bias();
                    self.layers[layer_index].neurons[neuron_index].sum = sum;
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
        for neuron_id in 0..&self.layers[layer_id].neurons.len() - 1  {
            if neuron_id == target_id {
                cost_vec.push((desired_output - self.layers[layer_id].neurons[neuron_id].activation).powf(2.0));
            } else {
                cost_vec.push((0.0 - self.layers[layer_id].neurons[neuron_id].activation).powf(2.0));
            }
        }
        cost_vec
    }

    pub fn get_mse_error(&self, cost_vector:Vec<f64>) -> f64 {
        mean::arithmetic(&cost_vector[..])
    }

    pub fn backpropagate(&mut self, &desired_output_id: &usize, &desired_output: &f64, &layer_id: &usize) 
    {
        for neuron_index in 0..&self.layers[layer_id].neurons.len() - 1 
        {
            for weight_index in 0..&self.layers[layer_id].neurons[neuron_index].weights.len() - 1 
            {
                let prev_activation = &self.layers[layer_id - 1].neurons[weight_index].n_value;
                let output_size = match self.layers.last() 
                {
                    Some(lyr) => lyr.len(),
                    None => panic!("failed to get output layer")
                };
                let mut y = &0.0;
                if neuron_index == desired_output_id 
                {
                    y = &desired_output;
                }

                match &self.layers[layer_id].kind {
                    LayerKind::output_layer =>  {
                        self.layers[layer_id].neurons[neuron_index].weights[weight_index] -=  
                        &self.layers[layer_id].neurons[neuron_index].dC_over_dA(*y) * 
                        &self.layers[layer_id].neurons[neuron_index].dA_over_dZ() * prev_activation;  

                        self.layers[layer_id].neurons[neuron_index].n_bias -= 
                        &self.layers[layer_id].neurons[neuron_index].dC_over_dA(*y) * 
                        &self.layers[layer_id].neurons[neuron_index].dA_over_dZ() *
                        &self.layers[layer_id].neurons[neuron_index].dZ_over_dB()
                        
                    },

                    LayerKind::hidden_layer => {
                        let changeInActivation = {
                            let mut sum = 0.0;
                            for next_neuron_id in 0..self.layers[layer_id + 1].neurons.len() - 1 {
                                sum +=
                                &self.layers[layer_id + 1].neurons[next_neuron_id].weights[neuron_index] * 
                                self.layers[layer_id + 1].neurons[next_neuron_id].dA_over_dZ() *
                                self.layers[layer_id + 1].neurons[next_neuron_id].dC_over_dA(*y) //ASK JUN WHAT THE DESIRED OUTPUT IS
                            }
                            sum
                        };



                        self.layers[layer_id].neurons[neuron_index].weights[weight_index] -= 1.0
                    }
                    _=> {}
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

