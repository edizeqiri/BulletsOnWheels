use crate::character::player::create_player_bundle;
use crate::gamestate::start::{apply_player_defaults, PLAYER_DEFAULTS};
use crate::gamestate::{GameState, PlayerResource};
use crate::weapon::Weapons;
use bevy::app::App;
use bevy::prelude::{
    resource_exists, Camera2d, Commands, IntoScheduleConfigs, Name, OnEnter, Res, Transform,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PLAYER_DEFAULTS)
        .add_systems(OnEnter(GameState::START), apply_player_defaults)
        .add_systems(
            OnEnter(GameState::RUNNING),
            (setup_camera, init.run_if(resource_exists::<PlayerResource>)),
        );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn init(mut commands: Commands, player_resource: Res<PlayerResource>) {
    commands.spawn(create_player_bundle(
        Transform::from_xyz(100.0, 0.0, 0.0),
        Weapons::default(),
        player_resource.max_health,
        Name::from("Player"),
    ));
}
