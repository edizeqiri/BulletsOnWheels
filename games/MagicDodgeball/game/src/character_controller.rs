use avian2d::prelude::*;
use bevy::ecs::query::Has;
use bevy::prelude::*;
use dev_tools::physics_config::PhysicsTuning;

/// This code was copied from:
/// https://github.com/avianphysics/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs#L6
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input,
                gamepad_input,
                update_grounded,
                movement,
                apply_movement_damping
            )
                .chain()
        );
    }
}

#[derive(Message)]
pub enum MovementAction {
    Move(f32),
    Jump
}

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")] // TODO(Clanker): Check if this makes sense
pub struct Grounded;


#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec2::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vec2::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as f32;

    if direction != 0.0 {
        movement_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }
}

fn gamepad_input(mut movement_writer: MessageWriter<MovementAction>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX) {
            movement_writer.write(MovementAction::Move(x));
        }

        if gamepad.just_pressed(GamepadButton::South) {
            movement_writer.write(MovementAction::Jump);
        }
    }
}

fn update_grounded(
    mut commands: Commands,
    tuning: Res<PhysicsTuning>,
    mut query: Query<(Entity, &ShapeHits, &Rotation), With<CharacterController>>,
) {
    for (entity, hits, rotation) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            (rotation * -hit.normal2).angle_to(Vec2::Y).abs() <= tuning.max_slope_angle
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}


fn movement(
    time: Res<Time>,
    tuning: Res<PhysicsTuning>,
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<(&mut LinearVelocity, Has<Grounded>)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs();

    for event in movement_reader.read() {
        for (mut linear_velocity, is_grounded) in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    let actual_acc = tuning
                        .movement_acceleration
                        .min(tuning.max_acceleration);
                    linear_velocity.x += *direction * actual_acc * delta_time;
                },
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = tuning.jump_impulse;
                    }
                },
            }
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(
    time: Res<Time>,
    tuning: Res<PhysicsTuning>,
    mut query: Query<&mut LinearVelocity>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs();

    for mut linear_velocity in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the
        // Y axis
        linear_velocity.x *= 1.0 / (1.0 + tuning.movement_damping * delta_time);
    }
}
