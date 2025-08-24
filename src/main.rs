mod character;
mod projectile;
mod weapon;

use crate::character::enemy::{Enemy, enemy_additions};
use crate::character::player::{Player, player_additions};
use crate::character::{CharacterBundle, Health, enemy_collision_groups, player_collision_groups, square_sprite};
use crate::projectile::Projectile;
use crate::weapon::Shootable;
use crate::weapon::Weapon;
use bevy::ecs::bundle::DynamicBundle;
use bevy::input::keyboard::Key::Play;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use bevy::color::palettes::css::{GREEN, YELLOW};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(logger()))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(3)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, init)
        .add_systems(FixedUpdate, shoot_every_second)
        .add_systems(Update, (handle_sensor_collision, debug_all_collision_events))
        .run();
}

fn logger() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=error,wgpu_hal=error,rustydefense=debug".into(),
        level: bevy::log::Level::DEBUG,
        custom_layer: |_| None,
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn init(mut commands: Commands) {
    commands.spawn((
        character::create_character(Transform::from_xyz(100.0, 0.0, 0.0)),
        player_additions(),
    ));
    commands.spawn((
        character::create_character(Transform::from_xyz(-100.0, 0.0, 0.0)),
        enemy_additions(),
    ));
}

fn shoot_every_second(
    mut commands: Commands,
    player_query: Query<&Weapon, With<Player>>,
    enemy_query: Query<&Weapon, With<Enemy>>,
    time: Res<Time<Fixed>>,
) {
    for weapon in &player_query {
        commands.spawn((
            weapon.shoot(50.0, Vec2::new(-1., 0.)),
            square_sprite(Color::Srgba(GREEN)),
            player_collision_groups(),
        ));
    }

    for weapon in &enemy_query {
        commands.spawn((
            weapon.shoot(100.0, Vec2::new(1., 0.)),
            square_sprite(Color::Srgba(YELLOW)),
            enemy_collision_groups()
        ));
    }
}

fn handle_sensor_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    projectile_query: Query<&Projectile>,
    mut health_query: Query<&mut Health>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if let Ok(bullet) = projectile_query.get(*entity1) {
                if let Ok(mut health) = health_query.get_mut(*entity2) {
                    health.current = health.current.saturating_sub(bullet.damage);
                    commands.entity(*entity1).despawn();
                    info!("Collision Event!")
                }
            } else if let Ok(bullet) = projectile_query.get(*entity2) {
                if let Ok(mut health) = health_query.get_mut(*entity1) {
                    health.current = health.current.saturating_sub(bullet.damage);
                    commands.entity(*entity2).despawn();
                    info!("Collision Event!")
                }
            }
        }
    }
}

fn debug_all_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
) {
    let event_count = collision_events.len();
    if event_count > 0 {
        info!("=== {} collision events this frame ===", event_count);
    }

    for (i, event) in collision_events.read().enumerate() {
        info!("Event {}: {:?}", i, event);
    }
}


