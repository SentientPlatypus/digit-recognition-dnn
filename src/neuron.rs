use crate::dendrite::Dendrite

pub struct Neuron {
    n_id: usize,
    n_value: i8,
    dendrites: Vec<Dendrite>,
    n_bias: f8,
    n_delta:i8
}

impl Neuron {
    fn set_dendrites(&self, n:usize) {
        print("kill me")
    }
}
