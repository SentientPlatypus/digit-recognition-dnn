use crate::dendrite::Dendrite;

pub struct Neuron {
    n_id: usize,
    n_value: i8,
    weights: Vec<f64>,
    n_bias: f32,
    n_delta:i8
}

impl Neuron {
    fn set_weights(&mut self, n:usize, weight:f64) {
        self.weights[n] = weight;
    }
}
