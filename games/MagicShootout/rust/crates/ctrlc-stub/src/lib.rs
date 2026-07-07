//! No-op replacement for the `ctrlc` crate.
//!
//! The real `ctrlc` pulls in `nix` under `cfg(unix)`, which fails to compile for
//! `wasm32-unknown-emscripten` (Godot's web target reports `target_family = "unix"`
//! but its libc lacks `sigwait` / `pthread_sigmask`). Bevy's `bevy_app`
//! unconditionally enables `ctrlc` via its `std` feature, so we swap it for this
//! stub with `[patch.crates-io]`.
//!
//! A terminal Ctrl+C handler has no meaning in a browser, and on desktop the
//! crate runs as a Godot GDExtension where Godot owns the process lifecycle, so
//! a no-op is the correct behavior on every target.

use std::fmt;

/// Mirror of `ctrlc::Error`, kept just rich enough for `bevy_app` to match on
/// `Error::MultipleHandlers` and `Display`-format the catch-all arm.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A handler was already installed.
    MultipleHandlers,
    /// An underlying system error.
    System(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MultipleHandlers => write!(f, "Ctrl-C handler already set"),
            Error::System(e) => write!(f, "system error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// No-op stand-in for `ctrlc::set_handler`. Always succeeds.
pub fn set_handler<F>(_user_handler: F) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    Ok(())
}

/// No-op stand-in for `ctrlc::try_set_handler`. Always succeeds.
pub fn try_set_handler<F>(_user_handler: F) -> Result<(), Error>
where
    F: FnMut() + 'static + Send,
{
    Ok(())
}
