use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::character::{CharacterCore, Health, MovementSpeed};
use crate::gamestate::InGameState;
use crate::level_manager::{CurrentLevel, LevelId};
// use crate::gamestate::EnemyResource; 
// use crate::weapon::Weapons;
// use crate::world::map::map::Level;

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<EnemySpawnedMessage>()
        .add_message::<CreateEnemyMessage>()
        .add_systems(
            Update,
            spawn_enemy_system
                .run_if(in_state(LevelId::Level1))
                .run_if(in_state(InGameState::RUNNING))
        );
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Message)]
pub struct EnemySpawnedMessage {
    pub entity: Entity
}

#[derive(Message)]
pub struct CreateEnemyMessage {
    pub position: Vec2
}

fn spawn_enemy_system(
    mut enemy_spawn_reader: MessageReader<CreateEnemyMessage>,
    mut enemy_spawn_writer: MessageWriter<EnemySpawnedMessage>,
    mut commands: Commands,
    current_level: Res<CurrentLevel>
) {
    for message in enemy_spawn_reader.read() {
        let Some(level) = current_level.entity else {
            return;
        };

        let enemy = commands
            .spawn(Transform::from_xyz(
                message.position.x,
                message.position.y,
                0.
            ))
            .id();

        commands.entity(level).add_child(enemy);
        enemy_spawn_writer.write(EnemySpawnedMessage { entity: enemy });
    }
}
