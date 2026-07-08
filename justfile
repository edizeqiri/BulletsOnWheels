godot := "godot"
godot_project := "games/MagicShootout/godot"

build:
    cargo build

build-release:
    cargo build --release

run: build
    {{ godot }} --path {{ godot_project }}

run-release: build-release
    {{ godot }} --path {{ godot_project }}

editor: build
    {{ godot }} --path {{ godot_project }} --editor

watch:
    cargo watch -x "build"

reload-godot: build
    pkill godot || true
    sleep 1
    {{ godot }} --editor --path {{ godot_project }} &

# --- Web (wasm32-unknown-emscripten) ---
# Requires the emscripten SDK (emcc on PATH) and `rust-src` on the toolchain below
# (rustup component add rust-src --toolchain nightly). Nightly is needed for
# -Zbuild-std; panic=abort (in .cargo/config.toml) avoids the Wasm-EH tag import.

wasm_toolchain := "nightly"

# `-Zbuild-std=std,panic_abort` (nightly) rebuilds std with panic=abort too;
# otherwise std's prebuilt objects still import the `__cpp_exception` Wasm-EH tag

# and Godot fails to link it ("tag import requires a WebAssembly.Tag").
wasm:
    cargo +{{ wasm_toolchain }} build -Zbuild-std=std,panic_abort --target wasm32-unknown-emscripten

wasm-release:
    cargo +{{ wasm_toolchain }} build -Zbuild-std=std,panic_abort --release --target wasm32-unknown-emscripten

# Build the debug wasm and export the "Web" preset to godot/exports/.
export-web: wasm
    mkdir -p {{ godot_project }}/exports
    {{ godot }} --headless --path {{ godot_project }} --export-debug "Web" exports/BulletsOnWheels.html

# Build the release wasm and export the "Web" preset to godot/exports/.

# Copies to index.html so GitHub Pages serves the game at the site root.
export-web-release: wasm-release
    mkdir -p {{ godot_project }}/exports
    {{ godot }} --headless --path {{ godot_project }} --export-release "Web" exports/BulletsOnWheels.html
    cp {{ godot_project }}/exports/BulletsOnWheels.html {{ godot_project }}/exports/index.html

# Serve the exported game locally. Browsers treat localhost as a secure context,
# so plain HTTP works here. Public http:// URLs do not: Godot Web requires a

# secure context, so outside-network demos need HTTPS.
serve-web:
    lsof -ti:8060 | xargs kill 2>/dev/null || true
    python3 -m http.server 8060 --directory {{ godot_project }}/exports &
    sleep 1
    @echo "Server URL: http://localhost:8060/BulletsOnWheels.html"

# IPv6 localhost variant. This is still local-only; public IPv6 over HTTP will

# fail Godot's Secure Context check in browsers.
serve-web-6:
    lsof -ti:8060 | xargs kill 2>/dev/null || true
    python3 -m http.server 8060 --bind ::1 --directory "{{ godot_project }}/exports" &
    sleep 1
    @echo "Server URL: http://[::1]:8060/BulletsOnWheels.html"

# Public outside-network demo URL via Cloudflare Tunnel.
# Install once with: brew install cloudflared

# Cloudflare provides a trusted https:// URL for your local HTTP server.
serve-web-public:
    command -v cloudflared >/dev/null || (echo "cloudflared not found. Install with: brew install cloudflared" && exit 1)
    lsof -ti:8060 | xargs kill 2>/dev/null || true
    python3 -m http.server 8060 --directory {{ godot_project }}/exports &
    sleep 1
    cloudflared tunnel --url http://localhost:8060

# Backwards-compatible alias.
serve-web-demo: serve-web-public

# Stop all web serving started by the recipes above.
stop-web:
    lsof -ti:8060 | xargs kill 2>/dev/null || true
    pkill -f "cloudflared tunnel --url http://localhost:8060" || true
    lsof -ti:20241 | xargs kill 2>/dev/null || true
