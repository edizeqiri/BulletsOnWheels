use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::character::Health;
use crate::projectile::Projectile;
use crate::world::walls::Wall;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(3)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, handle_sensor_collision);
}

/// Vibe Coded
fn handle_sensor_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    projectile_query: Query<&Projectile>,
    wall_query: Query<(), With<Wall>>,
    mut health_query: Query<&mut Health>
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            let (projectile_entity, other_entity) =
                match (projectile_query.get(*e1), projectile_query.get(*e2)) {
                    (Ok(_), Err(_)) => (*e1, *e2),
                    (Err(_), Ok(_)) => (*e2, *e1),
                    _ => continue
                };

            let projectile = projectile_query.get(projectile_entity).unwrap();

            // --- CASE 1: projectile hits wall ---
            if wall_query.get(other_entity).is_ok() {
                debug!("Projectile hit a wall");
                commands
                    .entity(projectile_entity)
                    .queue_silenced(|entity: EntityWorldMut| {
                        entity.despawn();
                    });
                continue;
            }

            // --- CASE 2: projectile hits something with health ---
            if let Ok(mut health) = health_query.get_mut(other_entity) {
                health.current = health.current.saturating_sub(projectile.damage);

                commands
                    .entity(projectile_entity)
                    .queue_silenced(|entity: EntityWorldMut| {
                        entity.despawn();
                    });
            }
        }
    }
}
