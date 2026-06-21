use bevy::math::Vec2;
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

    #[export_fields(value(export_type(f32), default(1.)))]
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
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(2.)
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
            speed: Speed(speed),
            fire_rate: FireRate(fire_rate),
            weapon_kind: WeaponKindComponent(weapon_kind)
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
    assets: Option<Res<ProjectileAssets>>
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
            if aim.vec.length_squared() > 0.0 {
                new_transform.rotation = Quat::from_rotation_z(aim.vec.to_angle());
            }
            commands
                .spawn_empty()
                .insert(projectile_bundle)
                .insert(new_transform)
                .insert(Shooter(message.shooter))
                .insert(GodotScene::from_handle(assets.projectile_scene.clone()));
        }
    }
}

fn update_projectile_system(
    projectile_query: Query<(&mut Transform, &Velocity), With<Projectile>>
) {
    for (mut transform, velocity) in projectile_query {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
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
