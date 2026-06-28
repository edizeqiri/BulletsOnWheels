use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::character::{CharacterCore, Health, MovementSpeed};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(RPlayer2D))]
pub struct PlayerBundle {
    player: Player,

    #[export_fields(
        current(export_type(f32), default(10.)),
        max(export_type(f32), default(10.))
    )]
    health: Health,

    #[export_fields(value(export_type(f32), default(200.)))]
    speed: MovementSpeed,

    core: CharacterCore,
}

#[derive(Message)]
pub struct PlayerDeathMessage {
    pub entity: Entity
}
