use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::Point;

/// An infinite line passing through two distinct points.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

#[wasm_bindgen]
impl Line {
    /// Create a new `Line` through `p1` and `p2`.
    #[wasm_bindgen(constructor)]
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    /// Length of the direction vector (distance between the two defining points).
    pub fn length(&self) -> f64 {
        self.p1.distance_to(&self.p2)
    }

    /// Slope of the line, or `f64::INFINITY` for vertical lines.
    pub fn slope(&self) -> f64 {
        let dx = self.p2.x - self.p1.x;
        if dx == 0.0 {
            f64::INFINITY
        } else {
            (self.p2.y - self.p1.y) / dx
        }
    }

    /// Perpendicular distance from a point to this line.
    pub fn distance_to_point(&self, p: &Point) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return self.p1.distance_to(p);
        }
        ((dy * p.x - dx * p.y + self.p2.x * self.p1.y - self.p2.y * self.p1.x) / len).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slope_horizontal() {
        let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0));
        assert_eq!(l.slope(), 0.0);
    }

    #[test]
    fn slope_vertical() {
        let l = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0));
        assert!(l.slope().is_infinite());
    }

    #[test]
    fn distance_to_origin() {
        // Line y = x  →  distance from (1,0) is 1/sqrt(2)
        let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let d = l.distance_to_point(&Point::new(1.0, 0.0));
        assert!((d - 1.0 / 2.0_f64.sqrt()).abs() < 1e-10);
    }
}
