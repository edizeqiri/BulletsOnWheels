pub mod enemy;
mod enemy_ai;
pub mod player;

use bevy::prelude::*;
use godot::classes::{AnimatedSprite2D, CharacterBody2D};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::weapon::Damage;
use crate::weapon::projectile::Projectile;
use crate::weapon::weapon::{Shooter, Weapon};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<CharacterHitMessage>()
        .add_plugins(enemy_ai::plugin)
        .add_plugins(enemy::plugin)
        .add_plugins(player::plugin)
        .add_systems(PhysicsUpdate, apply_character_movement)
        .add_observer(character_bullet_collision_system)
        .add_systems(Update, update_healthbar_animation_system);
}

#[derive(Component, Reflect)]
pub struct Health {
    pub current: f32,
    pub(crate) max: f32,
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
}

/// Movement Sink, godot style
/// move_and_slide is needed so that we do not have teleports by just changing
/// translation in transform
fn apply_character_movement(
    query: Query<(&GodotNodeHandle, &MovementDirection, &MovementSpeed)>,
    mut godot: GodotAccess,
) {
    for (handle, movement, speed) in &query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            continue;
        };
        let velocity = Vector2::new(movement.vec.x, movement.vec.y) * speed.0;
        body.set_velocity(velocity);
        body.move_and_slide();
    }
}

#[derive(Message)]
struct CharacterHitMessage {
    pub source: Entity,
    pub target: Entity,
    pub health: f32,
}

fn character_bullet_collision_system(
    collision: On<CollisionStarted>,
    mut health_query: Query<(&mut Health)>,
    projectile_query: Query<&Damage, With<Projectile>>,
    shooter_query: Query<&Shooter>,
    mut hit_writer: MessageWriter<CharacterHitMessage>,
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
        warn!("target has no health");
        return;
    };

    let Ok(shooter) = shooter_query.get(bullet_entity) else {
        error!("bullet has no shooter");
        return;
    };

    // skip self-hit
    if shooter.0 == target_entity {
        info!("Self hit");
        return;
    }
    info!(
        "Health at {} and receiving damage {}",
        health.current, damage.0
    );
    health.current -= damage.0;
    hit_writer.write(CharacterHitMessage {
        source: shooter.0,
        target: target_entity,
        health: health.current,
    });
}

fn update_healthbar_animation_system(
    mut damage_message: MessageReader<CharacterHitMessage>,
    characters: Query<&GodotNodeHandle, With<Health>>,
    mut godot: GodotAccess,
) {
    for damage in damage_message.read() {
        if let Ok(handle) = characters.get(damage.target) {
            let Some(character_body) = godot.try_get::<CharacterBody2D>(*handle) else {
                return;
            };
            let mut sprite = character_body.get_node_as::<AnimatedSprite2D>("Healthbar");

            if damage.health >= 0. && damage.health <= 4. {
                let number: u32 = damage.health as u32;
                sprite.play_ex().name(&number.to_string()).done();
            }
        }
    }
}
