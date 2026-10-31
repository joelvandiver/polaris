// TODO: Update tui
  // TODO: Render input box and result with ratatui
  // TODO: Add event loop for input, Enter, and quit
  
use polaris_math::geometry::{Point, distance};

/// Parse "ax ay bx by" and return the distance between the two points.
pub fn distance_from_input(input: &str) -> Result<f64, String> {
    let nums = input
        .split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|_| format!("not a number: {s}")))
        .collect::<Result<Vec<_>, _>>()?;
    let &[ax, ay, bx, by] = nums.as_slice() else {
        return Err(format!("expected 4 numbers, got {}", nums.len()));
    };
    Ok(distance(&Point { x: ax, y: ay }, &Point { x: bx, y: by }))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parses_four_numbers_into_distance() {
        assert_eq!(distance_from_input("0 0 3 4"), Ok(5.0));
    }

    #[test]
    fn rejects_wrong_count() {
        assert!(distance_from_input("0 0 3").is_err());
    }

    #[test]
    fn rejects_non_numbers() {
        assert!(distance_from_input("0 0 a 4").is_err());
    }
}
