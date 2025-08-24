use crate::{character};
use bevy::app::{App, Startup};
use bevy::prelude::{Camera2d, Commands, Name, Transform};
use crate::character::enemy::enemy_additions;
use crate::character::player::player_additions;
use crate::weapon::bow::Bow;
use crate::weapon::cooldown::WeaponCooldowns;
use crate::weapon::{Weapon, Weapons};
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera)
        .add_systems(Startup, init);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn init(mut commands: Commands) {

    let weapons = Weapons(vec![
        Weapon::Bow(Bow::new(1,10.,2.))
    ]);

    let cooldowns = WeaponCooldowns::new(&weapons);

    commands.spawn((
        Name::new("Player"),
        character::create_character(Transform::from_xyz(100.0, 0.0, 0.0)),
        player_additions(),
        weapons.clone(),
        cooldowns.clone()
    ));
    commands.spawn((
        Name::new("Enemy"),
        character::create_character(Transform::from_xyz(-100.0, 0.0, 0.0)),
        enemy_additions(),
        weapons,
        cooldowns,
    ));
}
