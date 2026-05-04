use crate::world::map::map::{Level, Map};
use crate::world::map::simple_city_map::SimpleCityMap;
use crate::world::map::walls::create_wall_bundle;
use crate::world::LevelState;
use bevy::app::App;
use bevy::prelude::{Commands, Name, OnEnter, Transform, Visibility};
use glam::Vec2;

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::TWO), generate_level2_system);
}

// TODO: Refactor since DRY
fn generate_level2_system(mut command: Commands) {

    let mut map = SimpleCityMap::default();

    let mut start = Vec2::new(100., 100.);

    command
        .spawn((
            Name::new("Level 2"),
            Level(LevelState::ONE),
            Visibility::default(),
            Transform::default()
        ))
        .with_children(|cmd| {
            map.add_path(start, 100.);
            let paths = map.get_paths().clone(); // clone needed, because of map ownership
            cmd.spawn((
                Name::new("SimpleCityMap"),
                map,
                Transform::default(),
                Visibility::default()
            ))
                .with_children(|cmd| {
                    paths.iter().for_each(|path| {
                        path.points.iter().for_each(|vertice| {
                            cmd.spawn(create_wall_bundle(Transform::from_xyz(
                                vertice.x, vertice.y, 0.
                            )));
                        });
                    });
                });
        });

}