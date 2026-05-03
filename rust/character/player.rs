use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;

use crate::character;
use crate::character::{Health, ShootingState, player_collision_groups, square_sprite};
use crate::gamestate::GameState;
use crate::weapon::{ShootEvent, Weapons};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PlayerDeathMessage>().add_systems(
        Update,
        (
            player_shoot_system,
            check_player_zero_health_system,
            handle_player_zero_health_system
        )
            .run_if(in_state(GameState::RUNNING))
    );
}

#[derive(Component)]
pub struct Player;

#[derive(Message)]
pub struct PlayerDeathMessage {
    pub entity: Entity
}

fn player_shoot_system(
    mut event_writer: MessageWriter<ShootEvent>,
    player_query: Query<(Entity, &ShootingState), With<Player>>,
    mut shoot_timer: Local<f32>,
    time: Res<Time>
) {
    let Ok((player, shooting_state)) = player_query.single() else {
        return;
    };

    *shoot_timer -= time.delta_secs();

    if shooting_state.is_shooting && *shoot_timer <= 0.0 {
        event_writer.write(ShootEvent {
            shooter: player,
            collision_groups: player_collision_groups()
        });
        *shoot_timer = 0.01;
    }
}

pub fn create_player_bundle(
    transform: Transform,
    weapons: Weapons,
    max_health: u32,
    name: Name
) -> impl Bundle {
    (
        name,
        character::create_character(transform, weapons.clone(), max_health),
        player_collision_groups(),
        Player,
        square_sprite(Color::Srgba(BLUE))
    )
}

fn check_player_zero_health_system(
    mut death_message: MessageWriter<PlayerDeathMessage>,
    query: Query<(&Health, Entity), (With<Player>, Changed<Health>)>
) {
    for (health, entity) in &query {
        if health.current <= 0 {
            death_message.write(PlayerDeathMessage { entity });
        }
    }
}

fn handle_player_zero_health_system(
    mut commands: Commands,
    mut player_death_messages: MessageReader<PlayerDeathMessage>
) {
    for message in player_death_messages.read() {
        debug!("Player died!");
        commands.entity(message.entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use bevy::MinimalPlugins;
    use bevy::app::App;
    use bevy::prelude::{AppExtStates, Entity, Name, NextState, Transform, With};
    use bevy::state::app::StatesPlugin;
    use test_case::test_case;

    use crate::character::player::{Player, create_player_bundle};
    use crate::character::{Health, player};
    use crate::gamestate::GameState;
    use crate::weapon;
    use crate::weapon::Weapons;

    // ----------- SETUP ----------- //
    pub struct Setup {
        pub app: App,
        pub player: Entity
    }

    impl Setup {
        pub fn new() -> Self {
            // setup App
            let mut app = App::new();

            app.add_plugins(MinimalPlugins)
                .add_plugins(StatesPlugin)
                .init_state::<GameState>()
                .add_plugins(weapon::plugin)
                .add_plugins(player::plugin);

            // setup Entities
            let player = app
                .world_mut()
                .spawn(create_player_bundle(
                    Transform::from_xyz(1.0, 1.0, 0.0),
                    Weapons::default(),
                    1,
                    Name::from("Player")
                ))
                .id();

            // start app
            app.update();

            Self { app, player }
        }
    }

    // ----------- TEST ----------- //

    #[test_case(GameState::RUNNING, 0 ; "player dies in game")]
    #[test_case(GameState::START, 1 ; "player can not die in menu")]
    fn zero_health_player_is_despawned(game_state: GameState, expected_num_players: usize) {
        let mut setup = Setup::new();

        {
            let world = setup.app.world_mut();
            world.resource_mut::<NextState<GameState>>().set(game_state);

            // when: player.health.current = 0
            world
                .query::<&mut Health>()
                .get_mut(world, setup.player)
                .unwrap()
                .current = 0;
        }

        // 1. message: health changes
        setup.app.update();
        // 2. message: player died
        setup.app.update();

        // then: no player exists
        let world = setup.app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<Player>>()
                .iter(&world)
                .len(),
            expected_num_players
        );
    }
}
