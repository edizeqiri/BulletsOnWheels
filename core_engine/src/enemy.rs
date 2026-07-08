use bevy::prelude::*;

use crate::gamestate::InGameState;

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<EnemySpawnedMessage>()
        .add_message::<CreateEnemyMessage>()
        .add_systems(
            Update,
            spawn_enemy_system.run_if(in_state(InGameState::RUNNING)),
        );
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Message)]
pub struct EnemySpawnedMessage {
    pub entity: Entity,
}

#[derive(Message)]
pub struct CreateEnemyMessage {
    pub position: Vec2,
}

fn spawn_enemy_system(
    mut enemy_spawn_reader: MessageReader<CreateEnemyMessage>,
    mut enemy_spawn_writer: MessageWriter<EnemySpawnedMessage>,
    mut commands: Commands,
) {
    for message in enemy_spawn_reader.read() {
        let enemy = commands
            .spawn(Transform::from_xyz(
                message.position.x,
                message.position.y,
                0.,
            ))
            .id();

        enemy_spawn_writer.write(EnemySpawnedMessage { entity: enemy });
    }
}
