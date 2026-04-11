#!/usr/bin/env bash
# build.sh – build the Polaris WASM package and copy it into web/pkg/
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔨  Building polaris-core → WASM..."
wasm-pack build crates/polaris-core \
  --target web \
  --out-dir "$(pwd)/web/pkg"

echo "✅  Build complete – open web/index.html in a browser (requires a local server)."
echo ""
echo "    Quick server options:"
echo "      python3 -m http.server 8080 --directory web"
echo "      npx serve web"
