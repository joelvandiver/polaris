/**
 * main.js – Polaris web front-end
 *
 * Loads the Rust/WASM polaris-core module and wires up an interactive
 * canvas-based drawing tool that demonstrates the core geometry primitives.
 */

import init, {
  Editor,
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
let editor;
let appState;

function resizeCanvas() {
  canvas.width = container.clientWidth;
  canvas.height = container.clientHeight;
  if (editor) {
    editor.resize_viewport(canvas.width, canvas.height);
    syncState();
    render();
  }
}

window.addEventListener("resize", resizeCanvas);
resizeCanvas();

// ─── State ──────────────────────────────────────────────────────────────────

editor = new Editor(canvas.width, canvas.height);
syncState();

// ─── Tool buttons ───────────────────────────────────────────────────────────

const toolButtons = {
  point: document.getElementById("btn-point"),
  segment: document.getElementById("btn-segment"),
  circle: document.getElementById("btn-circle"),
  line: document.getElementById("btn-line"),
};

Object.entries(toolButtons).forEach(([name, btn]) => {
  btn.addEventListener("click", () => {
    const status = editor.set_tool(name);
    syncState();
    Object.values(toolButtons).forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    render();
    setStatus(status);
  });
});

document.getElementById("btn-clear").addEventListener("click", () => {
  const status = editor.clear();
  syncState();
  render();
  setStatus(status);
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

// ─── Canvas interaction ─────────────────────────────────────────────────────

function canvasPoint(e) {
  const rect = canvas.getBoundingClientRect();
  return { x: e.clientX - rect.left, y: e.clientY - rect.top };
}

canvas.addEventListener("click", (e) => {
  const p = canvasPoint(e);
  const status = editor.click(p.x, p.y, currentStyle());
  syncState();
  render();
  setStatus(status);
});

// Preview pending point on mouse move
canvas.addEventListener("mousemove", (e) => {
  if (!appState?.pending_point) return;
  render();
  const p = canvasPoint(e);
  drawPreview(appState.pending_point, p, appState.tool);
});

// ─── Rendering ──────────────────────────────────────────────────────────────

function render() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawGrid();

  // Parse the scene JSON produced by Rust and draw each element
  if (!appState) {
    return;
  }

  for (const el of appState.scene.elements) {
    if ("PointEl" in el) drawPoint(el.PointEl);
    else if ("SegmentEl" in el) drawSegment(el.SegmentEl);
    else if ("CircleEl" in el) drawCircle(el.CircleEl);
    else if ("LineEl" in el) drawLine(el.LineEl);
  }

  if (appState.pending_point) {
    drawPendingPoint(appState.pending_point);
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

function drawPreview(start, current, tool) {
  ctx.save();
  ctx.setLineDash([4, 4]);
  ctx.strokeStyle = "#a0c4ff88";
  ctx.fillStyle = "#a0c4ff22";
  ctx.lineWidth = 1;

  if (tool === "circle") {
    const radius = Math.hypot(current.x - start.x, current.y - start.y);
    ctx.beginPath();
    ctx.arc(start.x, start.y, radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  } else {
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(current.x, current.y);
    ctx.stroke();
  }

  ctx.restore();
}

// ─── Sidebar ────────────────────────────────────────────────────────────────

function updateSidebar() {
  if (!appState) {
    elementListEl.innerHTML = "";
    return;
  }

  elementListEl.innerHTML = appState.elements
    .map(
      (el, i) =>
        `<div class="element-item">${i + 1}. <strong>${el.element_type}</strong> ${el.label}</div>`
    )
    .join("");
}

// ─── Status bar ─────────────────────────────────────────────────────────────

function setStatus(msg) {
  statusEl.textContent = msg;
}

function syncState() {
  appState = JSON.parse(editor.to_json());
  updateSidebar();
}

// ─── Demo scene ─────────────────────────────────────────────────────────────

function loadDemoScene() {
  const status = editor.load_demo_scene();
  syncState();
  render();
  setStatus(status);
}

loadDemoScene();
