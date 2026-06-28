use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;
use rand::Rng;

use crate::character::{MovementDirection, MovementSpeed};
use crate::gamestate::InGameState;
use crate::level_manager::{LevelId, LoadLevelMessage};
use crate::projectile::Projectile;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_button_system)
        .add_systems(FixedUpdate, random_walk_system);
}

#[derive(Component, Default)]
struct MenuButton;

#[derive(Component, Default)]
struct Shootable;

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(RShootableButton2d))]
struct ShootableButtonBundle {
    #[export_fields(value(export_type(ButtonType), default(ButtonType::START)))]
    button_type: ButtonTypeComponent,

    menu_button: MenuButton,
    shootable: Shootable,
    movement_direction: MovementDirection,

    #[export_fields(value(export_type(f32), default(50.)))]
    movement_speed: MovementSpeed
}

#[derive(Component, Default)]
struct ButtonTypeComponent(ButtonType);

#[derive(GodotConvert, Var, Export, Default, Clone, Debug)]
#[godot(via = GString)] // provides enum as string
pub enum ButtonType {
    #[default]
    START,
    SETTINGS,
    EXIT
}

fn handle_button_system(
    collision: On<CollisionStarted>,
    mut commands: Commands,
    button_query: Query<&ButtonTypeComponent, (With<MenuButton>, With<Shootable>)>,
    projectile_query: Query<Entity, With<Projectile>>
) {
    let event = collision.event();

    let button_type = if projectile_query.get(event.entity2).is_ok() {
        button_query.get(event.entity1).ok()
    } else if projectile_query.get(event.entity1).is_ok() {
        button_query.get(event.entity2).ok()
    } else {
        None
    };

    let Some(button_type) = button_type else {
        return;
    };

    info!("Button type hit: {:?}", button_type.0);
    match &button_type.0 {
        ButtonType::START => {
            commands.trigger(LoadLevelMessage {
                level_id: LevelId::Level1
            });
        },
        ButtonType::SETTINGS => {
            // TODO: open settings
        },
        ButtonType::EXIT => {
            info!("send exit game message");
            commands.set_state(InGameState::DEFEAT);
        }
    }
}
// fn check_collision(
// collisions: Collisions,
// menu_button_query: Query<Entity, With<MenuButton>>,
// wall_query: Query<Entity, With<StaticBody2DMarker>>
// ) {
// for
// if collisions.contains(a, b)
// }
fn random_walk_system(button_query: Query<&mut MovementDirection, With<MenuButton>>) {
    let mut rng = rand::rng();

    for mut movement_dir in button_query {
        let rng_vec = Vec2::new(
            rng.random_range(-100. ..100.),
            rng.random_range(-100. ..100.)
        )
        .normalize_or_zero();

        movement_dir.vec = rng_vec;
    }
}
