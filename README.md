# BulletsOnWheels

## Tools

Nightly rustfmt is used:

```bash
rustup toolchain install nightly
cargo +nightly fmt
```

or make nightly the default for this project only:

```bash
rustup override set nightly
```


## Architecture

<!-- ARCH:events -->
### Events

![Event Flow](docs/events.svg)
<!-- /ARCH:events -->

<!-- ARCH:messages -->
### Messages

![Message Flow](docs/messages.svg)
<!-- /ARCH:messages -->

<!-- ARCH:states -->
### States

![State Flow](docs/states.svg)
<!-- /ARCH:states -->