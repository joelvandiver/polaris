# Next: get CI green

CI on `main` is red right now, so the nextest switch has nothing to land on. Fix these first, in this order.

1. **Format check fails.** `cargo fmt --all --check` flags these files:
   - `crates/polaris-math/src/geometry.rs`: trailing space after `// pub fn`
   - `crates/polaris-math/src/lib.rs`: module order
   - `crates/polaris-math/src/number.rs`: empty struct body

   Run `cargo fmt --all`. The `./run ci` script runs `cargo fmt --all` without `--check`, so it fixes formatting locally and hides the drift. Change it to `--check` so it matches CI.

2. **The wasm-check job builds crates that don't exist.** `.github/workflows/ci.yml`, last line:
   ```yaml
   - name: Build math + wasm for wasm32
     run: cargo build -p polaris-math -p polaris-wasm --target wasm32-unknown-unknown
   ```
   (It currently builds `pulsar-core` and `pulsar-syntax`.)

3. **Then do the TODO: add nextest to CI.** In the `rust` job:
   ```yaml
   - uses: taiki-e/install-action@nextest
   - name: Test
     run: |
       cargo nextest run --workspace
       cargo test --workspace --doc
   ```
   nextest doesn't run doctests, so keep the `--doc` line. Make the same change in `./run ci`.

## Notes for later (not now)
- `polaris-wasm/src/main.rs` is a leftover hello-world. Delete it; a cdylib crate doesn't need a binary.
- The `wasm_bindgen_test` in `polaris-wasm` won't run under `cargo test` on the host target. It needs `wasm-pack test --node` or something similar in CI to actually run.
- `polaris-core` is still the `add()` template code. Decide what it's for (the construction graph or scene model?) or remove it.
- `Line` and `Segment` have the same fields, so a degenerate line (`point_1 == point_2`) isn't ruled out. Settle this before building on them. The roadmap's next primitive is `Circle`.
