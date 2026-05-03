use bevy::prelude::*;

use crate::character::Health;
use crate::character::enemy::{Enemy, EnemySpawnedMessage};

#[cfg(debug_assertions)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn_enemy_health_bar, update_health_bar_position,update_health_bar_fill));
}

#[derive(Component)]
struct HealthBarFill {
    target: Entity
}
#[derive(Component)]
struct HealthBar {
    target: Entity
}

fn spawn_enemy_health_bar(mut commands: Commands, mut reader: MessageReader<EnemySpawnedMessage>) {
    for message in reader.read() {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(60.0),
                    height: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                BorderColor::all(Color::BLACK),
                HealthBar {
                    target: message.entity
                }
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.9, 0.2)),
                    HealthBarFill {
                        target: message.entity
                    }
                ));
            });
    }
}

// TODO: add despawn
fn despawn_enemy_health_bar() {}

fn update_health_bar_position(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut bars: Query<(&HealthBar, &mut Node)>,
    enemies: Query<&GlobalTransform, With<Enemy>>
) {
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    for (bar, mut node) in bars.iter_mut() {
        let Ok(enemy_transform) = enemies.get(bar.target) else {
            continue;
        };
        if let Ok(screen_pos) = camera.world_to_viewport(
            camera_transform,
            enemy_transform.translation() + Vec3::Y * 30.
        ) {
            node.left = Val::Px(screen_pos.x - 30.0);
            node.top = Val::Px(screen_pos.y);
        }
    }
}

fn update_health_bar_fill(
    enemies: Query<&Health, (With<Enemy>, Changed<Health>)>,
    mut fills: Query<(&HealthBarFill, &mut Node, &mut BackgroundColor)>
) {
    for (fill, mut node, mut color) in fills.iter_mut() {
        let Ok(health) = enemies.get(fill.target) else {
            continue;
        };

        let ratio = (health.current as f32 / health.max as f32).clamp(0.,1.);
        node.width = Val::Percent(ratio * 100.0);

        // interpolate green -> yellow -> red as health drops
        color.0 = if ratio > 0.5 {
            let t = (ratio - 0.5) * 2.0; // 1.0 at full, 0.0 at half
            Color::srgb(1.0 - t, 0.9, 0.2)
        } else {
            let t = ratio * 2.0; // 1.0 at half, 0.0 at empty
            Color::srgb(0.9, 0.9 * t, 0.2)
        };
    }
}
