use crate::character::player::create_player_bundle;
use crate::gamestate::start::{apply_player_defaults, StartGameMessage, PLAYER_DEFAULTS};
use crate::gamestate::{GameState, PlayerResource};
use crate::weapon::Weapons;
use bevy::app::App;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::prelude::{
    resource_exists, Camera2d, Commands, IntoScheduleConfigs, Name, OnEnter, Res, Startup,
    Transform,
};
use bevy_lunex::prelude::{Anchor, SystemCursorIcon};
use bevy_lunex::{
    OnHoverSetCursor, Rl, UiColor, UiFetchFromCamera, UiLayout, UiLayoutRoot, UiSourceCamera,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PLAYER_DEFAULTS)
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(OnEnter(GameState::START), apply_player_defaults)
        .add_systems(
            OnEnter(GameState::RUNNING),
            init.run_if(resource_exists::<PlayerResource>),
        );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        // This camera will become the source for all UI paired to index 0.
        Camera2d,
        UiSourceCamera::<0>,
        // Ui nodes start at 0 and move + on the Z axis with each depth layer.
        // This will ensure you will see up to 1000 nested children.
        Transform::from_translation(Vec3::Z * 1000.0),
        // Explained in # Chapters/Debug-Tooling section of the book
        RenderLayers::from_layers(&[0, 1]),
    ));
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create UI
    commands
        .spawn((
            // Initialize the UI root for 2D
            UiLayoutRoot::new_2d(),
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Start"),
                UiLayout::window()
                    .anchor(Anchor::CENTER) // Put the origin at the center
                    .pos(Rl((50.0, 50.0))) // Set the position to 50%
                    .size((200.0, 200.0)) // Set the size to [200.0, 50.0]
                    .pack(),
                //UiColor::from(Color::srgb(1.0, 0.0, 0.0)),
                Sprite::from_image(asset_server.load("start_button.png")),
            ))
            .observe(
                |trigger: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut exit: MessageWriter<StartGameMessage>| {
                    exit.write(StartGameMessage);
                    commands.entity(trigger.entity).despawn();
                },
            );
        });
}

fn init(mut commands: Commands, player_resource: Res<PlayerResource>) {
    commands.spawn(create_player_bundle(
        Transform::from_xyz(100.0, 0.0, 0.0),
        Weapons::default(),
        player_resource.max_health,
        Name::from("Player"),
    ));
}
