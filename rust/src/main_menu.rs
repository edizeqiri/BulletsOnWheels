use std::default;

use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::{
    level_manager::{LevelId, LoadLevelMessage},
    weapon::projectile::Projectile,
};

pub(super) fn plugin(app: &mut App) {
    app;
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
}

#[derive(Component, Default)]
struct ButtonTypeComponent(ButtonType);

#[derive(GodotConvert, Var, Export, Default, Clone)]
#[godot(via = GString)] // provides enum as string
enum ButtonType {
    #[default]
    START,
    SETTINGS,
    EXIT,
}

fn handle_start_button_system(
    collision: On<CollisionStarted>,
    mut commands: Commands,
    button_query: Query<Entity, (With<MenuButton>, With<Shootable>)>,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    let event = collision.event();

    if let Ok(_) = button_query.get(event.entity1) {
        if projectile_query.get(event.entity2).is_ok() {
            commands.trigger(LoadLevelMessage {
                level_id: LevelId::Level1,
            });
        }
    } else if let Ok(_) = projectile_query.get(event.entity2) {
        if button_query.get(event.entity1).is_ok() {
            commands.trigger(LoadLevelMessage {
                level_id: LevelId::Level1,
            });
        }
    };
}
