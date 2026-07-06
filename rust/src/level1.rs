use bevy::prelude::*;
use godot::classes::{CollisionShape2D, RectangleShape2D};
use godot_bevy::prelude::*;
use godot_bevy_macros::GodotNode;
use rand::Rng;

use crate::enemy::CreateEnemyMessage;
use crate::gamestate::InGameState;
use crate::level_manager::LevelId;
use crate::score;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(score::plugin)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(
            Update,
            spawn_enemies_after_time
                .run_if(in_state(LevelId::Level1))
                .run_if(in_state(InGameState::RUNNING))
        );
}
#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component, Debug, GodotNode, Default)]
#[godot_node(base(Area2D), class_name(RSpawnPoint2D))]
struct SpawnArea;

fn spawn_enemies_after_time(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut enemy_writer: MessageWriter<CreateEnemyMessage>,
    spawn_area_query: Query<&GodotNodeHandle, With<SpawnArea>>,
    mut godot: GodotAccess
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for handle in spawn_area_query {
        let mut rng = rand::rng();
        let node = godot.get::<RSpawnPoint2D>(*handle);

        let col_shape = node.get_node_as::<CollisionShape2D>("CollisionShape2D");
        let shape = col_shape.get_shape().unwrap().cast::<RectangleShape2D>();
        let rect = (shape.get_rect().size / 2.0);
        let center = node.get_global_position();

        debug!("view: {:?}", rect);
        debug!("center: {:?}", center);
        let x = rng.random_range((center.x - rect.x)..(center.x + rect.x));
        let y = rng.random_range((center.y - rect.y)..(center.y + rect.y));
        debug!("x: {} | y: {}", x, y);
        enemy_writer.write(CreateEnemyMessage {
            position: Vec2 { x, y }
        });
    }
}
