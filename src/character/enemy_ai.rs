// move to player

use crate::character::enemy::Enemy;
use crate::character::player::Player;
use crate::character::{Aim, enemy_collision_groups};
use crate::weapon::ShootEvent;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (shoot_at_player, move_at_player));
}

fn shoot_at_player(
    mut shoot_event: EventWriter<ShootEvent>,
    enemy_query: Query<(Entity, &mut Aim), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    for (enemy, mut aim) in enemy_query {
        *aim = Aim {
            vec: player.translation.xy().normalize(),
        };
        shoot_event.write(ShootEvent {
            shooter: enemy,
            collision_groups: enemy_collision_groups(),
        });
    }
}

fn move_at_player(
    player_query: Query<&Velocity, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&mut Velocity, With<Enemy>>,
) {
    if let Ok(velocity) = player_query.single() {
        for mut enemy_velocity in enemy_query {
            enemy_velocity.linvel = velocity.linvel;
        }
    }
}
