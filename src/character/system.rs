use crate::character::Health;
use crate::character::enemy::Enemy;
use crate::character::player::Player;
use bevy::app::{App, Update};
use bevy::log::debug;
use bevy::prelude::{Changed, Commands, Entity, Query, With};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_player_zero_health_system)
        .add_systems(Update, handle_enemy_zero_health_system);
}

fn handle_player_zero_health_system(
    mut commands: Commands,
    query: Query<(&Health, Entity), (With<Player>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0 {
            debug!("Player died!");
            commands.entity(entity).despawn();
        }
    }
}

fn handle_enemy_zero_health_system(
    mut commands: Commands,
    query: Query<(&Health, Entity), (With<Enemy>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0 {
            debug!("Enemy died!");
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::character::enemy::Enemy;
    use crate::character::system::{
        handle_enemy_zero_health_system, handle_player_zero_health_system,
    };
    use bevy::a11y::AccessibilitySystems::Update;
    use bevy::app::App;

    // fn before_all() {
    //     // setup App
    //     let mut app = App::new()
    //         .add_systems(bevy::app::Update, handle_player_zero_health_system)
    //         .add_systems(bevy::app::Update, handle_enemy_zero_health_system);
    //
    //     // setup Entities
    //     let enemy_id = app
    //         .world_mut()
    //         .spawn(Enemy {
    //
    //         });
    // }
}
