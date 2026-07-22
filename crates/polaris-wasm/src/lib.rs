#[cfg(test)]
mod tests {
    use polaris_core::{Point, distance};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn distance_3_4_5_across_boundary() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert_eq!(distance(&a, &b), 5.0);
    }
}
