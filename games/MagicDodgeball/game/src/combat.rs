use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use avian2d::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

/* Projectile 
 * Collision system
 * Health
 * 
*/

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Projectile,
}

#[derive(Default, Component)]
 struct Projectile;

 #[derive(Bundle)]
 struct ProjectileBundle {
     projectile: Projectile,
     collision_layers: CollisionLayers,
     collider: Collider,
     rigid_body: RigidBody,
     velocity: LinearVelocity,
     sprite: Sprite,
     transform: Transform
 }

 impl ProjectileBundle {
     fn new(speed: f32, direction: Vec2) -> Self {
         Self {
             projectile: Projectile,
             rigid_body: RigidBody::Kinematic,
             collider: Collider::circle(4.0),
             collision_layers: CollisionLayers::new(
                 GameLayer::Projectile,
                 [GameLayer::Enemy],
             ),
             velocity: LinearVelocity(direction * speed),
             sprite: Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(8.0)),
             transform: Transform::default(),
         }
     }
 }


#[derive(Message)] 
struct ShotFiredMessage {
     origin: Vec2,
     direction: Vec2,
}