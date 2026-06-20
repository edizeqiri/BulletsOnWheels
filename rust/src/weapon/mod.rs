use bevy::app::App;
use bevy::prelude::*;

pub(crate) mod projectile;
pub(crate) mod weapon;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(weapon::plugin);
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Damage(pub f32);

impl Default for Damage {
    fn default() -> Self {
        Self(1.)
    }
}
