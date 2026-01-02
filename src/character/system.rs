use crate::character::enemy::Enemy;
use crate::character::player::Player;
use crate::character::{CharacterBundle, Health};
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
    use crate::character::enemy::{create_enemy_bundle, Enemy};
    use crate::character::system::{
        handle_enemy_zero_health_system, handle_player_zero_health_system,
    };

    use crate::character::player::{create_player_bundle, Player};
    use crate::character::Health;
    use crate::weapon::bow::Bow;
    use crate::weapon::{Weapon, Weapons};
    use bevy::app::App;
    use bevy::prelude::{Entity, Transform, With, World};

    fn setup_test_app() -> App {
        // setup App
        let mut test_app = App::new();

        test_app
            .add_systems(bevy::app::Update, handle_player_zero_health_system)
            .add_systems(bevy::app::Update, handle_enemy_zero_health_system);

        // setup Entities
        let weapons = Weapons(vec![Weapon::Bow(Bow::new(1, 1000., 0.5))]);

        test_app.world_mut().spawn(create_player_bundle(
            Transform::from_xyz(1.0, 1.0, 0.0),
            weapons.clone(),
            1,
            "Player",
        ));

        test_app.world_mut().spawn(create_enemy_bundle(
            Transform::from_xyz(0.0, 0.0, 0.0),
            weapons.clone(),
            1,
            "Enemy",
        ));

        // start app
        test_app.update();

        test_app
    }

    #[test]
    fn zero_health_player_is_despawned() {
        let mut test_app = setup_test_app();

        // pre-check: there is 1 Enemy and 1 Player
        {
            let world = test_app.world_mut();
            let player = get_single_player(world);
            get_single_enemy(world);

            // when: player.health.current = 0
            world
                .query::<&mut Health>()
                .get_mut(world, player)
                .unwrap()
                .current = 0;
        }

        test_app.update();

        // then: 0 Player, 1 Enemy
        let world = test_app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<Player>>()
                .iter(&world)
                .len(),
            0
        );
        assert_eq!(
            world
                .query_filtered::<Entity, With<Enemy>>()
                .iter(&world)
                .len(),
            1
        );
    }

    #[test]
    fn zero_health_enemy_is_despawned() {
        let mut test_app = setup_test_app();

        // pre-check: 1 Enemy, 1 Player
        {
            let world = test_app.world_mut();
            let enemy = get_single_enemy(world);
            get_single_player(world);

            // when: enemy.health.current = 0
            world
                .query::<&mut Health>()
                .get_mut(world, enemy)
                .unwrap()
                .current = 0;
        }

        test_app.update();

        // then: 1 Player, 0 Enemy
        let world = test_app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<Player>>()
                .iter(&world)
                .len(),
            1
        );
        assert_eq!(
            world
                .query_filtered::<Entity, With<Enemy>>()
                .iter(&world)
                .len(),
            0
        );
    }

    // ----------- HELPER FUNCTIONS ----------- //

    fn get_single_player(world: &mut World) -> Entity {
        world
            .query_filtered::<Entity, With<Player>>()
            .single(world)
            .expect("Expected exactly one Player entity")
    }

    fn get_single_enemy(world: &mut World) -> Entity {
        world
            .query_filtered::<Entity, With<Enemy>>()
            .single(world)
            .expect("Expected exactly one Enemy entity")
    }
}
