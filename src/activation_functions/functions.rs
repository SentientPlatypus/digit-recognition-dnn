
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



