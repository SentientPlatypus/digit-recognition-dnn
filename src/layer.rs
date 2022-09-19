use crate::neuron::Neuron;


pub struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    fn initialize(&self, n:i8) {

    }

    fn set_neuron(&mut self, neuron:Neuron, index:usize) {
        self.neurons[index] = neuron;
    }

    fn get_neuron(&self, index:usize) -> &Neuron {
        &self.neurons[index]
    }
}
