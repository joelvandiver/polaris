# Next: Get CI green (~10 min)

**Why:** `main` fails CI in two places: formatting, and the wasm job builds crates that don't exist. Fix that before adding nextest, or you won't be able to tell whether nextest works.

## Steps

1. **Reproduce the failure ("red").** The wasm job fails because the crate names are wrong:
   ```sh
   cargo build -p pulsar-core -p pulsar-syntax --target wasm32-unknown-unknown
   # error: package ID specification `pulsar-core` did not match any packages
   ```
   (If the target is missing: `rustup target add wasm32-unknown-unknown`.)

2. **Fix the wasm job.** In `.github/workflows/ci.yml`, replace the last step (lines 31–32):
   ```yaml
         - name: Build math + wasm for wasm32
           run: cargo build -p polaris-math -p polaris-wasm --target wasm32-unknown-unknown
   ```

3. **Make `./run ci` check formatting the way CI does.** In `run`, line 16:
   ```sh
           cargo fmt --all --check
   ```
   Right now the script reformats files instead of failing, which is how the format drift got past you.

4. **Verify locally ("green"):**
   ```sh
   ./run ci
   cargo build -p polaris-math -p polaris-wasm --target wasm32-unknown-unknown
   ```

5. **Commit and push.** Include the formatting changes already sitting in your working tree:
   ```sh
   git add .github/workflows/ci.yml run crates/
   git commit -m "Fix CI: correct wasm crate names, fmt --check in run script"
   git push
   ```

## Done when
- [ ] `./run ci` passes with `--check`
- [ ] The wasm32 build of `polaris-math` and `polaris-wasm` succeeds
- [ ] Both jobs (`rust` and `wasm-check`) are green on GitHub Actions

## On deck
- **Add nextest to CI** (the TODO item): add `taiki-e/install-action@nextest`, then run `cargo nextest run --workspace` and `cargo test --workspace --doc`. Make the same change in `./run ci`.
- **Delete `crates/polaris-wasm/src/main.rs`.** It's a leftover hello-world; the cdylib crate doesn't need a binary.
- **Start `Circle` in `polaris-math::geometry`.** Write tests first: `Circle { center, radius }`, `contains(&Point)`, and reject a negative radius.
