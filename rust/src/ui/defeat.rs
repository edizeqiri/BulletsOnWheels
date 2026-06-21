use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::gamestate::{CharacterDeathMessage, GameStateEnum, InGameState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::DEFEAT), ask_for_player_name_system);
}

#[derive(AssetCollection, Resource)]
pub(crate) struct DefeatAssets {
    #[asset(path = "scenes/defeat/enter_name.tscn")]
    pub enter_name_scene: Handle<GodotResource>,
}

fn ask_for_player_name_system(
    mut commands: Commands,
    assets: Res<DefeatAssets>
) {
    commands
        .spawn_empty()                
        .insert(GodotScene::from_handle(assets.enter_name_scene.clone()));
}
