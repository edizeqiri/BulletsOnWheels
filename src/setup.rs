use crate::character::player::create_player_bundle;
use crate::weapon::Weapons;
use crate::weapon::cooldown::WeaponCooldowns;
use crate::character::enemy::create_enemy_bundle;
use crate::character::player::create_player_bundle;
use crate::weapon::bow::Bow;
use crate::weapon::gun::Gun;
use crate::weapon::{Weapon, Weapons};
use bevy::app::{App, Startup};
use bevy::prelude::{Camera2d, Commands, Transform};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera)
        .add_systems(Startup, init);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn init(mut commands: Commands) {

    commands.spawn(create_player_bundle(
        Transform::from_xyz(100.0, 0.0, 0.0),
        Weapons::default(),
        1000,
        "Player".to_string(),
    ));
}
