use bevy::prelude::*;

use crate::gamestate::{CharacterDeathMessage, InGameState};
use crate::player::Player;
use crate::projectile::Projectile;
use crate::weapon::Damage;
use crate::weapon::{Shooter, Weapon};
use crate::{enemy, enemy_ai};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<CharacterHitMessage>()
        .add_message::<CharacterDeathMessage>()
        .add_plugins(enemy_ai::plugin)
        .add_plugins(enemy::plugin)
        .add_observer(character_bullet_collision_system.run_if(in_state(InGameState::RUNNING)))
        .add_systems(
            Update,
            handle_character_zero_health_system.run_if(in_state(InGameState::RUNNING)),
        );
}

#[derive(Component, Reflect)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 4.,
            max: 4.,
        }
    }
}

#[derive(Component, Default)]
pub struct ShootingState {
    pub(crate) is_shooting: bool,
}

#[derive(Component, Copy, Clone)]
pub struct Aim {
    pub vec: Vec2,
}

impl Default for Aim {
    fn default() -> Self {
        Self {
            vec: Vec2::new(0., 0.),
        }
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct MovementDirection {
    pub vec: Vec2,
}

#[derive(Component, Reflect, Copy, Clone)]
pub struct MovementSpeed(pub f32);

impl Default for MovementSpeed {
    fn default() -> Self {
        Self(200.)
    }
}

#[derive(Bundle, Default)]
pub struct CharacterCore {
    weapon: Weapon,
    aim: Aim,
    movement: MovementDirection,
    shooting_state: ShootingState,
    player_name: PlayerName,
}

#[derive(Component, Default)]
pub struct PlayerName(String);

#[derive(Message)]
pub struct CharacterHitMessage {
    pub target: Entity,
    pub health: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Message, Event)]
pub struct CollisionStartedDomain {
    pub entity1: Entity,
    pub entity2: Entity,
}

fn character_bullet_collision_system(
    collision: On<CollisionStartedDomain>,
    mut health_query: Query<&mut Health>,
    projectile_query: Query<&Damage, With<Projectile>>,
    shooter_query: Query<&Shooter>,
    mut hit_writer: MessageWriter<CharacterHitMessage>,
    mut death_message_writer: MessageWriter<CharacterDeathMessage>,
) {
    let event = collision.event();

    let (damage, target_entity, bullet_entity) =
        if let Ok(damage) = projectile_query.get(event.entity2) {
            (damage, event.entity1, event.entity2)
        } else if let Ok(damage) = projectile_query.get(event.entity1) {
            (damage, event.entity2, event.entity1)
        } else {
            return;
        };

    let Ok(mut health) = health_query.get_mut(target_entity) else {
        debug!("target has no health");
        return;
    };

    let Ok(shooter) = shooter_query.get(bullet_entity) else {
        error!("bullet has no shooter");
        return;
    };

    // skip self-hit
    if shooter.0 == target_entity {
        debug!("Self hit");
        return;
    }
    debug!(
        "Health at {} and receiving damage {}",
        health.current, damage.0
    );

    if health.current <= 0. {
        debug!("Enemy already dead");
        return;
    }

    health.current -= damage.0;
    hit_writer.write(CharacterHitMessage {
        target: target_entity,
        health: health.current,
    });

    let character_is_dead = health.current <= 0.;
    if character_is_dead {
        death_message_writer.write(CharacterDeathMessage {
            source: shooter.0,
            target: target_entity,
        });
    }
}

fn handle_character_zero_health_system(
    mut commands: Commands,
    mut character_death_messages: MessageReader<CharacterDeathMessage>,
    player_query: Query<Entity, With<Player>>,
) {
    for message in character_death_messages.read() {
        let Ok(player) = player_query.single() else {
            return;
        };
        if message.target == player {
            info!("Player dead");
            commands.set_state(InGameState::DEFEAT);
        }

        commands
            .entity(message.target)
            .queue_silenced(|e: EntityWorldMut| {
                e.despawn();
            });
    }
}
