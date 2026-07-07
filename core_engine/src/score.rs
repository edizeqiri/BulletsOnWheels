use std::fs::{self, File};
use std::path::Path;

use ::std::io::Write;
use bevy::prelude::*;

use crate::gamestate::{CharacterDeathMessage, InGameState};
use crate::player::Player;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(load_score_board())
        .insert_resource(ScoreBoard::default())
        .add_systems(
            Update,
            (init_score, score_tracker).run_if(in_state(InGameState::RUNNING)),
        )
        .add_observer(save_score_board)
        .add_observer(despawn_ask_for_player_name_system);
}

#[derive(Event, Clone, Default)]
pub struct NameEnteredEvent {
    pub name: String,
}

#[derive(Component)]
pub struct LiveScore {
    pub count: i32,
    pub highscore: i32,
    pub is_new_highscore: bool,
}

impl Default for LiveScore {
    fn default() -> Self {
        Self {
            count: 0,
            highscore: -1,
            is_new_highscore: false,
        }
    }
}

const SCORE_BOARD_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/score_board.csv");

#[derive(Resource, Default)]
pub struct ScoreBoard {
    pub entries: Vec<ScoreBoardEntry>,
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

pub struct ScoreBoardEntry {
    pub name: String,
    pub score: i32,
}

fn init_score(mut score_query: Query<&mut LiveScore>, score_board: Res<ScoreBoard>) {
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
    mut score_query: Query<&mut LiveScore>,
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

fn save_score_board(
    trigger: On<NameEnteredEvent>,
    score_query: Query<&LiveScore>,
    mut score_board: ResMut<ScoreBoard>,
    mut commands: Commands,
) {
    let Ok(score) = score_query.single() else {
        info!("No score found => No high score can be saved.");
        return;
    };
    let entered_name = &trigger.event().name;

    // todo: limit to 5 and save all entries instead of only one
    score_board.entries.push(ScoreBoardEntry {
        name: entered_name.clone(),
        score: score.count,
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
            entries: Vec::default(),
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
                score: score as i32,
            })
        })
        .collect();

    score_board_entries.sort_by(|a, b| b.score.cmp(&a.score));

    ScoreBoard {
        entries: score_board_entries,
    }
}

#[derive(Component)]
pub struct DeathHighscoreScene;

fn despawn_ask_for_player_name_system(
    _trigger: On<SpawnLeaderBoardEvent>,
    death_high_score_query: Query<Entity, With<DeathHighscoreScene>>,
    mut commands: Commands,
) {
    let Ok(deaht_high_score_scene) = death_high_score_query.single() else {
        error!("Could not despawn death highscore scene.");
        return;
    };
    commands.entity(deaht_high_score_scene).despawn();
}
