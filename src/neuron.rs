use rand::Rng;

use crate::activation_functions::functions::derivative_sigmoid;

#[derive(Default)]



pub struct Neuron {
    pub n_id: usize,
    pub n_value: f64,
    pub weights: Vec<f64>,
    pub n_bias: f64,
    pub n_delta:i8,
    pub n_sum: f64,
    pub error_sum: f64,
}

impl Neuron {
    pub fn set_weights(&mut self, n:usize, weight:f64) {
        self.weights[n] = weight;
    }

    pub fn update_weights(&mut self, n:usize, change:f64) {
        self.weights[n] -= change;
    }
    pub fn set_value(&mut self, value:f64) {
        self.n_value = value;
    }

    pub fn dC_over_dA(&self, desired_output:f64) -> f64 {
        return (2.0 * (self.n_value  - desired_output))
    }

    pub fn dA_over_dZ(&self) -> f64{
        derivative_sigmoid(self.n_sum)
    }

    pub fn dZ_over_dB(&self) -> f64 {
        1.0
    }

    


    pub fn generate_random_weight(&mut self, index:usize) {
        if index >= self.weights.len() {
            self.weights.push(rand::thread_rng().gen_range(-1.0..1.0));
        }
        else {
            self.weights[index] = rand::thread_rng().gen_range(-1.0..1.0);
        }
    }
}
