use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{Circle, Color, Line, Point, Scene, Segment, Style};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum Tool {
    Point,
    Segment,
    Circle,
    Line,
}

impl Tool {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "point" => Some(Self::Point),
            "segment" => Some(Self::Segment),
            "circle" => Some(Self::Circle),
            "line" => Some(Self::Line),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Point => "point",
            Self::Segment => "segment",
            Self::Circle => "circle",
            Self::Line => "line",
        }
    }

    fn selection_status(self) -> String {
        format!("Tool: {} - click on the canvas", self.as_str())
    }

    fn pending_status(self) -> &'static str {
        match self {
            Self::Point => "Click on the canvas",
            Self::Segment => "Click a second point to complete the segment",
            Self::Circle => "Click to set the radius end-point",
            Self::Line => "Click a second point to define the line direction",
        }
    }

    fn added_status(self) -> &'static str {
        match self {
            Self::Point => "Point added",
            Self::Segment => "Segment added",
            Self::Circle => "Circle added",
            Self::Line => "Line added",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ElementSummary {
    element_type: String,
    label: String,
}

#[derive(Serialize)]
struct EditorSnapshot<'a> {
    scene: &'a Scene,
    elements: &'a [ElementSummary],
    pending_point: Option<Point>,
    tool: &'static str,
}

#[wasm_bindgen]
pub struct Editor {
    scene: Scene,
    tool: Tool,
    pending_point: Option<Point>,
    elements: Vec<ElementSummary>,
}

#[wasm_bindgen]
impl Editor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            scene: Scene::new(width, height),
            tool: Tool::Point,
            pending_point: None,
            elements: Vec::new(),
        }
    }

    pub fn set_tool(&mut self, tool: &str) -> Result<String, JsValue> {
        let tool = Tool::from_str(tool)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown tool: {tool}")))?;
        self.tool = tool;
        self.pending_point = None;
        Ok(tool.selection_status())
    }

    pub fn resize_viewport(&mut self, width: f64, height: f64) {
        self.scene.resize_viewport(width, height);
    }

    pub fn clear(&mut self) -> String {
        self.scene.clear();
        self.pending_point = None;
        self.elements.clear();
        "Canvas cleared".to_string()
    }

    pub fn click(&mut self, x: f64, y: f64, style: Style) -> String {
        let point = Point::new(x, y);

        if self.tool == Tool::Point {
            self.scene.add_point(point, style);
            self.push_summary(
                "Point",
                format!("({}, {})", fmt1(x), fmt1(y)),
            );
            return format!("Point added at ({}, {})", fmt1(x), fmt1(y));
        }

        if self.pending_point.is_none() {
            self.pending_point = Some(point);
            return self.tool.pending_status().to_string();
        }

        let start = self.pending_point.take().unwrap();

        match self.tool {
            Tool::Point => unreachable!(),
            Tool::Segment => {
                self.scene
                    .add_segment(Segment::new(start, point), style);
                self.push_summary(
                    "Segment",
                    format!(
                        "({},{}) -> ({},{})",
                        fmt1(start.x),
                        fmt1(start.y),
                        fmt1(point.x),
                        fmt1(point.y)
                    ),
                );
            }
            Tool::Circle => {
                let radius = start.distance_to(&point);
                self.scene.add_circle(Circle::new(start, radius), style);
                self.push_summary(
                    "Circle",
                    format!("r={} @ ({},{})", fmt1(radius), fmt1(start.x), fmt1(start.y)),
                );
            }
            Tool::Line => {
                self.scene.add_line(Line::new(start, point), 1.0, style);
                self.push_summary(
                    "Line",
                    format!(
                        "through ({},{}) -> ({},{})",
                        fmt1(start.x),
                        fmt1(start.y),
                        fmt1(point.x),
                        fmt1(point.y)
                    ),
                );
            }
        }

        self.tool.added_status().to_string()
    }

    pub fn load_demo_scene(&mut self) -> String {
        self.scene.clear();
        self.pending_point = None;
        self.elements.clear();

        let cx = self.scene.width() / 2.0;
        let cy = self.scene.height() / 2.0;
        let r = self.scene.width().min(self.scene.height()) * 0.18;

        let blue_stroke = Color::new(100, 180, 255, 255);
        let blue_fill = Color::new(60, 80, 200, 40);
        let gold_stroke = Color::new(255, 200, 60, 255);
        let gold_fill = Color::new(200, 150, 0, 30);
        let white_stroke = Color::new(220, 220, 255, 220);
        let transparent = Color::new(0, 0, 0, 0);

        self.scene.add_circle(
            Circle::new(Point::new(cx - r / 2.0, cy), r),
            Style::new(blue_stroke, blue_fill, 1.5),
        );
        self.scene.add_circle(
            Circle::new(Point::new(cx + r / 2.0, cy), r),
            Style::new(gold_stroke, gold_fill, 1.5),
        );

        self.scene.add_point(
            Point::new(cx - r / 2.0, cy),
            Style::new(white_stroke, transparent, 1.0),
        );
        self.scene.add_point(
            Point::new(cx + r / 2.0, cy),
            Style::new(white_stroke, transparent, 1.0),
        );
        self.scene.add_segment(
            Segment::new(Point::new(cx - r / 2.0, cy), Point::new(cx + r / 2.0, cy)),
            Style::new(white_stroke, transparent, 1.0),
        );

        self.push_summary("Circle", format!("Vesica (left)  r={}", fmt1(r)));
        self.push_summary("Circle", format!("Vesica (right) r={}", fmt1(r)));
        self.push_summary("Point", "Centre 1".to_string());
        self.push_summary("Point", "Centre 2".to_string());
        self.push_summary("Segment", "Axis".to_string());

        "Demo: Vesica Piscis - a fundamental shape in sacred geometry".to_string()
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        let snapshot = EditorSnapshot {
            scene: &self.scene,
            elements: &self.elements,
            pending_point: self.pending_point,
            tool: self.tool.as_str(),
        };

        serde_json::to_string(&snapshot).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Editor {
    fn push_summary(&mut self, element_type: &str, label: String) {
        self.elements.push(ElementSummary {
            element_type: element_type.to_string(),
            label,
        });
    }
}

fn fmt1(value: f64) -> String {
    format!("{value:.1}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn style() -> Style {
        Style::new(Color::new(0, 0, 0, 255), Color::new(0, 0, 0, 0), 1.0)
    }

    #[test]
    fn circle_creation_requires_two_clicks() {
        let mut editor = Editor::new(640.0, 480.0);
        editor.set_tool("circle").unwrap();

        assert_eq!(editor.click(10.0, 20.0, style()), "Click to set the radius end-point");
        assert_eq!(editor.click(13.0, 24.0, style()), "Circle added");

        let json = editor.to_json().unwrap();
        assert!(json.contains("\"CircleEl\""));
        assert!(json.contains("\"pending_point\":null"));
    }

    #[test]
    fn tool_change_clears_pending_point() {
        let mut editor = Editor::new(640.0, 480.0);
        editor.set_tool("segment").unwrap();
        editor.click(10.0, 20.0, style());

        editor.set_tool("line").unwrap();
        let json = editor.to_json().unwrap();
        assert!(json.contains("\"pending_point\":null"));
        assert!(json.contains("\"tool\":\"line\""));
    }
}
