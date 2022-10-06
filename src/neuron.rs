use rand::Rng;

#[derive(Default)]
pub struct Neuron {
    pub n_id: usize,
    pub n_value: f64,
    pub weights: Vec<f64>,
    pub n_bias: f64,
    pub n_delta:i8
}

impl Neuron {
    fn set_weights(&mut self, n:usize, weight:f64) {
        self.weights[n] = weight;
    }

    pub fn set_value(&mut self, value:f64) {
        self.n_value = value;
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
