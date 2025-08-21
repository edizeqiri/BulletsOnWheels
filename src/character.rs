pub mod enemy;
pub mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::character::player::{create_player, PlayerBundle};
const PLAYER_GROUP: Group = Group::GROUP_1;
const ENEMY_GROUP: Group = Group::GROUP_2;


fn collider() -> Collider {
    Collider::ball(20.0)

}
#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

pub fn get_random_player() -> PlayerBundle {
    create_player()
}

fn square_sprite(color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..Default::default()
    }
}

