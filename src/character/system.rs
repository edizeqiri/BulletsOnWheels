use crate::character::Health;
use crate::character::enemy::Enemy;
use crate::character::player::Player;
use bevy::app::{App, Update};
use bevy::log::debug;
use bevy::prelude::{Changed, Commands, Entity, MessageWriter, Query, With};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_enemy_zero_health_system);
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
    use crate::character::enemy::{Enemy, create_enemy_bundle};
    use crate::character::system::handle_enemy_zero_health_system;

    use crate::character::Health;
    use crate::character::player::{Player, create_player_bundle};
    use crate::weapon::{Weapon, Weapons};
    use bevy::app::{App, Update};
    use bevy::prelude::{Entity, Name, Transform, With, World};

    fn setup_test_app() -> App {
        // setup App
        let mut test_app = App::new();

        test_app.add_systems(Update, handle_enemy_zero_health_system);
        // setup Entities

        test_app.world_mut().spawn(create_player_bundle(
            Transform::from_xyz(1.0, 1.0, 0.0),
            Weapons::default(),
            1,
            Name::from("Player"),
        ));

        test_app.world_mut().spawn(create_enemy_bundle(
            Transform::from_xyz(0.0, 0.0, 0.0),
            Weapons::default(),
            1,
            Name::from("Enemy"),
        ));

        // start app
        test_app.update();

        test_app
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
