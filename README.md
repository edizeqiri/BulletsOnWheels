# BulletsOnWheels

Monorepo for the BulletsOnWheels game engine. 

## Core

Rust(Bevy): Bevy used as the main driver with own core and own domain ECS

## Games

- [MagicShootout](games/MagicShootout/README.md)

## Example Game with 
### Menu
![](games/MagicShootout/docs/mainmenu.png)
### Level 1
![](games/MagicShootout/docs/level1.png)

## Tools

### Rust
Nightly rustfmt is used:

```bash
rustup toolchain install nightly
cargo +nightly fmt
```

or make nightly the default for this project only:

```bash
rustup override set nightly
```
## Vendors

The Rust integration can use the godot_bevy plugin to use godot as the frontend with Bevy in the backend.

## Architecture

<!-- ARCH:events -->
## Events

![Event Flow](docs/events.svg)
<!-- /ARCH:events -->

## Architecture

<!-- ARCH:messages -->
## Messages

![Message Flow](docs/messages.svg)
<!-- /ARCH:messages -->

## Architecture

<!-- ARCH:states -->
## States

![State Flow](docs/states.svg)
<!-- /ARCH:states -->
