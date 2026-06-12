godot := "godot"
godot_project := "godot"

build:
    cargo build
build-release:
    cargo build --release

run: build
    {{godot}} --path {{godot_project}}

run-release: build-release
    {{godot}} --path {{godot_project}}

editor: build
    {{godot}} --path {{godot_project}} --editor

watch:
    cargo watch -x "build
