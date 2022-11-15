use rand::Rng;
use std::fmt;


#[derive(Default)]



pub struct Neuron {
    pub id:usize,
    pub activation: f64,
    pub weights: Vec<f64>,
    pub bias: f64,
    pub sum: f64,
    pub error_sum: f64,
}


impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Neuron")
         .field("id", &self.id)
         .field("activation", &self.activation)
         .field("sum", &self.sum)
         .field("bias", &self.bias)
         .field("error_sum", &self.error_sum)
         .field("Weights", &self.weights)
         .finish()
    }
}

impl Neuron {

    pub fn new(id:usize) -> Neuron{
        Neuron {
            id:id,
            activation: 0.0,
            weights: Vec::new(), 
            bias: 0.0, 
            sum: 0.0, 
            error_sum: 0.0
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn bias(&self) -> f64 {
        self.bias
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn err(&self) -> f64 {
        self.error_sum
    }

    pub fn act(&self) -> f64 {
        self.activation
    }
    
    pub fn weight(&self, index:usize) -> f64 {
        self.weights[index]
    }

    pub fn set_weight(&mut self, n:usize, value:f64) {
        self.weights[n] = value;
    }

    pub fn set_bias(&mut self, value:f64) {
        self.bias = value
    }

    pub fn set_act(&mut self, value:f64) {
        self.activation = value
    }

    pub fn set_sum(&mut self, value:f64) {
        self.sum = value
    }

    pub fn add_err(&mut self, value:f64) {
        self.error_sum += value
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
