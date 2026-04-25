use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::geometry::{Circle, Line, Point, Segment};

/// The color of a drawn element, in RGBA form.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[wasm_bindgen]
impl Color {
    /// Create a new `Color` from RGBA components.
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Return the CSS `rgba(…)` string for this color.
    pub fn to_css(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a as f64 / 255.0)
    }
}

/// Style applied to a drawn element.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Style {
    pub stroke: Color,
    pub fill: Color,
    pub stroke_width: f64,
}

#[wasm_bindgen]
impl Style {
    /// Create a new `Style`.
    #[wasm_bindgen(constructor)]
    pub fn new(stroke: Color, fill: Color, stroke_width: f64) -> Self {
        Self { stroke, fill, stroke_width }
    }

    /// A sensible default style – black stroke, transparent fill, 1px.
    pub fn default_style() -> Style {
        Style {
            stroke: Color::new(0, 0, 0, 255),
            fill: Color::new(0, 0, 0, 0),
            stroke_width: 1.0,
        }
    }
}

/// A discriminated union of drawable geometry elements.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Element {
    PointEl { point: Point, style: Style },
    SegmentEl { segment: Segment, style: Style },
    LineEl { line: Line, width: f64, style: Style },
    CircleEl { circle: Circle, style: Style },
}

/// The main scene – a collection of styled geometry elements.
///
/// From JavaScript you add geometry through the typed helper methods and
/// retrieve the serialized scene via `to_json()` for the renderer to consume.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    elements: Vec<Element>,
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl Scene {
    /// Create an empty scene with the given viewport dimensions.
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Self {
        Self { elements: Vec::new(), width, height }
    }

    /// Viewport width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Viewport height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Update the viewport dimensions used by the renderer.
    pub fn resize_viewport(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    /// Number of elements in the scene.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` when there are no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Add a point to the scene.
    pub fn add_point(&mut self, point: Point, style: Style) {
        self.elements.push(Element::PointEl { point, style });
    }

    /// Add a line segment to the scene.
    pub fn add_segment(&mut self, segment: Segment, style: Style) {
        self.elements.push(Element::SegmentEl { segment, style });
    }

    /// Add an infinite line to the scene (clipped to the viewport in the renderer).
    pub fn add_line(&mut self, line: Line, width: f64, style: Style) {
        self.elements.push(Element::LineEl { line, width, style });
    }

    /// Add a circle to the scene.
    pub fn add_circle(&mut self, circle: Circle, style: Style) {
        self.elements.push(Element::CircleEl { circle, style });
    }

    /// Remove all elements from the scene.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Serialize the scene to a JSON string so the JavaScript renderer can
    /// consume it without needing direct access to the Rust structs.
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
