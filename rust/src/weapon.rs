use bevy::app::App;
use bevy::prelude::*;
use crate::weapon_impl;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(weapon_impl::plugin);
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Damage(pub f32);

impl Default for Damage {
    fn default() -> Self {
        Self(1.)
    }
}
