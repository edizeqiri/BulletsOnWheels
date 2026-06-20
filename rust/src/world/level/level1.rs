use bevy::prelude::*;

use crate::character::enemy::CreateEnemyMessage;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_seconds(3.))
        .add_systems(FixedUpdate, spawn_enemies_after_time);
}

fn spawn_enemies_after_time(mut enemy_writer: MessageWriter<CreateEnemyMessage>) {

    // todo: make range, maybe random around last enemy death
    enemy_writer.write(CreateEnemyMessage { position: Vec2 { x: 100., y: 100. }});
}
