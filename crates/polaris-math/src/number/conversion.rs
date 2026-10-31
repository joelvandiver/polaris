// TODO: Add From<Integer> for RationalNumber
// TODO: Convert all Self::Errors to anyhow (?).

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

impl TryFrom<Integer> for NaturalNumber {
    type Error = String;

    fn try_from(i: Integer) -> Result<Self, Self::Error> {
        let i = i.get();

        if i > 0 {
            return Ok(NaturalNumber::new(i as u64).unwrap());
        }
        return Err(format!("The number is not natural: {i}"));
    }
}

impl TryFrom<Integer> for WholeNumber {
    type Error = String;

    fn try_from(i: Integer) -> Result<Self, Self::Error> {
        let i = i.get();
        if i >= 0 {
            Ok(WholeNumber::new(i as u64))
        } else {
            Err(format!("The number is not a whole number: {i}"))
        }
    }
}

impl TryFrom<WholeNumber> for Integer {
    type Error = String;

    fn try_from(w: WholeNumber) -> Result<Self, Self::Error> {
        let w = w.get();
        let m = i64::MAX as u64;
        if w > m {
            return Err(format!("The whole number is outside of the supported range for a positive integer: {w}"));
        }
        Ok(Integer::new(w as i64))
    }
}

// A rational number, ℚ =
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct RationalNumber(u64, u64);
// A real number, ℝ
// A complex number, ℂ

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    mod natural_to_whole {
        use super::*;

        proptest! {
            #[test]
            fn succeeds(n in 1..=u64::MAX) {
                let nat = NaturalNumber::new(n).unwrap();
                prop_assert_eq!(WholeNumber::from(nat).get(), n);
            }
        }
    }

    mod whole_to_natural {
        use super::*;
        #[test]
        fn fails_for_0() {
            let w = WholeNumber::new(0);
            let result = NaturalNumber::try_from(w);
            assert_eq!(result, Err("0 is not a natural number.".to_string()))
        }

        proptest! {
            #[test]
            fn passes_for_positive(n in 1..=u64::MAX) {
                let w = WholeNumber::new(n);
                let nat = NaturalNumber::try_from(w).unwrap();
                prop_assert_eq!(nat.get(), n);
            }
        }
    }
    
    mod integer_to_natural {
        use super::*;

        #[test]
        fn fail_on_0() {
            let i = Integer::new(0);
            let result = NaturalNumber::try_from(i);
            assert_eq!(result, Err(String::from("The number is not natural: 0")));
        }
        proptest! {
            #[test]
            fn pass_for_positives(i in 1..i64::MAX) {
                let int = Integer::new(i);
                let result = NaturalNumber::try_from(int);
                assert_eq!(result.unwrap().get(), i as u64);
            }
            #[test]
            fn fail_for_negatives(i in i64::MIN..0) {
                let int = Integer::new(i);
                let result = NaturalNumber::try_from(int);
                assert_eq!(result, Err(format!("The number is not natural: {i}")))
            }
        }
    }

    mod integer_to_whole {
        use super::*;

        #[test]
        fn zero_to_whole_succeeds() {
            let i = Integer::new(0);
            let result = WholeNumber::try_from(i);
            assert_eq!(result.unwrap().get(), 0)
        }

        proptest! {
            #[test]
            fn passes_for_positives(i in 1..i64::MAX) {
                let int = Integer::new(i);
                let result = WholeNumber::try_from(int);
                assert_eq!(result.unwrap().get(), i as u64);
            }
            #[test]
            fn fails_for_negatives(n in i64::MIN..0) {
                let i = Integer::new(n);
                let result = WholeNumber::try_from(i);
                assert_eq!(
                    result,
                    Err(format!("The number is not a whole number: {n}"))
                );
            }
        }
    }
    
    mod whole_to_integer {
        use super::*;

        proptest! {
            #[test]
            fn fail_on_larger_than_i64_max(n in (i64::MAX as u64)+1..u64::MAX) {
                let w = WholeNumber::new(n);
                let result = Integer::try_from(w);
                assert_eq!(result, Err(format!("The whole number is outside of the supported range for a positive integer: {w}")))
            }
            #[test]
            fn pass_on_positive_iu64(n in 0..i64::MAX) {
                let w = WholeNumber::new(n as u64);
                let result = Integer::try_from(w);
                assert_eq!(result.unwrap().get(), n);
            }
        }
    }
}
