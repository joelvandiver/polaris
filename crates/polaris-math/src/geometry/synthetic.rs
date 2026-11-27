use super::analytic;

/// Identifies an object in a scene by
/// its position in constuction order.
pub struct ConstructionIdx(usize);

/// TODO: Consider `Branch` for `which`.
/// TODO: Consider using NaturalNumber.

/// Represents a step in a geometric construction.
/// Objects refer to earlier objects by id.
/// Only points hold coordinates.
pub enum Construction {
    Point(analytic::Point),
    Line {
        through: [ConstructionIdx; 2],
    },
    Circle {
        center: ConstructionIdx,
        through: ConstructionIdx,
    },
    Intersection {
        of: [ConstructionIdx; 2],
        /// Constructions can intersect multiple times;
        /// `which` reprsents which one.
        which: usize,
    },
}
