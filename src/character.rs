pub mod enemy;
pub mod player;

use crate::weapon::bow::Bow;
use crate::weapon::Weapon;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;

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
    max: u32,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    health: Health,
    weapon: Weapon,
    body: RigidBody,
    sensor: Sensor,
    collider: Collider,
    active_events: ActiveEvents,
    transform: Transform,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    damping: Damping,
}

pub fn create_character(transform: Transform) -> CharacterBundle {
    CharacterBundle {
        health: Health { current: 0, max: 1 },
        weapon: Weapon::Bow(Bow {}),
        body: RigidBody::Dynamic,
        sensor: Sensor,
        collider: collider(),
        transform: transform,
        active_events: ActiveEvents::COLLISION_EVENTS,
        locked_axes: LockedAxes::ROTATION_LOCKED, // Prevent spinning
        gravity_scale: GravityScale(0.0), // Disable gravity
        damping: Damping { linear_damping: 10.0, angular_damping: 10.0 }, // High damping
    }
}

pub fn square_sprite(color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..Default::default()
    }
}

