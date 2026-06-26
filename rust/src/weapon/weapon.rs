use std::time::{Duration, Instant};

use bevy::math::{FloatPow, Vec2};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::character::Aim;
use crate::gamestate::AppState;
use crate::weapon::Damage;
use crate::weapon::projectile::{Projectile, ProjectileBundle, Velocity, create_projectile};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ShootMessage>()
        .add_systems(
            Update,
            on_shoot_message_system.run_if(in_state(AppState::RUNNING))
        )
        .add_systems(Update, update_projectile_system);
}

#[derive(Component, GodotNode, Default, Clone)]
#[godot_node(base(Node2D), class_name(RWeapon2D))]
pub struct Weapon {
    #[export_fields(value(export_type(f32), default(1.)))]
    damage: Damage,

    // TODO(bug): this export is somehow not working
    pub speed: Speed,

    #[export_fields(value(export_type(f32), default(0.)))]
    fire_rate: FireRate,

    #[export_fields(value(export_type(WeaponKind), default(WeaponKind::GUN)))]
    weapon_kind: WeaponKindComponent
}

#[derive(GodotConvert, Var, Export, Default, Clone)]
#[godot(via = GString)] // provides enum as string
pub enum WeaponKind {
    #[default]
    GUN,
    BOW,
    STAFF
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Speed {
    pub current: f32,
    pub max: f32
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            current: 0.,
            max: 150.
        }
    }
}
#[derive(Component, Debug, Clone, Default)]
pub struct FireRate(pub f32);
#[derive(Component, Clone, Default)]
pub struct WeaponKindComponent(pub WeaponKind);

impl Weapon {
    pub fn new(damage: f32, speed: f32, fire_rate: f32, weapon_kind: WeaponKind) -> Self {
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
    pub aim: Aim
}

#[derive(Component, Clone)]
pub struct Weapons {
    pub list: Vec<Weapon>
}
#[derive(Component)]
pub struct Shooter(pub Entity);

#[derive(AssetCollection, Resource)]
pub(crate) struct ProjectileAssets {
    #[asset(path = "scenes/characters/projectile.tscn")]
    pub projectile_scene: Handle<GodotResource>
}

pub(crate) fn on_shoot_message_system(
    mut commands: Commands,
    mut shoot_message: MessageReader<ShootMessage>,
    mut shooter_query: Query<(&Transform, &Weapon, &Aim)>,
    assets: Option<Res<ProjectileAssets>>,
    time: Res<Time>
) {
    // If the projectile assets are not yet loaded/inserted, consume any queued
    // shoot messages (to avoid a burst once assets arrive) and skip spawning
    // projectiles.
    let assets = match assets {
        Some(a) => a,
        None => {
            for _ in shoot_message.read() { /* drop events until assets are ready */ }
            return;
        }
    };

    for message in shoot_message.read() {
        if let Ok((transform, weapon, aim)) = shooter_query.get_mut(message.shooter) {
            let projectile_bundle = weapon.shoot(message.aim.vec);
            let mut new_transform = transform.clone();

            // rotate sprite to aim direction
            if aim.vec.length_squared() > 0.0 {
                new_transform.rotation = Quat::from_rotation_z(aim.vec.to_angle());
            }

            commands
                .spawn_empty()
                .insert(projectile_bundle)
                .insert(new_transform)
                .insert(SpawnedTime(time.elapsed()))
                .insert(Shooter(message.shooter))
                .insert(GodotScene::from_handle(assets.projectile_scene.clone()));
        }
    }
}
#[derive(Component)]
pub struct SpawnedTime(Duration);

fn update_projectile_system(
    time: Res<Time>,
    projectile_query: Query<
        (&mut Transform, &Velocity, &SpawnedTime, &mut Speed),
        With<Projectile>
    >
) {
    for (mut transform, velocity, spawned_time, mut speed) in projectile_query {
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
            ]
        }
    }
}
