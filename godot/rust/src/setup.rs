use bevy::app::App;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::{
    Camera2d, Commands, IntoScheduleConfigs, Name, OnEnter, Res, Startup, Transform,
    resource_exists, *
};

use crate::character::player::{Player, create_player_bundle};
use crate::gamestate::start::{PLAYER_DEFAULTS, StartGameMessage, apply_player_defaults};
use crate::gamestate::{GameState, PlayerResource};
use crate::weapon::Weapons;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PLAYER_DEFAULTS)
        .add_systems(
            OnEnter(GameState::RUNNING),
            init.run_if(resource_exists::<PlayerResource>)
        );

}


fn init(mut commands: Commands, player_resource: Res<PlayerResource>) {
    commands.spawn(create_player_bundle(
        Transform::from_xyz(100.0, 0.0, 0.0),
        Weapons::default(),
        player_resource.max_health,
        Name::from("Player")
    ));
}
