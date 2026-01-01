mod character;
mod gamepad;
mod physics;
mod projectile;
mod setup;
mod terrain;
mod weapon;

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(logger())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(physics::plugin)
        .add_plugins(gamepad::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        .add_plugins(terrain::plugin)
        .run();
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,rustydefense=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
        fmt_layer: |_| None,
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("sprites.png");
    let tile_size = UVec2::splat(64);

    let layout = TextureAtlasLayout::from_grid(tile_size, 16, 16, Some(UVec2::splat(16)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((Sprite::from_atlas_image(
        texture_handle,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    ),));
}
