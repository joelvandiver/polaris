pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Find the distance between 2 points in the Euclidean plane.
pub fn distance(a: &Point, b: &Point) -> f64 {
    (b.x - a.x).hypot(b.y - a.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_exact_3_4_5() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert_eq!(distance(&a, &b), 5.0);
    }

    #[test]
    fn distance_is_symmetric() {
        let a = Point { x: 1.5, y: -2.0 };
        let b = Point { x: -3.0, y: 0.25 };
        assert_eq!(distance(&a, &b), distance(&b, &a));
    }

    #[test]
    fn distance_float_tolerance() {
        let a = Point { x: 0.1, y: 0.2 };
        let b = Point { x: 0.4, y: 0.6 };
        // 0.3-0.4-0.5 triangle; exact equality would fail in f64
        assert!((distance(&a, &b) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn distance_zero_for_same_point() {
        let a = Point { x: 7.0, y: -1.0 };
        assert_eq!(distance(&a, &a), 0.0);
    }
}
