# Polaris TODO

Plan: [docs/plans/001-mvp-roadmap.md](docs/plans/001-mvp-roadmap.md). A task is done only when: tests first → passing → CI green → documented.

## M0 — Walking skeleton + CI  ← current

- [ ] Cargo workspace with `crates/polaris-core` (failing `distance` test first)
- [ ] `crates/polaris-wasm` + wasm-pack build
- [ ] `web/` Vite + React + TS scaffold (failing Vitest wasm-import test first)
- [ ] Playwright smoke test: SVG canvas renders
- [ ] GitHub Actions: `rust`, `wasm`, `web`, `e2e` jobs, all required
- [ ] ADR-001 architecture; README quickstart

## M1 — Geometry kernel

- [ ] Primitives: Point / Line / Segment / Circle (+ids)
- [ ] Numeric tolerance module
- [ ] Intersections: line–line, line–circle, circle–circle (+ property tests)
- [ ] Dependency graph & recompute; cycle rejection
- [ ] ADR-002 numeric tolerance

## M2 — WASM boundary

- [ ] wasm-bindgen API: document CRUD, movePoint, sceneSnapshot
- [ ] wasm-bindgen-tests (headless) + Vitest integration tests
- [ ] Generated TS types
- [ ] ADR-003 boundary format

## M3 — React UI

- [ ] `<Scene>` SVG renderer from snapshot
- [ ] Hit-testing (pure fn, unit-tested)
- [ ] Tool state machine: Select/Point/Line/Segment/Circle/Intersect
- [ ] Drag loop; pan/zoom viewport
- [ ] docs/ui-tools.md

## M4 — Sketchpad loop E2E

- [ ] Playwright: triangle + midpoints drag test
- [ ] Undo/redo (command stack)
- [ ] Save/load JSON round-trip
- [ ] ADR-004 persistence; getting-started walkthrough

## M5 — Post-MVP backlog

Measurements · labels/styles · perpendicular/parallel/bisector · export SVG/PNG · GitHub Pages deploy · traces/loci · constraint solver
