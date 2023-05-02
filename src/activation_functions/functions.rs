
use std::f32::consts::E;

use json::number::NumberOutOfScope;

use crate::layer::Layer;



pub fn sigmoid(v: f64) -> f64 {
    if v < -40.0 {
        0.0
    } else if v > 40.0 {
        1.0
    } else {
        1.0 / (1.0 + f64::exp(-v))
    }
}

pub fn derivative_sigmoid(v:f64) ->f64 {
    sigmoid(v) * (1.0 - sigmoid(v))
}

pub fn exp_decay_coef(epochs:f64) -> f64 {
    (-1.0/10.0 * epochs).exp()
}

pub fn relu(v:f64) -> f64 {
    if v < 0.0
    {
        return 0.0
    }
    v
}

pub fn derivative_relu(v:f64) -> f64 {
    if v > 0.0 {
        return 1.0
    } 
    return 0.0
}


pub fn softmax(v:f64, layer:&Layer) -> f64 {
    let denom:f64 = {
        let mut sum:f64 = 0.0;
        for neuron in &layer.neurons {
            sum += neuron.act().exp();
        }
        sum
    };
    v.exp() / denom
}

/*
SOFTMAX BUT REMOVES THE MAXIMUM VALUE??? */
pub fn softmax2(layer:&Layer) -> Vec<f64> {
    let max_act:usize = {
        let mut neuron_in:usize = 0;
        for neuron_i in 0..layer.neurons.len() {
            if layer.neurons[neuron_i].act() > layer.neurons[neuron_in].act() {
                neuron_in = neuron_i;
            }
        }
        neuron_in
    };

    let mut output:Vec<f64> = Vec::new();
    for neuron_i in 0..layer.neurons.len() {
        output.push(layer.neurons[neuron_i].act() - layer.neurons[max_act].act());
    }

    let denom:f64 = {
        let mut sum:f64 = 0.0;
        for x in &output {
            sum += x.exp();
        }
        sum
    };

    let mut softmaxlyr:Vec<f64> = Vec::new();
    for i in 0..output.len() {
        softmaxlyr.push(output[i].exp() / denom)
    }
    softmaxlyr
}

pub fn cross_entropy_loss(actual:i64, output_vec:Vec<f64>) ->f64 {
    let mut sum:f64 = 0.0;
    for val_i in 0..output_vec.len() {
        if val_i == actual as usize {
            sum += E as f64 * output_vec[val_i].ln()
        } else {
            sum += 1.00 / E as f64 * output_vec[val_i].ln()
        }
    }
    -1.00 * sum
}
