// move to player

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::character::enemy::Enemy;
use crate::character::player::Player;
use crate::character::{enemy_collision_groups, Aim};
use crate::weapon::ShootEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot_at_player)
        .add_systems(Update, move_at_player);
}

fn shoot_at_player(
    mut shoot_event: MessageWriter<ShootEvent>,
    enemy_query: Query<(Entity, &mut Aim, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    for (enemy, mut aim, enemy_transform) in enemy_query {
        *aim = Aim {
            vec: player.translation.xy() - enemy_transform.translation.xy()
        };

        shoot_event.write(ShootEvent {
            shooter: enemy,
            collision_groups: enemy_collision_groups()
        });
    }
}

fn move_at_player(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(&mut Velocity, &Transform), With<Enemy>>
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut enemy_velocity, enemy_transform) in enemy_query {
            enemy_velocity.linvel =
                (player_transform.translation.xy() - enemy_transform.translation.xy()) * 0.8;
        }
    }
}
