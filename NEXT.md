# Next

Plan: [docs/plans/001-mvp-roadmap.md](docs/plans/001-mvp-roadmap.md) · Tasks: [TODO.md](TODO.md)

## Now — M0: `web/` Vite + React + TS scaffold (wasm import test first)

1. **Red** — `web/src/wasm.test.ts`: `import init, { distance } from '../pkg/polaris_wasm'` → `await init()`, assert `distance({x:0,y:0},{x:3,y:4}) === 5`. Fails: no `web/`, no exported binding.

<!--
Implement a named tuple (?)
Point(f64, f64)
-->

2. **Green** — add `#[wasm_bindgen] pub fn distance(...)` to `crates/polaris-wasm/src/lib.rs` (currently test-only, no public API); `wasm-pack build --target web --out-dir ../../web/pkg`; `npm create vite@latest web -- --template react-ts`; Vitest + `vite-plugin-wasm`.
3. **CI** — new `web` job: `npm ci`, `tsc --noEmit`, `eslint`, `vitest run`; needs the wasm build step before it (reuse/extend `wasm-check`, which today only compiles for `wasm32`, no `wasm-pack`).
4. **Doc** — README quickstart: build wasm → run web tests.

**Why this:** Next M0 item, and it closes the last unproven layer — nothing yet verifies JS↔WASM works, the roadmap's flagged integration risk.

## On deck

- Playwright smoke test: `<svg data-testid="canvas">` renders (+ `e2e` job).
- Make `rust` / `wasm` / `web` / `e2e` required checks on `main`.
- ADR-001 architecture + README quickstart.

## Drift

TODO line 9 (`crates/polaris-wasm` + wasm-pack build) is unchecked but the crate and a `wasm-check` CI job exist. Partial: crate wired, but no `wasm-pack` build and no public binding yet. Consider splitting into "crate wired ✓" / "wasm-pack build ✗".
