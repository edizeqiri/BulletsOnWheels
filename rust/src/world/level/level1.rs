use bevy::prelude::*;

use crate::{character::enemy::CreateEnemyMessage, world::level_manager::LevelId};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(1.5))
        .add_systems(FixedUpdate, spawn_enemies_after_time.run_if(in_state(LevelId::Level1)));
}

fn spawn_enemies_after_time(mut enemy_writer: MessageWriter<CreateEnemyMessage>) {
    // todo: make range, maybe random around last enemy death
    enemy_writer.write(CreateEnemyMessage {
        position: Vec2 { x: 100., y: 100. }
    });
}
