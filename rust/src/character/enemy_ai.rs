// move to player

use std::default;

use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::{
    character::{
        Aim, Movement, MovementSpeed, enemy::Enemy, enemy_ai::EnemyType::Hunter, player::Player,
    },
    weapon::weapon::ShootMessage,
};
//use crate::weapon::ShootEvent;
//use crate::world::LevelState;
//use crate::world::map::map::Level;
pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GodotTransformConfig::two_way())
        .add_systems(Update, shoot_at_player_system)
        .add_systems(Update, enemy_move_system)
        .add_systems(Update, set_all_fugitive_system);
}

#[enum_delegate::register]
pub trait EnemyAI {
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
pub struct EnemyFugitive;

impl EnemyAI for EnemyFugitive {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        todo!()
    }
}

#[derive(Debug)]
pub struct EnemySeeker;

impl EnemyAI for EnemySeeker {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        todo!()
    }
}

#[derive(Component, Debug)]
struct EnemyHunter;
impl EnemyAI for EnemyHunter {
    fn moving(&self, player: &Transform, enemy: &Transform) -> Vec2 {
        player.translation.xy() - enemy.translation.xy()
    }
}

fn shoot_at_player_system(
    mut shoot_event: MessageWriter<ShootMessage>,
    enemy_query: Query<(Entity, &mut Aim, &Transform, &EnemyType), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    for (enemy, mut aim, enemy_transform, enemy_type) in enemy_query {
        aim.vec = enemy_type.shooting(player, enemy_transform);

        shoot_event.write(ShootMessage { shooter: enemy });
    }
}

fn enemy_move_system(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(&mut Movement, &Transform, &EnemyType), With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut enemy_velocity, enemy_transform, enemy_type) in enemy_query {
            enemy_velocity.vec = enemy_type
                .moving(player_transform, enemy_transform)
                .normalize();
        }
    }
}

fn set_all_fugitive_system(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query {
        commands
            .entity(enemy)
            .insert(EnemyType::Hunter(EnemyHunter));
    }
}
