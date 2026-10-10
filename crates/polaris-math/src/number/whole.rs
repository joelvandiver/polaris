
/// A whole number, W = {0, 1, 2, 3, ...}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WholeNumber(u64);

impl WholeNumber {
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn whole_accepts_min() {
        assert_eq!(WholeNumber::new(u64::MIN).get(), 0);
    }

    #[test]
    fn whole_accepts_max() {
        assert_eq!(WholeNumber::new(u64::MAX).get(), u64::MAX);
    }

    // TODO: continue...
    //

    proptest! {
    }
}
