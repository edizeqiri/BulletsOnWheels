godot := "godot"
godot_project := "godot"

build:
    cargo build --manifest-path rust/Cargo.toml

build-release:
    cargo build --release --manifest-path rust/Cargo.toml

run: build
    {{godot}} --path {{godot_project}}

run-release: build-release
    {{godot}} --path {{godot_project}}

editor: build
    {{godot}} --path {{godot_project}} --editor

watch:
    cargo watch -x "build --manifest-path rust/Cargo.toml"
