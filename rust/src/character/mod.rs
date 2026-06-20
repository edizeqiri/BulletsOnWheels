pub mod enemy;
mod enemy_ai;
pub mod player;

use bevy::prelude::*;
use godot::classes::CharacterBody2D;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::weapon::Damage;
use crate::weapon::projectile::Projectile;
use crate::weapon::weapon::{Shooter, Weapon};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(enemy_ai::plugin)
        .add_plugins(enemy::plugin)
        .add_plugins(player::plugin)
        .add_systems(PhysicsUpdate, apply_character_movement)
        .add_observer(character_bullet_collision_system);
}

#[derive(Component, Reflect, Default)]
pub struct Health {
    pub current: f32,
    pub(crate) max: f32,
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
/// move_and_slide is needed so that we do not have teleports by just changing translation in transform
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

fn character_bullet_collision_system(
    collision: On<CollisionStarted>,
    mut health_query: Query<(&mut Health)>,
    projectile_query: Query<&Damage, With<Projectile>>,
    shooter_query: Query<&Shooter>,
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
}
