use crate::dendrite::Dendrite;

pub struct Neuron {
    n_id: usize,
    n_value: i8,
    dendrites: Vec<Dendrite>,
    n_bias: f32,
    n_delta:i8
}

impl Neuron {
    fn set_dendrites(&self, n:usize) {
        let d:Dendrite = Dendrite {
            d_weight = 0.0,
            d_points_to = 789789
        }
    }
}
