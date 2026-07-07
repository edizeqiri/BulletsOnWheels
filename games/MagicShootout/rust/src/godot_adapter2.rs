use bevy::{app::App, ecs::{component::Component, system::Query}, log::info};
use godot::classes::{Label, RichTextLabel};
use godot_bevy::{interop::{BaseButtonSignals, GodotNodeHandle}, plugins::signals::GodotSignals};
use godot_bevy_macros::GodotNode;
use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState}};
use godot_bevy::prelude::*;

use crate::{gamestate::{AppState, InGameState}, level_manager::{CurrentLevel, LevelId::{self, Level1}}, score::{DeathHighscoreScene, LiveScore, NameEnteredEvent, ScoreBoard, SpawnLeaderBoardEvent}, world::RestartGameEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::RUNNING).load_collection::<WorldAssets>())
        .add_loading_state(
            LoadingState::new(AppState::RUNNING).load_collection::<ScoreBoardAssets>()
        )
        .add_plugins(GodotSignalsPlugin::<NameEnteredEvent>::default())
        .add_systems(
            Update,
            connect_restart_buttons.run_if(
                in_state(InGameState::PAUSED)
                    .or_else(in_state(InGameState::DEFEAT).and_then(in_state(Level1)))
            )
        )
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            spawn_death_scene_on_player_death.run_if(in_state(LevelId::MainMenu))
        )
        .add_systems(
            Update,
            track_death_scene
                .run_if(in_state(InGameState::DEFEAT))
                .run_if(in_state(LevelId::MainMenu))
        )
        .add_systems(
            Update,
            update_score_board
                .run_if(in_state(InGameState::DEFEAT))
                .run_if(in_state(LevelId::Level1))
        )
        .add_systems(
            Update,
            update_score_label
                .run_if(in_state(LevelId::Level1))
                .run_if(in_state(InGameState::RUNNING))
        )
        .add_systems(
            OnEnter(InGameState::DEFEAT),
            spawn_ask_for_player_name_system.run_if(in_state(LevelId::Level1))
        )
        .add_systems(
            Update,
            connect_enter_name_system.run_if(in_state(LevelId::Level1))
        )
        .add_observer(spawn_score_board)
    ;
}

#[derive(Component, Default, GodotNode)]
#[godot_node(base(Button), class_name(RRestartButton))]
pub struct RestartButton {
    pub is_connected: bool
}

fn connect_restart_buttons(
    mut restart_buttons: Query<(&GodotNodeHandle, &mut RestartButton)>,
    signal: GodotSignals<RestartGameEvent>
) {
    for (restart_handle, mut restart_button) in &mut restart_buttons {
        if restart_button.is_connected {
            continue;
        }

        signal.connect(
            *restart_handle,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| {
                info!("Restart button pressed");
                Some(RestartGameEvent)
            }
        );

        restart_button.is_connected = true;
    }
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "scenes/defeat/player_death_restart.tscn")]
    pub death_scene_restart: Handle<GodotResource>,

    #[asset(path = "scenes/defeat/player_death_highscore.tscn")]
    pub player_death_highscore_scene: Handle<GodotResource>,
    pub is_connected: bool
}

#[derive(Component)]
struct DeathTimer(Timer);

fn spawn_death_scene_on_player_death(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    assets: Option<Res<WorldAssets>>
) {
    let Some(ref assets) = assets else {
        info!("player death asset not loaded yet");
        return;
    };

    let Some(level) = current_level.entity else {
        info!("No level id");
        return;
    };

    let scene = commands
        .spawn((
            GodotScene::from_handle(assets.death_scene_restart.clone()),
            DeathTimer(Timer::from_seconds(3., TimerMode::Once))
        ))
        .id();

    commands.entity(level).add_child(scene);
}

