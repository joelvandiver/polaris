/**
 * main.js – Polaris web front-end
 *
 * Loads the Rust/WASM polaris-core module and wires up an interactive
 * canvas-based drawing tool that demonstrates the core geometry primitives.
 */

import init, {
  Point,
  Segment,
  Circle,
  Line,
  Scene,
  Style,
  Color,
} from "./pkg/polaris_core.js";

// ─── Initialise WASM ────────────────────────────────────────────────────────

await init();

// ─── Canvas setup ───────────────────────────────────────────────────────────

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const container = document.getElementById("canvas-container");
const statusEl = document.getElementById("status");
const elementListEl = document.getElementById("element-list");
let scene;

function resizeCanvas() {
  canvas.width = container.clientWidth;
  canvas.height = container.clientHeight;
  if (scene) {
    render();
  }
}

window.addEventListener("resize", resizeCanvas);
resizeCanvas();

// ─── State ──────────────────────────────────────────────────────────────────

scene = new Scene(canvas.width, canvas.height);
let tool = "point"; // 'point' | 'segment' | 'circle' | 'line'
let pendingPoint = null; // first click while drawing a two-click shape: {x, y}
let elements = []; // plain JS records for the sidebar

// ─── Tool buttons ───────────────────────────────────────────────────────────

const toolButtons = {
  point: document.getElementById("btn-point"),
  segment: document.getElementById("btn-segment"),
  circle: document.getElementById("btn-circle"),
  line: document.getElementById("btn-line"),
};

Object.entries(toolButtons).forEach(([name, btn]) => {
  btn.addEventListener("click", () => {
    tool = name;
    pendingPoint = null;
    Object.values(toolButtons).forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    setStatus(`Tool: ${name} – click on the canvas`);
  });
});

document.getElementById("btn-clear").addEventListener("click", () => {
  scene.clear();
  elements = [];
  pendingPoint = null;
  updateSidebar();
  render();
  setStatus("Canvas cleared");
});

// ─── Style helpers ──────────────────────────────────────────────────────────

function hexToRgb(hex) {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return { r, g, b };
}

function currentStyle() {
  const strokeHex = document.getElementById("stroke-color").value;
  const fillHex = document.getElementById("fill-color").value;
  const strokeWidth = parseFloat(
    document.getElementById("stroke-width").value
  );
  const s = hexToRgb(strokeHex);
  const f = hexToRgb(fillHex);
  return new Style(
    new Color(s.r, s.g, s.b, 255),
    new Color(f.r, f.g, f.b, 60),
    strokeWidth
  );
}

function makeStyle(stroke, fill, strokeWidth) {
  return new Style(
    new Color(stroke.r, stroke.g, stroke.b, stroke.a),
    new Color(fill.r, fill.g, fill.b, fill.a),
    strokeWidth
  );
}

// ─── Canvas interaction ─────────────────────────────────────────────────────

function canvasPoint(e) {
  const rect = canvas.getBoundingClientRect();
  return { x: e.clientX - rect.left, y: e.clientY - rect.top };
}

canvas.addEventListener("click", (e) => {
  const p = canvasPoint(e);
  const style = currentStyle();

  if (tool === "point") {
    scene.add_point(new Point(p.x, p.y), style);
    elements.push({ type: "Point", label: `(${p.x.toFixed(1)}, ${p.y.toFixed(1)})` });
    updateSidebar();
    render();
    setStatus(`Point added at (${p.x.toFixed(1)}, ${p.y.toFixed(1)})`);
    return;
  }

  if (!pendingPoint) {
    pendingPoint = p;
    const hint =
      tool === "segment"
        ? "Click a second point to complete the segment"
        : tool === "circle"
        ? "Click to set the radius end-point"
        : "Click a second point to define the line direction";
    setStatus(hint);
    return;
  }

  const p1 = pendingPoint;
  const p2 = p;
  pendingPoint = null;

  if (tool === "segment") {
    const seg = new Segment(new Point(p1.x, p1.y), new Point(p2.x, p2.y));
    scene.add_segment(seg, style);
    elements.push({
      type: "Segment",
      label: `(${p1.x.toFixed(1)},${p1.y.toFixed(1)}) → (${p2.x.toFixed(1)},${p2.y.toFixed(1)})`,
    });
  } else if (tool === "circle") {
    const radius = Math.hypot(p2.x - p1.x, p2.y - p1.y);
    const circle = new Circle(new Point(p1.x, p1.y), radius);
    scene.add_circle(circle, style);
    elements.push({ type: "Circle", label: `r=${radius.toFixed(1)} @ (${p1.x.toFixed(1)},${p1.y.toFixed(1)})` });
  } else if (tool === "line") {
    const line = new Line(new Point(p1.x, p1.y), new Point(p2.x, p2.y));
    scene.add_line(line, 1, style);
    elements.push({
      type: "Line",
      label: `through (${p1.x.toFixed(1)},${p1.y.toFixed(1)}) → (${p2.x.toFixed(1)},${p2.y.toFixed(1)})`,
    });
  }

  updateSidebar();
  render();
  setStatus(`${tool.charAt(0).toUpperCase() + tool.slice(1)} added`);
});

// Preview pending point on mouse move
canvas.addEventListener("mousemove", (e) => {
  if (!pendingPoint) return;
  render();
  const p = canvasPoint(e);
  ctx.save();
  ctx.setLineDash([4, 4]);
  ctx.strokeStyle = "#a0c4ff88";
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(pendingPoint.x, pendingPoint.y);
  ctx.lineTo(p.x, p.y);
  ctx.stroke();
  ctx.restore();
});

// ─── Rendering ──────────────────────────────────────────────────────────────

