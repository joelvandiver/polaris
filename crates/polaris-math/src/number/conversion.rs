use super::integer::Integer;
use super::natural::NaturalNumber;
use super::whole::WholeNumber;

/// N ⊂ W: always succeeds.
impl From<NaturalNumber> for WholeNumber {
    fn from(n: NaturalNumber) -> Self {
        Self::new(n.get())
    }
}

impl TryFrom<WholeNumber> for NaturalNumber {
    type Error = String;

    fn try_from(w: WholeNumber) -> Result<Self, Self::Error> {
        Self::new(w.get()).ok_or_else(|| "0 is not a natural number.".to_string())
    }
}

impl TryFrom<Integer> for WholeNumber {
    type Error = String;

    fn try_from(i: Integer) -> Result<Self, Self::Error> {
        let i = i.get();
        // TODO: convert to match and handle all cases.
        if i > 0 {
            Ok(WholeNumber::new(i as u64))
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
    fn whole_to_natural_fails() {
        let w = WholeNumber::new(0);
        let result = NaturalNumber::try_from(w);
        assert_eq!(result, Err("0 is not a natural number.".to_string()))
    }

    #[test]
    fn max_positive_integer_to_whole_succeeds() {
        let i = Integer::new(i64::MAX);
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
