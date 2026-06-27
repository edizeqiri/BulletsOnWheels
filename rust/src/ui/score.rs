use std::fs::{self, File};
use::std::io::Write;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::prelude::*;
use godot::classes::Label;
use godot_bevy::prelude::*;

use crate::character::player::{self, Player};
use crate::gamestate::{AppState, CharacterDeathMessage, ExitGameMessage};
use crate::world::NameEnteredEvent;
use crate::world::level_manager::{CurrentLevel, Score};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(load_score_board())
        .add_loading_state(
            LoadingState::new(AppState::RUNNING)
                .load_collection::<ScoreBoardAssets>(),
        )
        .add_systems(Update, (score_tracker, update_score_label))
        .add_observer(save_high_score)
        .add_observer(spawn_score_board);
}

use std::path::Path;

const SCORE_BOARD_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/score_board.csv");

#[derive(AssetCollection, Resource)]
pub struct ScoreBoardAssets {
    #[asset(path = "scenes/defeat/score_board.tscn")]
    pub score_board_scene: Handle<GodotResource>,
}

#[derive(Resource)]
struct ScoreBoard {
    entry: Vec<ScoreBoardEntry>,
    high_score: u32 // technically seen could be taken from entry directly -> but this avoids all the overhead to get it
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self {
            entry: vec![
                ScoreBoardEntry {
                    name: "test".to_string(),
                    score: 0,
                },
            ],
            high_score: 0
        }
    }
}

#[derive(Event)]
pub struct SpawnLeaderBoardEvent;

struct ScoreBoardEntry {
    name: String,
    score: u32
}

fn score_tracker(
    mut death_message_reader: MessageReader<CharacterDeathMessage>,
    player_query: Query<Entity, With<Player>>,
    mut score_query: Query<&mut Score>,
    mut score_board: ResMut<ScoreBoard>
) {
    for message in death_message_reader.read() {
        let Ok(player) = player_query.single() else {
            continue;
        };
        if message.source != player {
            continue;
        };

        let Ok(mut score) = score_query.single_mut() else {
            error!("component score not existent.");
            return;
        };
        
        score.count += 1;
        score_board.high_score = update_high_score(score.count, score_board.high_score);
    }
}

// todo: this function shall be "level state" dependent
fn update_score_label(
    score_query: Query<&Score, Changed<Score>>,
    current_level: Res<CurrentLevel>,
    mut scene_tree: SceneTreeRef,
    score_board: Res<ScoreBoard>
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
        score.count,
        score_board.high_score,
    ));
}

fn save_high_score(
    trigger: On<NameEnteredEvent>,
    score_query: Query<&Score>,
    mut score_board: ResMut<ScoreBoard>,
    mut commands: Commands,
) {
    let Ok(score) = score_query.single() else {
        info!("No score found => No high score can be saved.");
        return;
    };
    let entered_name = &trigger.event().name;

    // no need for high score update, as it is constantly updated
    score_board.entry.push(ScoreBoardEntry {
       name: entered_name.clone(),
       score: score.count
    });
    
    save_score_board(entered_name, score.count);

    commands.trigger(SpawnLeaderBoardEvent);
}

fn update_high_score(score: u32, current_high_score: u32) -> u32 {
    if score > current_high_score {
        info!("NEW HIGHSCORRRRRE!!!");
        return score;
    };
    return current_high_score;
}

fn save_score_board(
    name: &str,
    score: u32,
) {
    let score_board_path = Path::new(SCORE_BOARD_PATH);

    let Ok(mut file) = File::options()
        .append(true)
        .create(true)
        .open(score_board_path)
    else {
        error!("No Score Board File could be opened at: {:?}", SCORE_BOARD_PATH);
        return;
    };

    if let Err(error) = writeln!(file, "{},{}", name, score) {
        error!("Could not write to score board: {}", error);
    }
}

fn load_score_board() -> ScoreBoard {
    let Ok(score_board_file) = fs::read_to_string(SCORE_BOARD_PATH) else {
        warn!("Could not read high score from file: {}", SCORE_BOARD_PATH);
        return ScoreBoard {
            high_score: 0,
            entry: Vec::default() };
    };

    let mut score_board: Vec<ScoreBoardEntry> = score_board_file
        .lines()
        .filter_map(|line| {
            let Some((name, score_as_string)) = line.split_once(',') else {
                error!("Cannot read score board at: {}", SCORE_BOARD_PATH);
                return None;
            };
    
            let Ok(score) = score_as_string.trim().parse::<u32>() else {
                error!("Cannot parse score in {}", SCORE_BOARD_PATH);
                return None;
            };
    
            Some(ScoreBoardEntry {
                name: name.trim().to_string(),
                score,
            })
        })
        .collect();
    
    score_board.sort_by(|a, b| b.score.cmp(&a.score));
    
    let high_score = score_board
        .first()
        .map(|entry| entry.score)
        .unwrap_or(0);
    
    ScoreBoard {
        entry: score_board,
        high_score: high_score,
    }
}

fn spawn_score_board(
    _trigger: On<SpawnLeaderBoardEvent>,
    score_board: Res<ScoreBoard>,
    mut scene_tree: SceneTreeRef,
    mut commands: Commands,
    assets: Res<ScoreBoardAssets>,
) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(assets.score_board_scene.clone()));
    let Some(root) = scene_tree.get().get_root() else {
        return;
    };

    let score_board_label_path = "/root/ScoreBoard/Top5";
    
    let Some(mut top5_label) =
        root.try_get_node_as::<Label>(score_board_label_path)
    else {
        warn!("Could not find {}", score_board_label_path);
        return;
    };

    let content = prepare_leader_board_content(score_board);
    
    top5_label.set_text(&content);
}

fn prepare_leader_board_content(score_board: Res<ScoreBoard>,) -> String {
    return score_board
        .entry
        .iter()
        .take(5)
        .enumerate()
        .map(|(i, entry)| {
            format!("{}. {} {}", i + 1, entry.name, entry.score)
        })
        .collect::<Vec<_>>()
        .join("\n");
}

