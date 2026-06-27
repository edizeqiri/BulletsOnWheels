use bevy::{ecs::event::Trigger, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::classes::LineEdit;
use godot_bevy::prelude::*;

use crate::gamestate::InGameState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(OnEnter(InGameState::DEFEAT), (spawn_ask_for_player_name_system, connect_enter_name_system))
        .add_observer(name_submitted);
}

#[derive(Event, Clone)]
struct NameEnteredEvent {
    name: String
}

#[derive(AssetCollection, Resource)]
pub(crate) struct DefeatAssets {
    #[asset(path = "scenes/defeat/enter_name.tscn")]
    pub enter_name_scene: Handle<GodotResource>,
}

fn spawn_ask_for_player_name_system(
    mut commands: Commands,
    assets: Res<DefeatAssets>,
) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(assets.enter_name_scene.clone()));
    

}

fn connect_enter_name_system(
    text_field_object: Query<&GodotNodeHandle, With<LineEditMarker>>,
    entered_name_signal: GodotSignals<NameEnteredEvent>,
) {
    let Ok(handler) = text_field_object.single() else {
        info!("handler of line edit not found");
        return;
    };
    
    entered_name_signal.connect(
        *handler, 
        LineEditSignals::TEXT_SUBMITTED, 
        None, 
        |args, _node_handle, _ent| {
            let Some(name) = args.get(0)?.try_to::<String>().ok() else {
                error!("Name could not be parsed");
                return None;
            };
        
            Some(NameEnteredEvent { name })
        }
    );
}

fn name_submitted(trigger: On<NameEnteredEvent>) {
    let entered_name = &trigger.event().name;
    info!("entered name {}", entered_name)
}

