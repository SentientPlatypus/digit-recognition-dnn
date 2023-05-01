///
/// THE NETWORK
/// 


use crate::{layer::{Layer, LayerKind, self}, neuron::{Neuron, }, image_processing::{NumberImg, Dataset}, activation_functions::{functions::{softmax, cross_entropy_loss, softmax2}, self}};
use crate::activation_functions::functions::{
    sigmoid,
    derivative_sigmoid,
    exp_decay_coef,
    relu,
    derivative_relu
};
use indicatif::{ProgressBar, ProgressStyle};
use textplots::{Chart, Plot, Shape};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;




#[derive(Serialize, Deserialize)]
pub struct Network {
    pub layers: Vec<Layer>,
    pub cost:f64,
    pub binary_output:bool,
}



impl Network {

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn new(layer_sizes:Vec<usize>, binary_output:bool) -> Network {
        ///Creates a new network.
        let mut network:Network = Network { layers: Vec::new(), cost:0.0, binary_output: binary_output};
        for size_index in 0..layer_sizes.len()
        {
            let last_index = layer_sizes.len() - 1;

            let kind: LayerKind = {
                if size_index == 0 {
                    LayerKind::InputLayer
                } else if size_index == last_index {
                    LayerKind::OutputLayer  
                } else {
                    LayerKind::HiddenLayer
                }
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


    ///Literally for putting your inputs into the input layer.
    pub fn set_inputs(&mut self, pixels:&Vec<f64>) {
        let input_layer: &mut Layer = &mut self.layers[0];
        for neuron_index in 0..input_layer.len() {
            input_layer.neurons[neuron_index].set_act(f64::from(pixels[neuron_index]));
            input_layer.neurons[neuron_index].set_err(0.0);
        }
    }

    ///Resets the error in each neuron of the network. Should do this after each backpropagation
    pub fn reset(&mut self) {
        for layer_i in 0..self.layers.len() {
            for neuron_index in 0..self.layers[layer_i].neurons.len() {
                self.layers[layer_i].neurons[neuron_index].set_err(0.0);
            }
        }
    }

    //initializes network with random weights
    ///Initializes network with random weights and biases.
    pub fn initialize(&mut self){
        //Go through each layer except the input and softmax layer
        (1..self.layers.len() - 1).for_each(|layer_index: usize| 
        {
            //Go through each neuron of the above layer
            (0..self.layers[layer_index].neurons.len()).for_each(|neuron_index: usize| 
            {
                for count in 0..self.layers[layer_index].in_features{
                    self.layers[layer_index].neurons[neuron_index].generate_random_weight(count);
                }
                self.layers[layer_index].neurons[neuron_index].generate_random_bias();
            });
        });   
    }

    ///Iterate through the layers except for the first layer
    pub fn feedforward(&mut self) {
        let mut softmax_vals:Vec<f64> = Vec::new();
        for layer_index in 1..self.layers.len() {
            // Iterate through the neurons of that layer
            for neuron_index in 0..self.layers[layer_index].neurons.len() {

                //the sum is 0
                let mut sum:f64 = 0.0;
                let new_activation:f64;

                //Check if its the softmax layer
                if layer_index == self.layers.len() - 1 {
                    assert_eq!(self.layers[layer_index - 1].neurons.len(), self.layers[layer_index].neurons.len());
                    new_activation = softmax(self.layers[layer_index - 1].neurons[neuron_index].act(), &self.layers[layer_index]);
                }
                else {
                    //Iterate through the weights of the current neuron
                    for weight_index in 0..self.layers[layer_index].neurons[neuron_index].weights.len(){
                        sum += 
                            self.layers[layer_index].neurons[neuron_index].weights[weight_index] * 
                            self.layers[layer_index - 1].neurons[weight_index].act();
                    }
                    //ADD BIAS
                    sum += self.layers[layer_index].neurons[neuron_index].bias();

                    //SET SUM
                    self.layers[layer_index].neurons[neuron_index].set_sum(sum);

                    // APPLY ACTIVATION FUNCTION BASED UPON LAYER
                    new_activation = match self.layers[layer_index].kind {
                        LayerKind::InputLayer => sigmoid(sum),
                        LayerKind::HiddenLayer => sigmoid(sum),
                        LayerKind::OutputLayer => sigmoid(sum),
                    };    
                }

                self.layers[layer_index].neurons[neuron_index].set_act(new_activation);
            }
        }
    }


    ///returns the current output layer.
    pub fn get_output_vec(&self) -> Vec<f64> {
        let mut output:Vec<f64> = Vec::new();
        for neuron in &self.layers.last().expect("Failed to get output layer").neurons {
            output.push(neuron.act())
        }
        output
    }


    /// Returns what the output layer should have been.
    pub fn get_true_output_vec(&self, actual_y:i64) -> Vec<f64> {
        if self.binary_output {
            return vec![actual_y as f64];
        } else {
            let mut output_vec:Vec<f64> = Vec::new();
            for neuron_id in 0..self.layers.last().expect("Failed to get output layer").len() {
                if neuron_id == actual_y as usize {
                    output_vec.push(1.0);
                } else{
                    output_vec.push(0.0);
                }
            }
            return output_vec
        }
    }

    ///Gets the cost of the network. the actual parameter is what the actual output should have been.
    pub fn get_network_cost(&mut self, actual:i64) -> f64{
        
        let err:f64;
        let outputs:Vec<f64> = self.get_output_vec();

        //Is it not a binary output? eg: one neuron output?
        if !self.binary_output {
            err = cross_entropy_loss(actual, outputs);
        } else {
            //IT IS A BINARY OUTPUT
            //MSE
            err  = (actual as f64 - 
                self.layers.last()
                    .expect("Failed to get last layer")
                    .neurons.last()
                    .expect("failed to get last neuron")
                    .act()
            ).powf(2.0);
        }
        err
    }

    ///Does one backpropagation step.
    pub fn backpropagate(&mut self, inputs:&NumberImg, learning_rate:f64, regularization_c:f64, current_epoch:usize) {
        let true_output_vec: Vec<f64> = self.get_true_output_vec(inputs.correct_value);
        let predicted_output_vec:Vec<f64> = self.get_output_vec();
        let new_learning_rate:f64 = learning_rate * exp_decay_coef(current_epoch as f64 - 1.0);

        //Iterate through all the layers except the input from the last to the first (THE MINUS ONE IS TO EXCLUDE THE SOFTMAX LAYER)
        for lyr_index in (1..=self.layers.len() - 2).rev() {

            let mut partial_gradient:f64;
            let mut gradient:f64;

            //Iterate through all the neurons of the layer
            for n_index in 0..self.layers[lyr_index].len() {

                //Check if we are iterating through the last layer (THE ONE BEFORE SOFTMAX)
                if lyr_index == self.layers.len() - 2  {
                    //IF WE ARE USING CROSS ENTROPY LOSS:  
                    partial_gradient = (predicted_output_vec[n_index] - true_output_vec[n_index]) as f64 
                } else {
                    partial_gradient = self.layers[lyr_index].neurons[n_index].err()
                }

                partial_gradient *= match self.layers[lyr_index].kind {
                    LayerKind::HiddenLayer => derivative_sigmoid(self.layers[lyr_index].neurons[n_index].sum()),
                    LayerKind::InputLayer => derivative_sigmoid(self.layers[lyr_index].neurons[n_index].sum()),
                    LayerKind::OutputLayer => derivative_sigmoid(self.layers[lyr_index].neurons[n_index].sum())
                };

                //updating bias by this partial gradient
                let new_bias:f64 = self.layers[lyr_index].neurons[n_index].bias() - new_learning_rate * partial_gradient;

                self.layers[lyr_index].neurons[n_index].set_bias(
                    new_bias
                );

                //iterate through the range of the size of the previous layer AKA the in features of the layer
                for in_index in 0..self.layers[lyr_index].in_features() {
                    //Check if we are at the input layer
                    if lyr_index == 1 {
                        //gradient = partial gradient * input layer activation at index in_index
                        gradient = partial_gradient * self.layers[lyr_index - 1].neurons[in_index].act();
                    } else {
                        //set gradient to the product of partial gradient and the previous layer's neuron's activation
                        gradient = partial_gradient * self.layers[lyr_index - 1].neurons[in_index].act();
         
                        let to_add_err = partial_gradient * &self.layers[lyr_index].neurons[n_index].weight(in_index);

                        self.layers[lyr_index - 1].neurons[in_index].add_err(
                            to_add_err
                        );
                    }
                    gradient += regularization_c * self.layers[lyr_index].neurons[n_index].weight(in_index);
                    
                    let new_weight:f64 = self.layers[lyr_index].neurons[n_index].weight(in_index) - new_learning_rate * gradient;
                    self.layers[lyr_index].neurons[n_index].set_weight(in_index, new_weight);
                }
            }
        }
    }


    pub fn sgd(
        &mut self, 
        set:&mut Dataset,
        learning_rate:f64, 
        epochs:usize, 
        regularization_c:f64, 
        batch_size:usize, 
        iterations_per_epoch:usize,
    ) 
    {

        let mut avg_costs:Vec<(f32, f32)> = Vec::new();
        for epoch in 1..=epochs {

            let bar = ProgressBar::new(iterations_per_epoch as u64);
            bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.blue/red} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("▰▰╍"));

            let mut correct_predictions:i64 = 0;
            let mut sum:f64 = 0.0;
            let mut denom:f64 = 0.0;
            let mut mean = 0.0;

            let mut batch:Vec<&NumberImg> = Vec::new();
            for _case in 1..=batch_size {
                batch.push(set.random_choice());
            }

            for _iteration in 1..=iterations_per_epoch {

                for img in &batch {
                    self.reset();
                    self.set_inputs(
                        &img.pixel_brightness
                    );
                    
                    self.feedforward();

                    self.backpropagate(
                        img, 
                        learning_rate, 
                        regularization_c,
                        epoch
                    );
    
                    sum += self.get_network_cost(img.correct_value);
                    denom += 1.0;
    
                    if self.get_network_output() == img.correct_value {
                        correct_predictions += 1;
                    }
                }

                self.cost = sum / denom;

                mean = sum / denom;
                bar.inc(1);
                bar.set_message(format!("Average Cost: {} ({} correct/ {})", 
                    self.cost(), 
                    correct_predictions, 
                    denom
                ));
            }


            avg_costs.push((epoch as f32, mean as f32));

            bar.finish();
            println!(
                "Epoch {} complete. Cost:{}", 
                epoch, 
                self.cost(),
            );
        }



        Chart::new(200, 60, 0.0, epochs as f32 + 5.0 )
        .lineplot(&Shape::Lines(&avg_costs))
        .display();
    }


    ///get the network output
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

    ///serializes the network, and writes it to a file.
    pub fn to_file(&self, path:String) {
        let as_json_str:String = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).expect("Failed to get file");

        // Write the JSON string to the file
        file.write_all(as_json_str.as_bytes()).expect("Failed to write");
    }

    ///Generates a network from a file
    pub fn from_file(path:String) -> Network{
        let file = File::open(path).expect("failed to open path");
        let net: Network = serde_json::from_reader(file).expect("failed to deserialize");
        net
    }

}

