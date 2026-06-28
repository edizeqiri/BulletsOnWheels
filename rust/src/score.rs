use std::fs::{self, File};
use std::path::Path;

use ::std::io::Write;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::prelude::*;
use godot::classes::{Label, RichTextLabel};
use godot_bevy::prelude::*;

use crate::gamestate::{AppState, CharacterDeathMessage, ExitGameMessage, InGameState};
use crate::level_manager::LevelId::Level1;
use crate::level_manager::{CurrentLevel, LevelId, Score};
use crate::player::Player;
use crate::world::NameEnteredEvent;

pub(crate) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::RUNNING).load_collection::<ScoreBoardAssets>()
    )
    .insert_resource(ScoreBoard::default())
    .add_systems(
        Update,
        (init_score, score_tracker, update_score_label)
            .run_if(in_state(LevelId::Level1))
            .run_if(in_state(InGameState::RUNNING))
    )
    .add_systems(
        Update,
        update_score_board
            .run_if(in_state(InGameState::DEFEAT))
            .run_if(in_state(LevelId::Level1))
    )
    .add_observer(save_score_board)
    .add_observer(spawn_score_board);
}

const SCORE_BOARD_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/score_board.csv");

#[derive(AssetCollection, Resource)]
pub struct ScoreBoardAssets {
    #[asset(path = "scenes/defeat/score_board.tscn")]
    pub score_board_scene: Handle<GodotResource>,

    score_board_is_loaded: bool
}

#[derive(Resource, Default)]
struct ScoreBoard {
    entries: Vec<ScoreBoardEntry>
}

impl ScoreBoard {
    fn get_high_score(&self) -> i32 {
        let Some(high_score) = self.entries.iter().max_by_key(|e| e.score) else {
            return 0;
        };
        high_score.score as i32
    }
}

#[derive(Event)]
pub struct SpawnLeaderBoardEvent;

struct ScoreBoardEntry {
    name: String,
    score: i32
}

fn init_score(mut score_query: Query<&mut Score>, score_board: Res<ScoreBoard>) {
    let Ok(mut score) = score_query.single_mut() else {
        return;
    };
    if score.highscore != -1 {
        return;
    }
    score.highscore = score_board.get_high_score();
}

fn score_tracker(
    mut death_message_reader: MessageReader<CharacterDeathMessage>,
    player_query: Query<Entity, With<Player>>,
    mut score_query: Query<&mut Score>
) {
    for message in death_message_reader.read() {
        let Ok(player) = player_query.single() else {
            continue;
        };
        if message.source != player {
            continue;
        };

        let Ok(mut score) = score_query.single_mut() else {
            return;
        };

        score.count += 1;
        if score.count > score.highscore {
            score.highscore = score.count;
            score.is_new_highscore = true;
        };
    }
}

// todo: this function shall be "level state" dependent
fn update_score_label(
    score_query: Query<&Score, Changed<Score>>,
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

fn save_score_board(
    trigger: On<NameEnteredEvent>,
    score_query: Query<&Score>,
    mut score_board: ResMut<ScoreBoard>,
    mut commands: Commands
) {
    let Ok(score) = score_query.single() else {
        info!("No score found => No high score can be saved.");
        return;
    };
    let entered_name = &trigger.event().name;

    // todo: limit to 5 and save all entries instead of only one
    score_board.entries.push(ScoreBoardEntry {
        name: entered_name.clone(),
        score: score.count
    });

    score_board.entries.sort_by(|a, b| b.score.cmp(&a.score));

    let score_board_path = Path::new(SCORE_BOARD_PATH);

    let file = File::options()
        .append(true)
        .create(true)
        .open(score_board_path);

    if file.is_ok() {
        if let Err(error) = writeln!(file.unwrap(), "{},{}", entered_name, score.count) {
            error!("Could not write to score board: {}", error);
        }
    } else {
        error!(
            "No Score Board File could be opened at: {:?}, Scores will not be saved.",
            SCORE_BOARD_PATH
        );
    };

    commands.trigger(SpawnLeaderBoardEvent);
}

fn load_score_board() -> ScoreBoard {
    let Ok(score_board_file) = fs::read_to_string(SCORE_BOARD_PATH) else {
        warn!("Could not read high score from file: {}", SCORE_BOARD_PATH);
        return ScoreBoard {
            entries: Vec::default()
        };
    };

    let mut score_board_entries: Vec<ScoreBoardEntry> = score_board_file
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
                score: score as i32
            })
        })
        .collect();

    score_board_entries.sort_by(|a, b| b.score.cmp(&a.score));

    ScoreBoard {
        entries: score_board_entries
    }
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
