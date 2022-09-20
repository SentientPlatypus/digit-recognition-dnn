pub fn sigmoid(v: f64) -> f64 {
    if v < -40.0 {
        0.0
    } else if v > 40.0 {
        1.0
    } else {
        1.0 / (1.0 + f64::exp(-v))
    }
}

pub fn relu(v:f64) -> f64 {
    if v <= 0
    {
        0.0
    } else {
        v
    }
}
