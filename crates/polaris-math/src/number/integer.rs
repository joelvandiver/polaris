/// An integer, ℤ = {..., -2, -1, 0, 1, 2, ...}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer(i64);

impl Integer {
    pub const fn new(n: i64) -> Self {
        Self(n)
    }

    pub const fn get(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use proptest::prelude::*;

    // TODO: continue...
    //

    proptest! {}
}
