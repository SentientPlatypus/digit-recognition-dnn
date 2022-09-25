use crate::dendrite::Dendrite;

pub struct Neuron {
    pub n_id: usize,
    pub n_value: f64,
    pub weights: Vec<f64>,
    pub n_bias: f32,
    pub n_delta:i8
}

impl Neuron {
    fn set_weights(&mut self, n:usize, weight:f64) {
        self.weights[n] = weight;
    }
}
