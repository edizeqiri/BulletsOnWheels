use bevy::prelude::*;
use godot::register::info;
use godot_bevy::{plugins::input::MouseButton, prelude::*};

use crate::{
    character::{Aim, player::Player},
    weapon::{projectile::Velocity, weapon::{ShootMessage, Weapon}},
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_mouse_motion_system)
        .add_systems(Update, handle_mouse_system);
}

fn handle_mouse_motion_system(
    mut query: Query<&mut Aim, With<Player>>,
    mut events: MessageReader<MouseMotion>,
) {
    let Ok(mut aim) = query.single_mut() else {
        return;
    };

    for event in events.read() {
        aim.vec = event.position;
    }
}

fn handle_mouse_system(
    mut events: MessageReader<MouseButtonInput>,
    mut shoot_message: MessageWriter<ShootMessage>,
    player_query: Query<(Entity, &Aim, &Weapon), With<Player>>, // todo: which player?
) {
    for event in events.read() {
        if event.pressed {
            match event.button {
                MouseButton::Left => {
                    let Ok((player_entity, aim, weapon)) = player_query.single() else {
                        return;
                    };
                    shoot_message.write(ShootMessage {
                        shooter: player_entity,
                        velocity: Velocity(aim.vec.normalize() * weapon.speed.0)
                    });
                    info!("gaga: {:?}",aim.vec.normalize() * weapon.speed.0)
                    
                },
                _ => {},
            }
        }
    }
}
