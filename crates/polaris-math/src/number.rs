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

/// N ⊂ W: always succeeds.
impl From<NaturalNumber> for WholeNumber {
    fn from(n: NaturalNumber) -> Self {
        Self(n.get())
    }
}

impl TryFrom<WholeNumber> for NaturalNumber {
    type Error = String;

    fn try_from(w: WholeNumber) -> Result<Self, Self::Error> {
        Self::new(w.get()).ok_or_else(|| "0 is not a natural number.".to_string())
    }
}

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

impl TryFrom<Integer> for WholeNumber {
    type Error = String;

    fn try_from(i: Integer) -> Result<Self, Self::Error> {
        let i = i.get();
        // TODO: convert to match and handle all cases.
        if i > 0 {
            Ok(WholeNumber(i as u64))
        } else {
            // TODO: Assert error.
            // Err("".to_string())
            todo!()
        }
    }
}

/// A rational number, ℚ =
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct RationalNumber(u64, u64);
/// A real number, ℝ
/// A complex number, ℂ

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

    #[test]
    fn whole_accepts_min() {
        assert_eq!(WholeNumber::new(u64::MIN).get(), 0);
    }

    #[test]
    fn whole_accepts_max() {
        assert_eq!(WholeNumber::new(u64::MAX).get(), u64::MAX);
    }

    #[test]
    fn whole_to_natural_fails() {
        let w = WholeNumber(0);
        let result = NaturalNumber::try_from(w);
        assert_eq!(result, Err("0 is not a natural number.".to_string()))
    }

    #[test]
    fn max_positive_integer_to_whole_succeeds() {
        let i = Integer(i64::MAX);
        let result = WholeNumber::try_from(i);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), i64::MAX as u64);
    }

    // TODO: continue...
    // 
    
    proptest! {
        #[test]
        fn from_natural_to_whole_succeeds(n in 1..=u64::MAX) {
            let nat = NaturalNumber::new(n).unwrap();
            prop_assert_eq!(WholeNumber::from(nat).get(), n);
        }
        
        #[test]
        fn whole_to_natural_round_trips(n in 1..=u64::MAX) {
            let w = WholeNumber::new(n);
            let nat = NaturalNumber::try_from(w).unwrap();
            prop_assert_eq!(WholeNumber::from(nat), w);
        }
        
        #[test]
        fn conversion_preserves_order(a in 1..=u64::MAX, b in 1..=u64::MAX) {
            let (x, y) = (NaturalNumber::new(a).unwrap(), NaturalNumber::new(b).unwrap());
            prop_assert_eq!(x.cmp(&y), WholeNumber::from(x).cmp(&WholeNumber::from(y)));
        }
    }
    // TODO: positive_integers_to_whole_succeeds
    //          Get absolute value of integer before converting to Whole Number
    // TODO: negative_integers_to_whole_fails
}
