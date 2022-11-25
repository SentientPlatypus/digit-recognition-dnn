use std::iter;

use itertools::Itertools;

use crate::{layer::{Layer, LayerKind}, neuron::{Neuron, }, image_processing::{NumberImg, Dataset}};
use crate::activation_functions::functions::{
    sigmoid,
    derivative_sigmoid,
    exp_decay_coef,
    relu,
    derivative_relu
};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Network {
    pub layers: Vec<Layer>,
    pub cost:f64,
    pub binary_output:bool,
    pub epochs:i64,
}



impl Network {

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn new(layer_sizes:Vec<usize>, binary_output:bool) -> Network {
        let mut network:Network = Network { layers: Vec::new(), cost:0.0, binary_output: binary_output, epochs:0 };
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


                let new_activation = match self.layers[layer_index].kind {
                    LayerKind::InputLayer => relu(sum),
                    LayerKind::HiddenLayer => relu(sum),
                    LayerKind::OutputLayer => sigmoid(sum)
                };
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

    pub fn get_network_cost(&mut self, actual:i64) -> f64{
        let mut err:f64 = 0.0;
        let outputs:Vec<f64> = self.get_output_vec();
        if !self.binary_output {
            for r in 0..outputs.len() {
                if r == actual as usize {
                    err += (1.0 - outputs[r]).powf(2.0);
                } else {
                    err += (0.0 - outputs[r]).powf(2.0);
                }
            }
        } else {
            err  = (actual as f64 - 
                self.layers.last()
                    .expect("Failed to get last layer")
                    .neurons.last()
                    .expect("failed to get last neuron")
                    .act()
            ).powf(2.0);
        }
        // self.cost = err;
        err
    }

    pub fn backpropagate(&mut self, inputs:&NumberImg, learning_rate:f64, regularization_c:f64, current_epoch:f64) {
        let actual_y:i64 = inputs.correct_value;
        let predicted_output:i64 = self.get_network_output();
        for lyr_index in (0..self.layers.len()).rev() {

            let mut partial_gradient:f64 = 0.0;
            let mut gradient:f64 = 0.0;

            for n_index in 0..self.layers[lyr_index].len() {
                //Check if we are iterating through the last layer
                if lyr_index == self.layers.len() - 1  {
                    partial_gradient = -2.00 * (actual_y - predicted_output) as f64 
                } else {
                    partial_gradient = self.layers[lyr_index].neurons[n_index].err()
                }

                partial_gradient *= match self.layers[lyr_index].kind {
                    LayerKind::HiddenLayer => derivative_relu(self.layers[lyr_index].neurons[n_index].sum()),
                    LayerKind::InputLayer => derivative_relu(self.layers[lyr_index].neurons[n_index].sum()),
                    LayerKind::OutputLayer => derivative_sigmoid(self.layers[lyr_index].neurons[n_index].sum())
                };

                //updating bias by this partial gradient
                let new_bias:f64 = self.layers[lyr_index].neurons[n_index].bias() - learning_rate * partial_gradient * exp_decay_coef(current_epoch);

                self.layers[lyr_index].neurons[n_index].set_bias(
                    new_bias
                );

                for in_index in 0..self.layers[lyr_index].in_features() - 1 {
                    //Check if we are at the input layer
                    if lyr_index == 0 {
                        gradient = partial_gradient * self.layers[lyr_index].neurons[n_index].act();
                    } else {
                        //set gradient to the product of partial gradient and the previous layer's neuron's activation
                        gradient = partial_gradient * self.layers[lyr_index - 1].neurons[in_index].act();

                        let error:f64 = partial_gradient * self.layers[lyr_index].neurons[n_index].weight(in_index);
                        
                        self.layers[lyr_index - 1].neurons[in_index].add_err(
                            error
                        );
                    }
                    gradient += regularization_c * self.layers[lyr_index].neurons[n_index].weight(in_index);
                    
                    let new_weight:f64 = self.layers[lyr_index].neurons[n_index].weight(in_index) - learning_rate * gradient * exp_decay_coef(current_epoch);
                    self.layers[lyr_index].neurons[n_index].set_weight(in_index, new_weight);
                }
            }
        }
    }


    ///Returns mean cost over randomly picking <sample_size> number images from the dataset
    pub fn generate_mean_cost(&mut self, set:&Dataset, sample_size: usize, random_sample:bool) -> f64{
        let mut sum:f64 = 0.0;
        let mut denom:f64 = 0.0;
        let bar_length:u64 = {
            if !random_sample {
                set.images.len() as u64
            }
            else {
                sample_size as u64
            }
        };

        if !random_sample {
            for img in &set.images {
                self.set_inputs(&img.pixel_brightness);
                self.feedforward();
                sum += self.get_network_cost(img.correct_value);
            }
            denom = set.images.len() as f64;
        }
        else {
            for i in 1..=sample_size {
                let img:&NumberImg = set.random_choice();
                self.set_inputs(&img.pixel_brightness);
                self.feedforward();
                sum += self.get_network_cost(img.correct_value);
            }
            denom = sample_size as f64;
        }
        let mean: f64 = sum / denom;
        mean
    }

    pub fn sgd(
        &mut self, 
        set:&mut Dataset,
        learning_rate:f64, 
        epochs:usize, 
        regularization_c:f64, 
        batch_size:usize, 
        iterations_per_epoch:usize,
        cost_sample_size:usize,
        simple_random_sample:bool
    ) 
    {
        for epoch in 1..=epochs {

            let bar = ProgressBar::new(iterations_per_epoch as u64);
            bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.blue/red} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("▓▓▒"));

            let mut correct_predictions:i64 = 0;

            for iteration in 1..=iterations_per_epoch {
                let individual:&NumberImg = set.random_choice();
                self.set_inputs(
                    &individual.pixel_brightness
                );
    
                self.backpropagate(
                    individual, 
                    learning_rate, 
                    regularization_c,
                    epoch as f64
                );

                if self.get_network_output() == individual.correct_value {
                    correct_predictions += 1;
                }

                self.cost = self.generate_mean_cost(set, cost_sample_size, simple_random_sample);
                bar.inc(1);
                bar.set_message(format!("Average Cost: {} ({} correct/ {} iterations)", 
                    self.cost(), 
                    correct_predictions, 
                    iterations_per_epoch
                ));
            }
            bar.finish();

            println!(
                "Epoch {} complete. Cost:{}", 
                epoch, 
                self.cost(),
            );
        }
    }

    pub fn get_network_output(&self) -> i64 {
        match self.layers.last() {
            Some(lyr) => {

                if !self.binary_output {
                    let mut max_value:&Neuron = &lyr.neurons[0];
                    for neuron_index in 0..lyr.neurons.len() {
                        if lyr.neurons[neuron_index].act() > max_value.act() {
                            max_value = &lyr.neurons[neuron_index];
                        }
                    }
                    return max_value.id() as i64;
                } else {
                    return lyr.neurons.last().expect("Failed to get last neuron").act().round() as i64;
                }
            }
            None => {
                panic!("failed to retrieve last layer")
            }
        }
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }


}

