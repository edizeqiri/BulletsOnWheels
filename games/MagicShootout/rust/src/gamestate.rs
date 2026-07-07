use bevy::app::{App, AppExit, FixedUpdate, Update};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::Event;
use bevy::ecs::observer::On;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::{Message, MessageReader, MessageWriter, Res, State, States};
use godot::classes::Label;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};
use godot_bevy::prelude::SceneTreeRef;
use godot_bevy_macros::GodotNode;

use crate::level_manager::LevelId;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(exit_game);
    //.add_systems(FixedUpdate, log_state);
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) enum AppState {
    LOADING, // asset loading & level transition
    #[default]
    RUNNING, // character movement
    PAUSE,   // menu
    EXIT     // exit
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InGameState {
    PAUSED,
    #[default]
    RUNNING,
    DEFEAT
}

#[derive(Message)]
pub struct CharacterDeathMessage {
    pub source: Entity,
    pub target: Entity
}

#[derive(Event, Debug, Clone)]
pub struct ExitGameEvent;

#[derive(GodotNode, Component, Default)]
#[godot_node(base(Label), class_name(RExitGameLabel))]
pub struct ExitGameLabel;

#[cfg(not(target_arch = "wasm32"))]
fn exit_game(
    _trigger: On<ExitGameEvent>,
    mut exit: MessageWriter<AppExit>,
    mut scene_tree: SceneTreeRef
) {
    info!("Exit game.");

    // bevy exit
    exit.write(AppExit::Success);

    // godot exit
    scene_tree.get().quit();
}

#[cfg(target_arch = "wasm32")]
fn exit_game(
    _trigger: On<ExitGameEvent>,
    handles: Query<&GodotNodeHandle, With<ExitGameLabel>>,
    mut godot: GodotAccess
) {
    let Ok(handle) = handles.single() else {
        return;
    };
    let Some(mut label) = godot.try_get::<Label>(*handle) else {
        return;
    };
    label.set_visible(true);
}

fn log_state(
    appstate: Res<State<AppState>>,
    ingamestate: Res<State<InGameState>>,
    levlestate: Res<State<LevelId>>
) {
    info!("appstate is: {:?}", appstate.get());
    info!("ingamestate is: {:?}", ingamestate.get());
    info!("levlestate is: {:?}", levlestate.get());
}
