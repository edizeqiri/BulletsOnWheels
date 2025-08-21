use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Projectile {
    damage: f32,
    speed: f32,
    direction: Vec2,
}


#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    transform: Transform,
    sprite: Sprite,
}


pub fn create_projectile(damage: f32, speed: f32) -> ProjectileBundle {
    ProjectileBundle {
        projectile: Projectile {
            damage,
            speed,
            direction: Vec2::new(1.0, 1.0),
        },
        sprite: circle_sprite(Color::Srgba(Srgba::GREEN)),
        transform: Default::default(),
    }
}

fn circle_sprite(color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::new(75.0, 25.0)),
        ..Default::default()
    }
}

pub fn move_projectile(projectile: &Projectile, mut transform: Mut<Transform>) {
    let step = Vec3::new(
        projectile.direction.x + projectile.speed,
        projectile.direction.y + projectile.speed,
        0.0,
    );
    let old = transform.translation;
    let new = old + step;
    transform.translation = new;
}

