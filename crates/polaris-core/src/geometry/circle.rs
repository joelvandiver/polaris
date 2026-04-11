use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::Point;

/// A circle defined by a center point and radius.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[wasm_bindgen]
impl Circle {
    /// Create a new `Circle` with the given center and radius.
    #[wasm_bindgen(constructor)]
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    /// Circumference of the circle (2πr).
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    /// Area of the circle (πr²).
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// Returns `true` if the point lies inside the circle.
    pub fn contains(&self, p: &Point) -> bool {
        self.center.distance_to(p) <= self.radius
    }

    /// Returns `true` if two circles intersect.
    pub fn intersects(&self, other: &Circle) -> bool {
        let dist = self.center.distance_to(&other.center);
        dist <= self.radius + other.radius && dist >= (self.radius - other.radius).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let c = Circle::new(Point::new(0.0, 0.0), 1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn contains() {
        let c = Circle::new(Point::new(0.0, 0.0), 5.0);
        assert!(c.contains(&Point::new(3.0, 4.0)));
        assert!(!c.contains(&Point::new(4.0, 4.0)));
    }

    #[test]
    fn intersects() {
        let a = Circle::new(Point::new(0.0, 0.0), 3.0);
        let b = Circle::new(Point::new(4.0, 0.0), 2.0);
        assert!(a.intersects(&b));

        let c = Circle::new(Point::new(10.0, 0.0), 1.0);
        assert!(!a.intersects(&c));
    }
}
