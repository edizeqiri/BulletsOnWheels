use std::time::Duration;

use bevy::app::{App, Update};
use bevy::prelude::{Commands, EntityWorldMut, Fixed, MessageReader, Query, Time};
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::character::Health;
use crate::projectile::Projectile;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(3)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, handle_sensor_collision);
}

fn handle_sensor_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    projectile_query: Query<&Projectile>,
    mut health_query: Query<&mut Health>
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if let Ok(bullet) = projectile_query.get(*entity1) {
                if let Ok(mut health) = health_query.get_mut(*entity2) {
                    health.current = health.current.saturating_sub(bullet.damage);

                    commands
                        .entity(*entity1)
                        .queue_silenced(|entity: EntityWorldMut| {
                            entity.despawn();
                        });
                }
            } else if let Ok(bullet) = projectile_query.get(*entity2)
                && let Ok(mut health) = health_query.get_mut(*entity1)
            {
                health.current = health.current.saturating_sub(bullet.damage);

                commands
                    .entity(*entity2)
                    .queue_silenced(|entity: EntityWorldMut| {
                        entity.despawn();
                    });
            }
        }
    }
}
