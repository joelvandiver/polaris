# ⭐ Polaris

> A Rust + WebAssembly geometric vector graphics application inspired by
> Geometer's Sketchpad and GeoGebra, with a strong focus on **artistic output**.

---

## Overview

Polaris lets you construct precise geometric figures—points, segments, lines,
and circles—directly in the browser.  The rendering and geometry engine is
written in **Rust** and compiled to **WebAssembly** via
[wasm-pack](https://rustwasm.github.io/wasm-pack/), giving near-native
performance for complex constructions.

---

## Features (MVP)

| Feature | Status |
|---------|--------|
| `Point` primitive | ✅ |
| `Segment` primitive | ✅ |
| `Line` (infinite) primitive | ✅ |
| `Circle` primitive | ✅ |
| `Scene` – styled element container | ✅ |
| `Style` / `Color` for visual customisation | ✅ |
| Interactive HTML5 Canvas renderer | ✅ |
| Demo scene (Vesica Piscis) | ✅ |
| Persistent undo / redo | 🔜 |
| SVG export | 🔜 |
| Angle / ratio constraints | 🔜 |
| Locus tracing | 🔜 |
| Parametric constructions | 🔜 |

---

## Architecture

```
polaris/
├── Cargo.toml                   ← Rust workspace
├── build.sh                     ← one-command build
├── crates/
│   └── polaris-core/            ← Rust library crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── geometry/
│           │   ├── point.rs     ← Point   (2-D coordinate)
│           │   ├── segment.rs   ← Segment (bounded line)
│           │   ├── line.rs      ← Line    (infinite)
│           │   └── circle.rs    ← Circle  (centre + radius)
│           └── scene.rs         ← Scene, Style, Color
└── web/
    ├── index.html               ← Shell page
    ├── main.js                  ← ES module – WASM host + renderer
    └── pkg/                     ← wasm-pack output (generated)
```

**Data flow**

```
User click → main.js → Rust WASM (add geometry to Scene)
                     ← scene.to_json() → Canvas 2D renderer
```

---

## Roadmap

### Milestone 1 – Foundation (current)
- [x] Rust workspace with `wasm32-unknown-unknown` target
- [x] Core geometry types with unit tests
- [x] WASM bindings via `wasm-bindgen`
- [x] Interactive canvas front-end
- [x] Demo construction (Vesica Piscis)

### Milestone 2 – Construction Tools
- [ ] Intersection detection (line/circle, circle/circle)
- [ ] Perpendicular & parallel line tools
- [ ] Angle bisector
- [ ] Midpoint construction
- [ ] Undo / redo stack

### Milestone 3 – Artistic Output
- [ ] SVG export
- [ ] Configurable colour themes / palettes
- [ ] Stroke dash patterns
- [ ] Gradient fills
- [ ] Animated constructions (parametric time)

### Milestone 4 – Advanced Geometry
- [ ] Polygon tool (regular & freeform)
- [ ] Conic sections (ellipse, parabola, hyperbola)
- [ ] Transformation tools (rotate, reflect, scale)
- [ ] Locus tracing
- [ ] Projective & inversive geometry helpers

### Milestone 5 – User Experience
- [ ] Save / load scenes (JSON)
- [ ] Drag-to-move elements
- [ ] Snap-to-grid / snap-to-point
- [ ] Measurement labels
- [ ] Keyboard shortcuts

---

## Getting Started

### Prerequisites

| Tool | Version |
|------|---------|
| Rust (stable) | ≥ 1.70 |
| wasm-pack | ≥ 0.12 |
| Node.js (optional, for dev server) | ≥ 18 |

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack
```

### Build

```bash
./build.sh
```

### Develop

```bash
# Python built-in server
python3 -m http.server 8080 --directory web

# Or npx
npx serve web
```

Then open <http://localhost:8080>.

### Test

```bash
cargo test
```

---

## Technical Decisions

| Decision | Rationale |
|----------|-----------|
| **Rust + WASM** | Near-native performance for geometry computations; strong type safety |
| **wasm-bindgen** | Ergonomic JS↔Rust interop with TypeScript declarations |
| **serde + JSON** | Simple, debuggable scene serialisation without complex WASM memory sharing |
| **HTML5 Canvas** | Portable, zero-dependency renderer for the MVP |
| **No JS framework** | Keeps the build pipeline simple; a framework (e.g. Svelte) can be added later |

---

## Contributing

Issues and pull requests are welcome.  Please discuss significant changes in an
issue first.

## License

MIT
