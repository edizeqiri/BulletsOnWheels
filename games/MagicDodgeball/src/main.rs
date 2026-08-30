use bevy::asset::UnapprovedPathMode;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use core_engine::character::CharacterCore;

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
        .add_plugins(MD)
        .run();
}

struct MD;

impl Plugin for MD {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell::<WallBundle>(1);
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    player: Player,
    character: CharacterCore
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
