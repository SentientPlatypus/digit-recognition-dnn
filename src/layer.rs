use crate::neuron::{Neuron, self};
use len_trait::{Len, Empty};


pub enum LayerKind {
    input_layer,
    output_layer,
    hidden_layer
}
pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub kind: LayerKind,
    pub in_features: usize,
    pub out_features: usize
}

impl Layer {
    pub fn new(in_features:usize ,out_features:usize, kind:LayerKind) -> Layer
    {
        let neurons:Vec<Neuron> = Vec::new();
        let mut layer: Layer = Layer {
            neurons:neurons,
            kind: kind,
            in_features: in_features,
            out_features: out_features
        };
        (0..out_features-1).for_each(|i: usize| 
        {
            layer.neurons.push(
                Neuron::new()
            )
        });
        layer
    }

    fn get_neuron(&self, index:usize) -> &Neuron {
        &self.neurons[index]
    }

    pub fn len(&self) -> usize {
        self.neurons.len()
    }
}
