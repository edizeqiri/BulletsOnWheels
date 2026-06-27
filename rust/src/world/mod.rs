use bevy::prelude::*;

use crate::character::player::Player;
use crate::gamestate::CharacterDeathMessage;
use crate::world::level::level1;
use crate::world::level_manager::LoadLevelMessage;
mod level;
pub(crate) mod level_manager;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(level1::plugin)
        .add_plugins(level_manager::LevelManagerPlugin)
        .add_systems(Update, restart_on_death);
}

fn restart_on_death(
    mut commands: Commands,
    mut player_death_message: MessageReader<CharacterDeathMessage>,
    player_query: Query<(), With<Player>>
) {
    for message in player_death_message.read() {
        if player_query.get(message.target).is_ok() {
            commands.trigger(LoadLevelMessage {
                level_id: level_manager::LevelId::MainMenu
            });
        }
    }
}
