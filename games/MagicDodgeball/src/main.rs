use avian2d::PhysicsPlugins;
use avian2d::prelude::*;
use bevy::asset::UnapprovedPathMode;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use core_engine::character::CharacterCore;

use crate::character_controller::{CharacterControllerBundle, CharacterControllerPlugin};
use crate::debug::fps::FPS;
mod character_controller;
mod debug;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                })
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(core_engine::plugin)
        .add_plugins(FPS)
        .add_plugins(MD)
        .run();
}

struct MD;

impl Plugin for MD {
    fn build(&self, app: &mut App) {
        app
            // Libs
            .add_plugins(PhysicsPlugins::default() )
           // .add_plugins(PhysicsDebugPlugin)
            .add_plugins(bevy_framepace::FramepacePlugin)
            // Custom
            .add_plugins(CharacterControllerPlugin)
            // Resources
            .insert_resource(LevelSelection::index(0))
            .insert_resource(Gravity(Vec2::NEG_Y * 250.0))
            // LDTK
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell::<WallBundle>(1)
            // Systems
            .add_systems(Startup, setup)
        //    .add_systems(Update, simple_controlls)
        ;
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    collider: Collider,
    rigid_body: RigidBody,
    collision_margin: CollisionMargin
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Default::default(),
            collider: Collider::rectangle(8., 8.),
            //            collider: Collider::capsule(Vec2::new(-3., 0.), Vec2::new(3., 0.), 4.),
            rigid_body: RigidBody::Static,
            collision_margin: CollisionMargin(0.3)
        }
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    player: Player,
    character: CharacterCore,
    character_controller: CharacterControllerBundle,
    friction: Friction
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: Default::default(),
            player: Default::default(),
            character: Default::default(),
            character_controller: CharacterControllerBundle::new(Collider::rectangle(6., 6.)),
            friction: Friction::ZERO
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 296.0
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(304.0 / 2.0, 296.0 / 2.0, 0.0)
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk").into(),
        ..Default::default()
    });
}

// map [keys][function]
const PLAYER_SPEED: f32 = 1.;
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];

// TODO: Add physics to jump and maybe refactor jump out as an event or smth
fn simple_controlls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<Player>>
) {
    if keyboard.any_pressed(LEFT) {
        player.translation.x -= PLAYER_SPEED;
    }
    if keyboard.any_pressed(RIGHT) {
        player.translation.x += PLAYER_SPEED;
    }
    if keyboard.any_pressed(UP) {
        player.translation.y += PLAYER_SPEED;
    }
    if keyboard.any_pressed(DOWN) {
        player.translation.y -= PLAYER_SPEED;
    }
}
