use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::gamestate::{ExitGameMessage, InGameState};
use crate::weapon::projectile::Projectile;
use crate::world::level_manager::{LevelId, LoadLevelMessage};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_button_system);
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
    shootable: Shootable
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
