/// Represents a location in a space.
///
/// In Synthetic Geometry, the point is undefined.
pub struct Point;

/// Represents a line in space.
///
/// A line is an infinitely long, one-dimensional
/// object.
pub struct Line {
    point_a: Point,
    point_b: Point,
}

/// Represents a segment in a space.
///
/// A segement is a finite, one-dimensional
/// object.
pub struct Segment {
    point_a: Point,
    point_b: Point,
}

/// Represents a plan in a space.
///
/// A plane is an infinitely long and wide, two-
/// dimensional object.
pub struct Plane {
    point_a: Point,
    point_b: Point,
    point_c: Point,
}

/// Represents a circle in a plane.
///
/// A circle is defined by all points equidistant
/// to a point.
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

/// Represents a rectangle in a plane.
///
/// A rectangle is a 4-sided, two-dimensional
/// object with right angles between sides.
pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,
    pub width: f64,
    pub height: f64,
}

/// Represents a square in a plane.
///
/// A square is a 4 equal-sided, two-dimensional
/// object with right angles between sides.
pub struct Square {
    pub point_a: Point,
    pub point_b: Point,
    pub length: f64,
}

// TODO: Define other synthetic geometry types.
