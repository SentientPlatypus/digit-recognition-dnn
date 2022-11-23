

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
    (-epochs).exp()
}

pub fn relu(v:f64) -> f64 {
    if v < 0.0
    {
        return 0.0
    }
    v
}
