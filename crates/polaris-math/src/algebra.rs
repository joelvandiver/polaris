// TODO: Build an algebra problem generator
  // TODO: Evaluate LinearEquation at x
  // TODO: Solve LinearEquation for its root
  // TODO: Generate random linear equations
  // TODO: Format equations as readable text
pub struct LinearEquation {
    pub m: f64,
    pub b: f64,
}

pub fn linear(m: f64, b: f64) -> LinearEquation {
    LinearEquation { m, b }
}
