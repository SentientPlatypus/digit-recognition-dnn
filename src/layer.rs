use crate::neuron::{Neuron, self};
use len_trait::{Len, Empty};


pub enum LayerKind {
    input_layer,
    output_layer,
    hidden_layer
}
pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub kind: LayerKind
}

impl Layer {
    pub fn build(n:usize, kind:LayerKind) -> Layer
    {
        let neurons:Vec<Neuron> = Vec::new();
        let mut layer: Layer = Layer {
            neurons:neurons,
            kind: kind
        };
        (0..n-1).for_each(|i: usize| 
        {
            layer.neurons.push(
                Neuron {
                    n_id: i,
                    n_value : 0.0,
                    weights : Vec::new(),
                    n_bias : 0.0,
                    n_delta : 0,
                    n_sum : 0.0,
                    error_sum : 0.0
                }
            )
        });
        layer
    }

    pub fn get_max_neuron(&self) -> &Neuron {
        let mut max_neuron: &Neuron = &self.neurons[0];
        for neuron_index in 1..&self.neurons.len() - 1 {
            if self.neurons[neuron_index].n_value > max_neuron.n_value {
                max_neuron = &self.neurons[neuron_index];
            }
        }
        max_neuron
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
