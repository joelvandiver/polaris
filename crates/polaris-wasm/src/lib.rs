use polaris_core::{self, Point};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn distance(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    polaris_core::distance(&Point { x: ax, y: ay }, &Point { x: bx, y: by })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn distance_3_4_5_across_boundary() {
        assert_eq!(distance(0.0, 0.0, 3.0, 4.0), 5.0);
    }
}
