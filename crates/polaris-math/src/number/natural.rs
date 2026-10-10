use std::num::NonZeroU64;

/// A natural number, ℕ = {1, 2, 3, ...}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NaturalNumber(NonZeroU64);

impl NaturalNumber {
    pub const fn new(n: u64) -> Option<Self> {
        match NonZeroU64::new(n) {
            Some(n) => Some(Self(n)),
            None => None,
        }
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn natural_rejects_zero_and_negatives() {
        assert_eq!(NaturalNumber::new(0), None);
    }

    #[test]
    fn natural_accepts_min() {
        assert_eq!(NaturalNumber::new(1).unwrap().get(), 1);
    }

    #[test]
    fn natural_accepts_max() {
        assert_eq!(NaturalNumber::new(u64::MAX).unwrap().get(), u64::MAX);
    }

    // TODO: continue...
    //

    proptest! {
    }
}
