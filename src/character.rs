pub mod enemy;
pub mod player;

use bevy::prelude::*;
use crate::character::player::{create_player, PlayerBundle};

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

