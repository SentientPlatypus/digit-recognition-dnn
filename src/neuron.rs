use rand::Rng;

#[derive(Default)]
pub struct Neuron {
    pub n_id: usize,
    pub n_value: u8,
    pub weights: Vec<f64>,
    pub n_bias: f32,
    pub n_delta:i8
}

impl Neuron {
    fn set_weights(&mut self, n:usize, weight:f64) {
        self.weights[n] = weight;
    }

    pub fn set_value(&mut self, value:u8) {
        self.n_value = value;
    }

    pub fn generate_random_weight(&mut self, index:usize) {
        self.weights[index] = rand::thread_rng().gen_range(-1.0..1.0);
    }
}
