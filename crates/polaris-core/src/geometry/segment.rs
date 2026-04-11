use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::Point;

/// A line segment between two endpoints.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

#[wasm_bindgen]
impl Segment {
    /// Create a new `Segment` from `start` to `end`.
    #[wasm_bindgen(constructor)]
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Length of the segment.
    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }

    /// Midpoint of the segment.
    pub fn midpoint(&self) -> Point {
        self.start.midpoint(&self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let s = Segment::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
        assert!((s.length() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn midpoint() {
        let s = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 4.0));
        assert_eq!(s.midpoint(), Point::new(1.0, 2.0));
    }
}
