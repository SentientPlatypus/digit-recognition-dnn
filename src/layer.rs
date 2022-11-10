use crate::neuron::{Neuron, self};
use len_trait::{Len, Empty};
use std::fmt;

pub enum LayerKind {
    input_layer,
    output_layer,
    hidden_layer
}

// impl fmt::Debug for LayerKind {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub kind: LayerKind,
    pub in_features: usize,
    pub out_features: usize
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layer")
         .field("neurons", &self.neurons)
        //  .field("kind", &self.kind)
         .field("in_features", &self.in_features)
         .field("out_features", &self.out_features)
         .finish()
    }
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
                Neuron::new(i)
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
