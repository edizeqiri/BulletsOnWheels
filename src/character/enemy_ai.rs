// move to player

use crate::character::enemy::Enemy;
use crate::character::enemy_ai::EnemyType::Hunter;
use crate::character::player::Player;
use crate::character::{Aim, enemy_collision_groups};
use crate::weapon::ShootEvent;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot_at_player)
        .add_systems(Update, move_at_player);
}

#[enum_delegate::register]
trait EnemyAI {
    fn shooting(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        player.translation.xy() - enemy.translation.xy()
    }
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2;
}

#[enum_delegate::implement(EnemyAI)]
#[derive(Component, Debug)]
pub enum EnemyType {
    // Default, want to hunt the player
    Hunter(EnemyHunter),
    // Wants to run away from player
    Fugitive(EnemyFugitive),
    // Wants to go to the exit as soon as possible
    Seeker(EnemySeeker),
}
impl Default for EnemyType {
    fn default() -> Self {
        Hunter(EnemyHunter)
    }
}
#[derive(Debug)]
struct EnemyFugitive;

impl EnemyAI for EnemyFugitive {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        todo!()
    }
}

#[derive(Debug)]
struct EnemySeeker;

impl EnemyAI for EnemySeeker {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        todo!()
    }
}

#[derive(Component, Debug)]
struct EnemyHunter;
impl EnemyAI for EnemyHunter {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        (player.translation.xy() - enemy.translation.xy()) * 0.8
    }
}

fn shoot_at_player(
    mut shoot_event: MessageWriter<ShootEvent>,
    enemy_query: Query<(Entity, &mut Aim, &Transform, &EnemyType), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    for (enemy, mut aim, enemy_transform, enemy_type) in enemy_query {
        aim.vec = enemy_type.shooting(player, enemy_transform);

        shoot_event.write(ShootEvent {
            shooter: enemy,
            collision_groups: enemy_collision_groups(),
        });
    }
}

fn move_at_player(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(&mut Velocity, &Transform, &EnemyType), With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut enemy_velocity, enemy_transform, enemy_type) in enemy_query {
            enemy_velocity.linvel = enemy_type.moving(player_transform, enemy_transform);
        }
    }
}
