godot := "godot"
godot_project := "godot"

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
    cargo watch -x "build

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

# Serve the exported game. The "Web" preset injects a service worker that sets
# the COOP/COEP headers needed for threads, so a plain HTTP server works after
# the first load. Then open http://localhost:8060/BulletsOnWheels.html
serve-web:
    python3 -m http.server 8060 --directory {{ godot_project }}/exports
