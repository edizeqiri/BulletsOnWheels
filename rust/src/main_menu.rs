use bevy::prelude::*;
use godot::classes::CharacterBody2D;
use godot::prelude::*;
use godot_bevy::prelude::*;
use rand::Rng;

use crate::character::{MovementDirection, MovementSpeed};
use crate::gamestate::InGameState;
use crate::level_manager::{LevelId, LoadLevelMessage};
use crate::projectile::Projectile;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_button_system)
        .add_systems(FixedUpdate, random_walk_system)
        .add_systems(PhysicsUpdate, wall_bounce_system);
}

#[derive(Component, Default)]
struct MenuButton;

#[derive(Component)]
struct WalkTimer(Timer);

impl Default for WalkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

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
    walk_timer: WalkTimer,

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

/// CharacterBody2D vs StaticBody2D does not emit collision signals, so
/// `CollisionStarted` never fires for buttons hitting walls.
fn wall_bounce_system(
    mut query: Query<(&GodotNodeHandle, &mut MovementDirection), With<MenuButton>>,
    mut godot: GodotAccess
) {
    for (handle, mut dir) in &mut query {
        let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
            continue;
        };

        let mut normal = Vector2::ZERO;
        for i in 0..body.get_slide_collision_count() {
            if let Some(collision) = body.get_slide_collision(i) {
                normal += collision.get_normal();
            }
        }

        if normal == Vector2::ZERO {
            continue;
        }

        let n = normal.normalized();
        dir.vec = dir.vec.reflect(Vec2::new(n.x, n.y));
    }
}

fn random_walk_system(
    mut button_query: Query<
        (&mut MovementDirection, &mut MovementSpeed, &mut WalkTimer),
        With<MenuButton>
    >,
    time: Res<Time>
) {
    let mut rng = rand::rng();

    for (mut movement_dir, mut speed, mut timer) in &mut button_query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            movement_dir.vec = Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0))
                .normalize_or_zero();
            speed.0 = rng.random_range(40.0..60.0);
        }
    }
}
