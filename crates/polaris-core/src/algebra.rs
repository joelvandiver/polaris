pub struct LinearEquation {
    pub m: f64,
    pub b: f64,
}

pub fn linear(m: f64, b: f64) -> LinearEquation {
    LinearEquation { m, b }
}
