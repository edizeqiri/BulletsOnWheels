// move to player

use crate::character::Aim;
use crate::character::enemy::Enemy;
use crate::weapon::cooldown::WeaponCooldowns;
use crate::weapon::{ShootEvent, Weapons, shoot_all_weapons};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot_at_player);
}

fn shoot_at_player(
    mut commands: Commands,
    mut shoot_event: EventWriter<ShootEvent>,
    mut shooter_query: Query<(&Weapons, &Aim, &mut WeaponCooldowns, &Transform)>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy in enemy_query {
        shoot_all_weapons(&mut commands, &mut shooter_query, enemy);
    }
}
