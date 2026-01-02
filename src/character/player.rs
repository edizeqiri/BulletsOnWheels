use crate::character;
use crate::character::{ENEMY_GROUP, PLAYER_GROUP, square_sprite, Health};
use crate::weapon::Weapons;
use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;

pub(super) fn plugin(app: &mut App) {
    app
        .add_message::<PlayerDeathMessage>()
        .add_systems(Update, check_player_zero_health_system)
        .add_systems(Update, handle_player_zero_health_system);
}

#[derive(Component)]
pub struct Player;

#[derive(Message)]
pub struct PlayerDeathMessage {
    pub entity: Entity,
}

pub fn create_player_bundle(
    transform: Transform,
    weapons: Weapons,
    max_health: u32,
    name: Name,
) -> impl Bundle {
    (
        name,
        character::create_character(transform, weapons.clone(), max_health),
        CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP),
        Player,
        square_sprite(Color::Srgba(BLUE)),
    )
}

fn check_player_zero_health_system(
    mut death_message: MessageWriter<PlayerDeathMessage>,
    query: Query<(&Health, Entity), (With<Player>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0 {
            death_message.write(PlayerDeathMessage {
                entity
            }
            );
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
    use crate::character::player::{check_player_zero_health_system, create_player_bundle, handle_player_zero_health_system, Player, PlayerDeathMessage};
    use crate::character::{player, Health};
    use crate::weapon::{Weapon, Weapons};
    use bevy::app::{App, Update};
    use bevy::prelude::{Entity, Name, Transform, With};

    // ----------- SETUP ----------- //
    pub struct Setup {
        pub app: App,
        pub player: Entity
    }

    impl Setup {
        pub fn new() -> Self {
            // setup App
            let mut app = App::new();

            app
                .add_plugins(player::plugin);

            // setup Entities
            let player = app.world_mut().spawn(create_player_bundle(
                Transform::from_xyz(1.0, 1.0, 0.0),
                Weapons::default(),
                1,
                Name::from("Player"),
            )).id();

            // start app
            app.update();

            // confirm setup: there is 1 Player
            let world = app.world_mut();
            let player = world
                .query_filtered::<Entity, With<Player>>()
                .single(world)
                .expect("Expected exactly one Player entity");

            Self { app, player }
        }
    }

    // ----------- TEST ----------- //

    #[test]
    fn zero_health_player_is_despawned() {
        let mut setup = Setup::new();

        {
            let world = setup.app.world_mut();

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
            0
        );
    }
}