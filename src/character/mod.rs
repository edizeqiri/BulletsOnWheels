pub mod enemy;
mod enemy_ai;
pub mod player;

use crate::weapon::Weapons;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(enemy_ai::plugin);
}

pub fn player_collision_groups() -> CollisionGroups {
    CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP)
}

pub fn enemy_collision_groups() -> CollisionGroups {
    CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP)
}

fn collider() -> Collider {
    Collider::ball(10.0)
}

#[derive(Component)]
pub struct Health {
    pub current: u32,
    //pub(crate) max: u32,
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

#[derive(Bundle)]
pub struct CharacterBundle {
    health: Health,
    weapon: Weapons,
    velocity: Velocity,
    body: RigidBody,
    //sensor: Sensor,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    damping: Damping,
    aim: Aim,
    shooting_state: ShootingState,
}


pub fn create_character(transform: Transform, weapons: Weapons) -> CharacterBundle {
    CharacterBundle {
        health: Health { current: 0 },
        weapon: weapons,
        velocity: Velocity::linear(Vec2::new(0., 0.)),
        body: RigidBody::Dynamic,
        //sensor: Sensor,
        collider: collider(),
        transform,
        active_events: ActiveEvents::COLLISION_EVENTS,
        locked_axes: LockedAxes::ROTATION_LOCKED, // Prevent spinning
        gravity_scale: GravityScale(0.0),         // Disable gravity
        damping: Damping {
            linear_damping: 10.0,
            angular_damping: 10.0,
        }, // High damping
        aim: Aim::default(),
        shooting_state: ShootingState::default(),
    }
}

pub fn square_sprite(color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::new(20.0, 20.0)),
        ..Default::default()
    }
}
