use crate::character::{self, MovementSpeed};
use crate::character::{CharacterCore, Health, ShootingState};
use bevy::prelude::*;
use godot_bevy::prelude::*;

// TODO: gamestate
//use crate::gamestate::GameState;
//use crate::weapon::{ShootEvent, Weapons};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PlayerDeathMessage>().add_systems(
        Update,
        (
            player_shoot_system,
            check_player_zero_health_system,
            handle_player_zero_health_system,
        ), //.run_if(in_state(GameState::RUNNING))
    );
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, GodotNode)]
#[godot_node(base(CharacterBody2D), class_name(RPlayer2D))]
pub struct PlayerBundle {
    player: Player,

    #[export_fields(
        current(export_type(f32), default(10.)),
        max(export_type(f32), default(10.))
    )]
    health: Health,

    #[export_fields(value(export_type(f32), default(200.)))]
    speed: MovementSpeed,

    core: CharacterCore,
}

#[derive(Message)]
pub struct PlayerDeathMessage {
    pub entity: Entity,
}

fn player_shoot_system(
    player_query: Query<(Entity, &ShootingState), With<Player>>,
    mut shoot_timer: Local<f32>,
    time: Res<Time>,
) {
}

fn check_player_zero_health_system(
    mut death_message: MessageWriter<PlayerDeathMessage>,
    query: Query<(&Health, Entity), (With<Player>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0. {
            death_message.write(PlayerDeathMessage { entity });
        }
    }
}

fn handle_player_zero_health_system(
    mut commands: Commands,
    mut player_death_messages: MessageReader<PlayerDeathMessage>,
) {
    for message in player_death_messages.read() {
        info!("Player dead");
        commands.entity(message.entity).despawn();
    }
}
