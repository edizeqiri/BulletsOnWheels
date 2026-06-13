pub mod enemy;
mod enemy_ai;
pub mod player;

use bevy::prelude::*;
use godot::classes::CharacterBody2D;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(enemy_ai::plugin)
        .add_plugins(enemy::plugin)
        .add_plugins(player::plugin)
        .add_systems(PhysicsUpdate, apply_movement);
}

#[derive(Component, Reflect, Default)]
pub struct Health {
    pub current: u32,
    pub(crate) max: u32,
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
pub struct Movement {
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
    //weapon: Weapons,
    aim: Aim,
    movement: Movement,
    speed: MovementSpeed,
    shooting_state: ShootingState,
}

fn apply_movement(
    query: Query<(&GodotNodeHandle, &Movement, &MovementSpeed)>,
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
