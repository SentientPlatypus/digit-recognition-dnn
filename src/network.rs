use itertools::Itertools;

use crate::{layer::{Layer, LayerKind}, neuron::{Neuron, }, image_processing::{NumberImg, Dataset}};
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

            let mut in_features:usize = 1 as usize;
            let mut out_features:usize = 1 as usize;
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

    pub fn get_output_vec(&self) -> Vec<f64> {
        let mut output:Vec<f64> = Vec::new();
        for neuron in &self.layers.last().expect("Failed to get output layer").neurons {
            output.push(neuron.act())
        }
        output
    }

    

    pub fn backpropagate(&mut self, inputs:&NumberImg, learning_rate:f64, regularization_c:f64) {
        self.set_inputs(&inputs.pixel_brightness);
        self.feedforward();

        let actual_y:i64 = inputs.correct_value;

        let predicted_output:i64 = self.get_network_output();
        
        for lyr_index in (0..self.layers.len()).rev() {

            let mut partial_gradient:f64 = 0.0;
            let mut gradient:f64 = 0.0;

            for out_index in (0..self.layers[lyr_index].out_features()) {
                println!("{}", out_index);
                //Check if we are iterating through the last layer
                //DE/Db = De/Da * Da/Ds * ds/ db
                //ERROR IS THAT WE HAVE MORE OUTFEATURES THAN NEURONS
                if lyr_index == self.layers.len() - 1  {
                    partial_gradient = -2.00 * (actual_y - predicted_output) as f64
                } else {
                    // MAYBE lyr_index + 1??
                    partial_gradient = self.layers[lyr_index ].neurons[out_index].err()
                         * sigmoid(self.layers[lyr_index].neurons[out_index].sum())
                }

                //updating bias by this partial gradient
                let new_bias:f64 = self.layers[lyr_index].neurons[out_index].bias() - learning_rate * partial_gradient;

                self.layers[lyr_index].neurons[out_index].set_bias(
                    new_bias
                );

                for i in (0..self.layers[lyr_index].in_features() - 1) {
                    //Check if we are at the input layer
                    if lyr_index == 0 {
                        gradient = partial_gradient * self.layers[lyr_index].neurons[out_index].act();
                    } else {
                        //set gradient to the product of partial gradient and the previous layer's neuron's activation
                        gradient = partial_gradient * self.layers[lyr_index - 1].neurons[i].act();
                        let error:f64 = partial_gradient * self.layers[lyr_index].neurons[out_index].weight(i);
                        self.layers[lyr_index - 1].neurons[i].add_err(
                            error
                        );
                    }
                    gradient += regularization_c * self.layers[lyr_index].neurons[out_index].weight(i);
                    let new_weight:f64 = self.layers[lyr_index].neurons[out_index].weight(i) - learning_rate * gradient;
                    self.layers[lyr_index].neurons[out_index].set_weight(i, new_weight);
                }
            }
        println!("Finished layer {}", lyr_index);
        }
    }

    pub fn sgd(&mut self, set:&Dataset, learning_rate:f64, epochs:usize) {
        for img_i in (0..=epochs) {
            self.set_inputs(
                &set.images[img_i].pixel_brightness
            );

            self.feedforward();

            self.backpropagate(
                &set.images[img_i], 
                learning_rate, 
                1.0
            );
            println!("Epoch {} complete", epochs);
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

