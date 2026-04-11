use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A 2-dimensional point with floating-point coordinates.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    /// Create a new `Point` at `(x, y)`.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Euclidean distance to another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Return the midpoint between this point and another.
    pub fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }

    /// Translate the point by `(dx, dy)`.
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn midpoint() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(4.0, 6.0);
        let m = a.midpoint(&b);
        assert_eq!(m, Point::new(2.0, 3.0));
    }

    #[test]
    fn translate() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.translate(3.0, -1.0), Point::new(4.0, 1.0));
    }
}
