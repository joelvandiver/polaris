# Polaris MVP Roadmap

**Decisions** (2026-07-18): TypeScript/React UI + Rust WASM geometry kernel; SVG rendering; MVP scope is classic constructions (points, lines, segments, circles, intersections, live drag). See ADR-001 (to be written in M0).

**Standards**: every step follows TDD (Red → Green → Refactor), full unit + integration coverage, CI-verifiable, documented per `/dev-workflow:document`.

## Architecture

```
polaris/
├── crates/
│   ├── polaris-core/   # Pure Rust geometry kernel. No wasm deps. cargo test.
│   └── polaris-wasm/   # Thin wasm-bindgen wrapper over polaris-core.
├── web/                # Vite + React + TS. SVG renderer + tools. Vitest/RTL + Playwright.
└── .github/workflows/  # CI
```

Core idea: the kernel owns the **document** — a dependency graph of geometric objects (free points → lines/circles → intersections). Moving a free point triggers recompute of dependents. The UI is a stateless view over kernel snapshots plus tool interactions.

## Test strategy (full scope)

| Layer | Kind | Tooling | CI job |
|---|---|---|---|
| Kernel math & graph | Unit + property | `cargo test`, `proptest` | `rust` |
| WASM boundary | Integration | `wasm-pack test --headless --chrome` | `wasm` |
| UI components & tools | Unit/component | Vitest + React Testing Library | `web` |
| JS ↔ WASM | Integration | Vitest against built wasm pkg | `web` |
| Full app (Sketchpad loop) | E2E | Playwright | `e2e` |
| Style/health | Lint | `cargo fmt --check`, `clippy -D warnings`, `eslint`, `tsc --noEmit` | `rust`/`web` |

---

## M0 — Walking skeleton + CI

Goal: empty-but-wired app; every test layer runs in CI and fails the build when broken.

1. **Red**: three failing tests
   - `polaris-core`: `distance(Point, Point)` unit test (exact + float tolerance case).
   - `web`: Vitest test that imports the wasm pkg and calls `distance`.
   - Playwright: app serves and renders an `<svg data-testid="canvas">`.
2. **Green**: Cargo workspace, `wasm-pack` build, Vite + React scaffold, minimal `distance` impl, empty SVG canvas component.
3. **Refactor**: shared build scripts, `justfile`/npm scripts for one-command dev & test.
4. **CI**: GitHub Actions workflow with `rust`, `wasm`, `web`, `e2e` jobs + caching. All four required.
5. **Document**: ADR-001 (architecture: React UI + Rust kernel, SVG), README quickstart (build/test commands).

## M1 — Geometry kernel: primitives & dependency graph

Goal: correct, fully unit-tested geometry with no UI.

1. **Red** (per feature, before implementation):
   - Primitives: `Point`, `Line`, `Segment`, `Circle` with ids; construction & accessor tests.
   - Numeric tolerance: epsilon-comparison tests (near-parallel, near-tangent).
   - Intersections: line–line (incl. parallel/collinear), line–circle (secant/tangent/miss), circle–circle (two/one/zero points, concentric). Property tests: intersection points lie on both objects.
   - Dependency graph: midpoint/intersection recompute when an ancestor moves; no stale values; cycle rejection.
2. **Green**: implement kernel; document model = `Document { objects, dependencies }` with topological recompute.
3. **Refactor**: extract numeric module (`eps`, comparisons) once tests pin behavior.
4. **CI**: `rust` job covers all of it; add `proptest` runs.
5. **Document**: ADR-002 numeric tolerance strategy; kernel module doc comments.

## M2 — WASM boundary

Goal: kernel usable from JS with a stable, tested API.

1. **Red**:
   - wasm-bindgen-tests: create document, add point/line/circle, move point, read scene snapshot.
   - Vitest integration: same flow through the built npm pkg; snapshot shape matches TS types.
2. **Green**: `polaris-wasm` API — `newDocument()`, `addPoint/addLine/addCircle/addIntersection`, `movePoint(id, x, y)`, `sceneSnapshot()` (serde-wasm-bindgen). Generated TS types.
3. **Refactor**: minimize copying across the boundary (single snapshot call per frame).
4. **CI**: `wasm` job (headless Chrome) + `web` integration tests.
5. **Document**: ADR-003 boundary format (snapshot-per-frame vs. mutation events); API reference in `web/README`.

## M3 — React UI: SVG renderer + tools

Goal: draw and manipulate constructions.

1. **Red** (component tests first):
   - `<Scene>` renders snapshot objects as SVG elements with stable keys.
   - Hit-testing: nearest object within pixel tolerance (unit-test the pure function).
   - Tools as a state machine (Select/Point/Line/Segment/Circle/Intersect): pointer-event sequences → expected kernel calls (mock kernel).
   - Drag: pointerdown→move→up on a point issues `movePoint` and re-renders.
2. **Green**: toolbar, tool state machine, SVG renderer, viewport (pan/zoom) transform.
3. **Refactor**: rendering perf — rAF-batched snapshot reads, memoized layers.
4. **CI**: `web` job.
5. **Document**: tool interaction spec in `docs/ui-tools.md`.

## M4 — The Sketchpad loop, end-to-end

Goal: the defining demo works and is protected by E2E tests.

1. **Red** (Playwright):
   - Construct a triangle from three points and segments; add side midpoints; drag a vertex → midpoints and intersections follow.
   - Undo/redo restores exact scene.
   - Save → reload → scene round-trips.
2. **Green**: command stack in kernel (undo/redo); JSON serialize/deserialize of `Document` (with serde round-trip unit tests written first).
3. **Refactor**: consolidate command handling.
4. **CI**: `e2e` job; drag simulation stability (retry policy documented, not silent).
5. **Document**: user-facing "getting started" doc with the triangle walkthrough; ADR-004 persistence format.

## M5 — Post-MVP (backlog, re-plan before starting)

Measurements (length/angle/area) · labels & styling · perpendicular/parallel/bisector constructions · export SVG/PNG · deploy to GitHub Pages from CI · loci/traces · constraint solver (revisit "constraint-first" idea).

## Risks & mitigations

- **Numeric robustness** (near-tangent, near-parallel): dedicated tolerance module + property tests early (M1), not retrofitted.
- **SVG performance under drag**: fine for MVP scale; rAF batching in M3; escape hatch = swap `<Scene>` to Canvas behind the same snapshot interface.
- **Two-toolchain CI time**: cache cargo + node; keep `e2e` on the built artifact only.
- **wasm-pack + Vite integration quirks**: resolved in M0 while the app is trivial, not mid-feature.

## Definition of done (every task)

tests written first → tests pass → CI green → change documented (`/dev-workflow:document`).
