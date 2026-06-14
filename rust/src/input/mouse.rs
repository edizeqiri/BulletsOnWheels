use bevy::prelude::*;
use godot_bevy::prelude::*;

use crate::character::{Aim, player::Player};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_mouse_motion_system);
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
        info!("Mouse moved by: {:?}", event.position);
    }
}
