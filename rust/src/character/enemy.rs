use bevy::prelude::*;
use godot_bevy::prelude::*;
use rand::{Rng, RngCore};

use crate::character::{CharacterCore, Health, MovementSpeed};
/*
use crate::gamestate::EnemyResource;
use crate::weapon::Weapons;
use crate::world::map::map::Level;
*/

pub(super) fn plugin(app: &mut App) {
    app.add_message::<EnemyDeathMessage>()
        .add_message::<EnemySpawnedMessage>()
        .add_message::<CreateEnemyMessage>()
        .add_systems(Update, check_enemy_zero_health_system)
        .add_systems(Update, handle_enemy_zero_health_system);
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(REnemy2D))]
pub struct EnemyBundle {
    enemy: Enemy,

    #[export_fields(
        current(export_type(f32), default(10.)),
        max(export_type(f32), default(10.))
    )]
    health: Health,

    #[export_fields(value(export_type(f32), default(100.)))]
    speed: MovementSpeed,

    core: CharacterCore,
}

/// This message will be reused for any enemy entity, even bullets. Don't ask
/// why.
#[derive(Message)]
pub struct EnemyDeathMessage {
    pub entity: Entity,
}

#[derive(Message)]
pub struct EnemySpawnedMessage {
    pub entity: Entity,
}

#[derive(Message)]
pub struct CreateEnemyMessage;

fn check_enemy_zero_health_system(
    mut death_message: MessageWriter<EnemyDeathMessage>,
    query: Query<(&Health, Entity), (With<Enemy>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0. {
            death_message.write(EnemyDeathMessage { entity });
        }
    }
}

fn handle_enemy_zero_health_system(
    mut commands: Commands,
    mut enemy_death_messages: MessageReader<EnemyDeathMessage>,
) {
    for message in enemy_death_messages.read() {
        commands.entity(message.entity).despawn();
    }
}
