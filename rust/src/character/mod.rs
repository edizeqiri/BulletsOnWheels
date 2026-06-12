pub mod enemy;
mod enemy_ai;
pub mod player;

use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(enemy_ai::plugin)
        .add_plugins(enemy::plugin)
        .add_plugins(player::plugin);
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

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(RCharacter2D))]
pub struct CharacterBundle {
    #[export_fields(current(export_type(u32), default(10)), max(export_type(u32), default(10)))]
    health: Health,
    //weapon: Weapons,
    transform: Transform,
    aim: Aim,
    shooting_state: ShootingState,
}

pub fn create_character(
    transform: Transform,
    //weapons: Weapons,
    max_health: u32,
) -> CharacterBundle {
    CharacterBundle {
        health: Health {
            current: max_health,
            max: max_health,
        },
        //weapon: weapons,
        transform,
        aim: Aim::default(),
        shooting_state: ShootingState::default(),
    }
}
