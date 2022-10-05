use crate::neuron::{Neuron, self};
use len_trait::{Len, Empty};

pub struct Layer {
    pub neurons: Vec<Neuron>
}

impl Layer {
    pub fn build(n:usize) -> Layer
    {
        let neurons:Vec<Neuron> = Vec::new();
        let mut layer: Layer = Layer {
            neurons:neurons
        };
        (0..n-1).for_each(|i: usize| 
        {
            layer.neurons.push(
                Neuron {
                    n_id: i,
                    n_value : 0.0,
                    weights : Vec::new(),
                    n_bias : 0.0,
                    n_delta : 0
                }
            )
        });
        layer
    }

    pub fn set_neuron(&mut self, neuron:Neuron, index:usize) {
        self.neurons[index] = neuron;
    }

    pub fn set_neuron_val(&mut self, value:f64, index:usize) {
        self.neurons[index].n_value = value;
    }

    fn get_neuron(&self, index:usize) -> &Neuron {
        &self.neurons[index]
    }

    pub fn len(&self) -> usize {
        self.neurons.len()
    }
}
