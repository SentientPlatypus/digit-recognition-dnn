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


    pub fn add_layer(&mut self, in_features:usize ,out_features:usize,size:usize, kind:LayerKind) {
        self.layers.push(Layer::new(in_features, out_features, size, kind))
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

    //[784, 8, 8, 10]
    pub fn new(layer_sizes:Vec<usize>) -> Network {
        let mut layers:Vec<Layer> = Vec::new();

        let mut network:Network = Network { layers: layers };
        for size_index in 0..layer_sizes.len()
        {
            let last_index = layer_sizes.len() - 1;

            let kind:LayerKind = match size_index {
                0 => LayerKind::hidden_layer,
                _last_index=> LayerKind::output_layer,
                _=> LayerKind::hidden_layer
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
        // println!("{:#?}", pixels);
        let input_layer = &mut self.layers[0];
        for neuron_index in 0..input_layer.len() {
            input_layer.neurons[neuron_index].set_act(f64::from(pixels[neuron_index]));
        }
    }
}

