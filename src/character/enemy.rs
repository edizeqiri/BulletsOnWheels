use crate::character;
use crate::character::{ENEMY_GROUP, Health, PLAYER_GROUP, square_sprite};
use crate::weapon::Weapons;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<EnemyDeathMessage>()
        .add_systems(Update, check_enemy_zero_health_system)
        .add_systems(Update, handle_enemy_zero_health_system);
}

#[derive(Component)]
pub struct Enemy;

#[derive(Message)]
pub struct EnemyDeathMessage {
    pub entity: Entity,
}

pub fn create_enemy_bundle(
    transform: Transform,
    weapons: Weapons,
    max_health: u32,
    name: Name,
) -> impl Bundle {
    (
        name,
        character::create_character(transform, weapons.clone(), max_health),
        CollisionGroups::new(ENEMY_GROUP, PLAYER_GROUP),
        Enemy,
        square_sprite(Color::Srgba(RED)),
    )
}

fn check_enemy_zero_health_system(
    mut death_message: MessageWriter<EnemyDeathMessage>,
    query: Query<(&Health, Entity), (With<Enemy>, Changed<Health>)>,
) {
    for (health, entity) in &query {
        if health.current <= 0 {
            death_message.write(EnemyDeathMessage { entity });
        }
    }
}

fn handle_enemy_zero_health_system(
    mut commands: Commands,
    mut enemy_death_messages: MessageReader<EnemyDeathMessage>,
) {
    for message in enemy_death_messages.read() {
        debug!("Enemy died!");
        commands.entity(message.entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use crate::character::enemy::{Enemy, create_enemy_bundle};
    use crate::character::{Health, enemy};
    use crate::weapon::Weapons;
    use bevy::app::App;
    use bevy::prelude::{Entity, Name, Transform, With};

    // ----------- SETUP ----------- //
    pub struct Setup {
        pub app: App,
        pub player: Entity,
    }

    impl Setup {
        pub fn new() -> Self {
            // setup App
            let mut app = App::new();

            app.add_plugins(enemy::plugin);

            // setup Entities
            let player = app
                .world_mut()
                .spawn(create_enemy_bundle(
                    Transform::from_xyz(1.0, 1.0, 0.0),
                    Weapons::default(),
                    1,
                    Name::from("Player"),
                ))
                .id();

            // start app
            app.update();

            // confirm setup: there is 1 Player
            let world = app.world_mut();
            let player = world
                .query_filtered::<Entity, With<Enemy>>()
                .single(world)
                .expect("Expected exactly one Player entity");

            Self { app, player }
        }
    }

    // ----------- TEST ----------- //

    #[test]
    fn zero_health_enemy_is_despawned() {
        let mut setup = Setup::new();

        {
            let world = setup.app.world_mut();

            // when: enemy.health.current = 0
            world
                .query::<&mut Health>()
                .get_mut(world, setup.player)
                .unwrap()
                .current = 0;
        }

        // 1. message: health changes
        setup.app.update();
        // 2. message: enemy died
        setup.app.update();

        // then: no enemy exists
        let world = setup.app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<Enemy>>()
                .iter(&world)
                .len(),
            0
        );
    }
}