function render() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawGrid();

  // Parse the scene JSON produced by Rust and draw each element
  let sceneData;
  try {
    sceneData = JSON.parse(scene.to_json());
  } catch {
    return;
  }

  for (const el of sceneData.elements) {
    if ("PointEl" in el) drawPoint(el.PointEl);
    else if ("SegmentEl" in el) drawSegment(el.SegmentEl);
    else if ("CircleEl" in el) drawCircle(el.CircleEl);
    else if ("LineEl" in el) drawLine(el.LineEl);
  }

  if (pendingPoint) {
    drawPendingPoint(pendingPoint);
  }
}

function drawGrid() {
  const step = 40;
  ctx.save();
  ctx.strokeStyle = "#1a1a30";
  ctx.lineWidth = 1;
  for (let x = 0; x < canvas.width; x += step) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }
  for (let y = 0; y < canvas.height; y += step) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
  ctx.restore();
}

function styleFromEl(style) {
  const { stroke, fill, stroke_width } = style;
  return {
    strokeStyle: `rgba(${stroke.r},${stroke.g},${stroke.b},${stroke.a / 255})`,
    fillStyle: `rgba(${fill.r},${fill.g},${fill.b},${fill.a / 255})`,
    lineWidth: stroke_width,
  };
}

function applyStyle(s) {
  ctx.strokeStyle = s.strokeStyle;
  ctx.fillStyle = s.fillStyle;
  ctx.lineWidth = s.lineWidth;
}

function drawPoint({ point, style }) {
  const s = styleFromEl(style);
  ctx.save();
  applyStyle(s);
  ctx.beginPath();
  ctx.arc(point.x, point.y, Math.max(3, s.lineWidth * 2), 0, Math.PI * 2);
  ctx.fillStyle = s.strokeStyle;
  ctx.fill();
  ctx.restore();
}

function drawSegment({ segment, style }) {
  const s = styleFromEl(style);
  ctx.save();
  applyStyle(s);
  ctx.beginPath();
  ctx.moveTo(segment.start.x, segment.start.y);
  ctx.lineTo(segment.end.x, segment.end.y);
  ctx.stroke();
  ctx.restore();
}

function drawCircle({ circle, style }) {
  const s = styleFromEl(style);
  ctx.save();
  applyStyle(s);
  ctx.beginPath();
  ctx.arc(circle.center.x, circle.center.y, circle.radius, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  ctx.restore();
}

function drawLine({ line, style }) {
  // Extend an infinite line to fill the viewport
  const s = styleFromEl(style);
  const { p1, p2 } = line;
  const dx = p2.x - p1.x;
  const dy = p2.y - p1.y;
  const scale = Math.max(canvas.width, canvas.height) * 10;
  const len = Math.hypot(dx, dy) || 1;
  const ux = (dx / len) * scale;
  const uy = (dy / len) * scale;
  ctx.save();
  applyStyle(s);
  ctx.beginPath();
  ctx.moveTo(p1.x - ux, p1.y - uy);
  ctx.lineTo(p1.x + ux, p1.y + uy);
  ctx.stroke();
  ctx.restore();
}

function drawPendingPoint(p) {
  ctx.save();
  ctx.fillStyle = "#a0c4ff";
  ctx.beginPath();
  ctx.arc(p.x, p.y, 5, 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();
}

// ─── Sidebar ────────────────────────────────────────────────────────────────

function updateSidebar() {
  elementListEl.innerHTML = elements
    .map(
      (el, i) =>
        `<div class="element-item">${i + 1}. <strong>${el.type}</strong> ${el.label}</div>`
    )
    .join("");
}

// ─── Status bar ─────────────────────────────────────────────────────────────

function setStatus(msg) {
  statusEl.textContent = msg;
}

// ─── Demo scene ─────────────────────────────────────────────────────────────

function loadDemoScene() {
  const w = canvas.width;
  const h = canvas.height;
  const blueStroke = { r: 100, g: 180, b: 255, a: 255 };
  const blueFill = { r: 60, g: 80, b: 200, a: 40 };
  const goldStroke = { r: 255, g: 200, b: 60, a: 255 };
  const goldFill = { r: 200, g: 150, b: 0, a: 30 };
  const whiteStroke = { r: 220, g: 220, b: 255, a: 220 };
  const transparent = { r: 0, g: 0, b: 0, a: 0 };

  // Vesica Piscis – two overlapping circles sharing a radius
  const cx = w / 2;
  const cy = h / 2;
  const r = Math.min(w, h) * 0.18;

  const c1 = new Circle(new Point(cx - r / 2, cy), r);
  const c2 = new Circle(new Point(cx + r / 2, cy), r);
  scene.add_circle(c1, makeStyle(blueStroke, blueFill, 1.5));
  scene.add_circle(c2, makeStyle(goldStroke, goldFill, 1.5));

  // Centre points
  scene.add_point(
    new Point(cx - r / 2, cy),
    makeStyle(whiteStroke, transparent, 1)
  );
  scene.add_point(
    new Point(cx + r / 2, cy),
    makeStyle(whiteStroke, transparent, 1)
  );

  // Connecting segment
  scene.add_segment(
    new Segment(new Point(cx - r / 2, cy), new Point(cx + r / 2, cy)),
    makeStyle(whiteStroke, transparent, 1)
  );

  elements.push({ type: "Circle", label: `Vesica (left)  r=${r.toFixed(1)}` });
  elements.push({ type: "Circle", label: `Vesica (right) r=${r.toFixed(1)}` });
  elements.push({ type: "Point", label: `Centre 1` });
  elements.push({ type: "Point", label: `Centre 2` });
  elements.push({ type: "Segment", label: `Axis` });

  updateSidebar();
  render();
  setStatus("Demo: Vesica Piscis – a fundamental shape in sacred geometry");
}

loadDemoScene();