fn track_death_scene(
    mut commands: Commands,
    time_query: Query<(&mut DeathTimer, Entity)>,
    time: Res<Time>
) {
    for (mut times, entity) in time_query {
        if times.0.is_finished() {
            commands.entity(entity).queue_silenced(|e: EntityWorldMut| {
                e.despawn();
            });
            commands.trigger(RestartGameEvent);
        } else {
            times.0.tick(time.delta());
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ScoreBoardAssets {
    #[asset(path = "scenes/defeat/score_board.tscn")]
    pub score_board_scene: Handle<GodotResource>,

    score_board_is_loaded: bool
}

fn spawn_score_board(
    _trigger: On<SpawnLeaderBoardEvent>,
    mut commands: Commands,
    mut assets: ResMut<ScoreBoardAssets>
) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(assets.score_board_scene.clone()))
        .insert(DespawnOnEnter(InGameState::RUNNING));
    assets.score_board_is_loaded = false;
}

fn update_score_board(
    score_board: Res<ScoreBoard>,
    mut scene_tree: SceneTreeRef,
    mut assets: ResMut<ScoreBoardAssets>
) {
    if assets.score_board_is_loaded {
        return;
    }

    let content = prepare_leader_board_content(score_board);

    assets.score_board_is_loaded = update_label(&mut scene_tree, "/root/ScoreBoard/Top5", content);
}


fn update_label(scene_tree: &mut SceneTreeRef, label_path: &str, content: String) -> bool {
    let Some(root) = scene_tree.get().get_root() else {
        return false;
    };

    let Some(mut label) = root.try_get_node_as::<RichTextLabel>(label_path) else {
        return false;
    };

    label.set_use_bbcode(true);
    label.set_text(&content);
    return true;
}


fn prepare_leader_board_content(score_board: Res<ScoreBoard>) -> String {
    let mut text = String::from("[table=3]");

    // Header
    text.push_str("[cell][left][b]Rank[/b][/left][/cell]");
    text.push_str("[cell][left][b]Name[/b][/left][/cell]");
    text.push_str("[cell][right][b]Score[/b][/right][/cell]");

    // Rows
    for (i, entry) in score_board.entries.iter().take(5).enumerate() {
        text.push_str(&format!(
            "[cell][left]{}.[/left][/cell]\
             [cell][left]{}[/left][/cell]\
             [cell][right]{}[/right][/cell]",
            i + 1,
            entry.name,
            entry.score
        ));
    }

    text.push_str("[/table]");
    text
}

fn connect_enter_name_system(
    enter_name_field: Query<&GodotNodeHandle, With<LineEditMarker>>,
    entered_name_signal: GodotSignals<NameEnteredEvent>,
    world_assets: Option<ResMut<WorldAssets>>
) {
    let Some(mut assets) = world_assets else {
        return;
    };

    if assets.is_connected {
        return;
    }
    let Ok(enter_name_handler) = enter_name_field.single() else {
        return;
    };

    entered_name_signal.connect(
        *enter_name_handler,
        LineEditSignals::TEXT_SUBMITTED,
        None,
        |args, _node_handle, _ent| {
            let Some(name) = args.get(0)?.try_to::<String>().ok() else {
                error!("Name could not be found or parsed");
                return None;
            };

            Some(NameEnteredEvent { name })
        }
    );
    info!("enter name signal connected");
    assets.is_connected = true;
}

fn spawn_ask_for_player_name_system(mut commands: Commands, mut assets: ResMut<WorldAssets>) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(
            assets.player_death_highscore_scene.clone()
        ))
        .insert(DeathHighscoreScene);
    assets.is_connected = false;
}

fn update_score_label(
    score_query: Query<&LiveScore, Changed<LiveScore>>,
    current_level: Res<CurrentLevel>,
    mut scene_tree: SceneTreeRef
) {
    let level_id = current_level.level_id;

    let Ok(score) = score_query.single() else {
        return;
    };

    let score_label_path = format!("{}/ScoreLabel", level_id.root_node_path());

    let Some(root) = scene_tree.get().get_root() else {
        warn!("no root");
        return;
    };

    let Some(mut score_label) = root.try_get_node_as::<Label>(&score_label_path) else {
        warn!("Could not find Label at {}", score_label_path);
        return;
    };

    score_label.set_text(&format!(
        "Score: {}\nHigh Score: {}",
        score.count, score.highscore,
    ));
}