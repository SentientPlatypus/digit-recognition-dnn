use crate::{layer::{Layer, LayerKind}, neuron::{Neuron, }, image_processing::NumberImg};
use crate::activation_functions::functions::{
    sigmoid
};


pub struct Network {
    pub layers: Vec<Layer>,
}



impl Network {

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn new(layer_sizes:Vec<usize>) -> Network {
        let mut network:Network = Network { layers: Vec::new() };
        for size_index in 0..layer_sizes.len()
        {
            let last_index = layer_sizes.len() - 1;

            let kind:LayerKind = match size_index {
                0 => LayerKind::InputLayer,
                last_index=> LayerKind::OutputLayer,
                _=> LayerKind::HiddenLayer
            };

            let mut in_features:usize = 0 as usize;
            let mut out_features:usize = 0 as usize;
            if size_index != 0 {
                in_features = layer_sizes[size_index - 1];
            } 
            if size_index != last_index {
                out_features = layer_sizes[size_index + 1];
            }
            network.add_layer(in_features, out_features, layer_sizes[size_index], kind);
        }
        network.initialize();
        network
    }

    pub fn add_layer(&mut self, in_features:usize ,out_features:usize,size:usize, kind:LayerKind) {
        self.layers.push(Layer::new(in_features, out_features, size, kind))
    }

    pub fn set_inputs(&mut self, pixels:&Vec<f64>) {
        let input_layer = &mut self.layers[0];
        for neuron_index in 0..input_layer.len() {
            input_layer.neurons[neuron_index].set_act(f64::from(pixels[neuron_index]));
        }
    }

    //initializes network
    pub fn initialize(&mut self){
        (1..self.layers.len()).for_each(|layer_index: usize| 
        {
            (0..self.layers[layer_index].neurons.len()).for_each(|neuron_index: usize| 
            {
                for count in 0..self.layers[layer_index].in_features{
                    self.layers[layer_index].neurons[neuron_index].generate_random_weight(count)
                }
            });
        });   
    }

    pub fn feedforward(&mut self) {
        for layer_index in 1..self.layers.len() {
            for neuron_index in 0..self.layers[layer_index].neurons.len() {
                let mut sum:f64 = 0.0;

                for weight_index in 0..self.layers[layer_index].neurons[neuron_index].weights.len(){
                    sum += 
                        self.layers[layer_index].neurons[neuron_index].weights[weight_index] * 
                        self.layers[layer_index - 1].neurons[weight_index].act();
                }
                sum += self.layers[layer_index].neurons[neuron_index].bias();
                self.layers[layer_index].neurons[neuron_index].set_sum(sum);
                let new_activation = sigmoid(sum);
                self.layers[layer_index].neurons[neuron_index].set_act(new_activation);
            }
        }
    }

    pub fn stochastic_gradient_descent(&mut self, inputs:&NumberImg, desired_y:i64, learning_rate:f64, regularization_c:f64) {
        self.set_inputs(&inputs.pixel_brightness);
        self.feedforward();

        let predicted_output:i64 = self.get_network_output();
        
        for lyr_index in (0..self.layers.len()).rev() {
            let partial_gradient:f64 = 0.0;
            let gradient:f64 = 0.0;
        }
    }



    pub fn get_network_output(&self) -> i64 {
        match self.layers.last() {
            Some(lyr) => {
                let mut max_value:&Neuron = &lyr.neurons[0];
                for neuron_index in 1..lyr.neurons.len() {
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


}

