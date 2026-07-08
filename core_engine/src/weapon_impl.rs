use std::time::Duration;

use bevy::math::Vec2;
use bevy::prelude::*;

use crate::character::Aim;
use crate::gamestate::InGameState;
use crate::projectile::{Projectile, ProjectileBundle, Velocity, create_projectile};
use crate::weapon::Damage;

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<ShootMessage>()
        .add_message::<ProjectileShot>()
        .add_systems(
            Update,
            on_shoot_message_system.run_if(in_state(InGameState::RUNNING)),
        )
        .add_systems(
            Update,
            update_projectile_system.run_if(in_state(InGameState::RUNNING)),
        );
}

#[derive(Default, Clone)]
pub enum WeaponKind {
    #[default]
    GUN,
    BOW,
    STAFF,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Speed {
    pub current: f32,
    pub max: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            current: 0.,
            max: 150.,
        }
    }
}
#[derive(Component, Debug, Clone, Default)]
pub struct FireRate(pub f32);

#[derive(Component, Clone, Default)]
pub struct WeaponKindComponent(pub WeaponKind);

#[derive(Component, Default, Clone)]
pub struct Weapon {
    damage: Damage,
    pub speed: Speed,
    fire_rate: FireRate,
    weapon_kind: WeaponKindComponent,
}

impl Weapon {
    pub fn new(damage: f32, _speed: f32, fire_rate: f32, weapon_kind: WeaponKind) -> Self {
        Self {
            damage: Damage(damage),
            fire_rate: FireRate(fire_rate),
            weapon_kind: WeaponKindComponent(weapon_kind),
            ..default()
        }
    }
    pub(crate) fn shoot(&self, direction: Vec2) -> ProjectileBundle {
        create_projectile(self.damage, self.speed, direction)
    }
}

#[derive(Message)]
pub struct ShootMessage {
    pub shooter: Entity,
    pub aim: Aim,
}

#[derive(Component, Clone)]
pub struct Weapons {
    pub list: Vec<Weapon>,
}
#[derive(Component)]
pub struct Shooter(pub Entity);

#[derive(Message)]
pub struct ProjectileShot {
    pub projectile: Entity,
}

pub(crate) fn on_shoot_message_system(
    mut commands: Commands,
    mut shoot_message: MessageReader<ShootMessage>,
    mut shoot_writer: MessageWriter<ProjectileShot>,
    mut shooter_query: Query<(&Transform, &Weapon, &Aim)>,
    time: Res<Time>,
) {
    for message in shoot_message.read() {
        if let Ok((transform, weapon, aim)) = shooter_query.get_mut(message.shooter) {
            let projectile_bundle = weapon.shoot(message.aim.vec);
            let mut new_transform = transform.clone();

            // rotate sprite to aim direction
            if aim.vec.length_squared() > 0.0 {
                new_transform.rotation = Quat::from_rotation_z(aim.vec.to_angle());
            }

            let projectile = commands
                .spawn_empty()
                .insert(projectile_bundle)
                .insert(new_transform)
                .insert(SpawnedTime(time.elapsed()))
                .insert(Shooter(message.shooter))
                .insert(DespawnOnExit(InGameState::RUNNING))
                .id();
            shoot_writer.write(ProjectileShot { projectile });
        }
    }
}
#[derive(Component)]
pub struct SpawnedTime(Duration);

fn update_projectile_system(
    time: Res<Time>,
    projectile_query: Query<
        (&mut Transform, &Velocity, &SpawnedTime, &mut Speed),
        With<Projectile>,
    >,
) {
    for (mut transform, velocity, _spawned_time, speed) in projectile_query {
        /*/
        if speed.current <= speed.max {
            //info!("current {}, max {}", speed.current, speed.max);
            speed.current =
                (time.elapsed_secs() * 5000. - spawned_time.0.as_secs_f32() * 10.).sqrt();
            speed.current = speed.current.clamp(0., speed.max);
        }*/

        transform.translation.x += velocity.0.x * speed.max * time.delta_secs();
        transform.translation.y += velocity.0.y * speed.max * time.delta_secs();
    }
}

impl Default for Weapons {
    fn default() -> Self {
        Weapons {
            list: vec![
                Weapon::new(1., 1000., 0.5, WeaponKind::BOW),
                Weapon::new(1., 250., 5., WeaponKind::GUN),
            ],
        }
    }
}
