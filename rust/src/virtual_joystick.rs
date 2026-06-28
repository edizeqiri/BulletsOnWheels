use bevy::prelude::*;
use godot::classes::{Control, Input};
use godot::global::JoyAxis;
use godot_bevy::prelude::*;

use crate::character::Aim;
use crate::gamepad::DEADZONE;
use crate::player::Player;
use crate::weapon_impl::ShootMessage;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, shoot_virtual_joystick);
}

#[derive(Debug, Component)]
struct VirtualJoystickComponent {
    pub timer: Timer
}

impl Default for VirtualJoystickComponent {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Repeating)
        }
    }
}

fn shoot_virtual_joystick(
    mut commands: Commands,
    mut timer: Local<VirtualJoystickComponent>,
    time: Res<Time>,
    mut shoot_message: MessageWriter<ShootMessage>,
    mut godot: GodotAccess,
    mut query: Query<(&mut Aim, Entity), With<Player>>
) {
    let gd_input = godot.singleton::<Input>();

    if gd_input.get_connected_joypads().is_empty() {
        let Ok((mut aim, player)) = query.single_mut() else {
            return;
        };

        timer.timer.tick(time.delta());

        if timer.timer.is_finished() {
            if aim.vec.length() < DEADZONE {
                return;
            }
            info!("aim len: {}", aim.vec);
            shoot_message.write(ShootMessage {
                shooter: player,
                aim: Aim { vec: aim.vec }
            });
        }
    }
}
