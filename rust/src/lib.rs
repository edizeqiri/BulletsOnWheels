use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;
mod character;
//mod gamestate;
mod input;
mod level_manager;

mod weapon;

#[bevy_app]
fn build_app(app: &mut App) {
    // GodotDefaultPlugins provides all standard godot-bevy functionality
    // For minimal setup, use individual plugins instead:
    // app.add_plugins(GodotTransformSyncPlugin)
    //     .add_plugins(GodotAudioPlugin)
    //     .add_plugins(BevyInputBridgePlugin);
    app
        .add_plugins(GodotDefaultPlugins)
        .add_plugins(weapon::plugin)
        .add_plugins(character::plugin)
        //.add_plugins(gamestate::plugin)
        .add_plugins(input::plugin)
        .add_plugins(level_manager::LevelManagerPlugin);

}

fn hello_world_system(mut timer: Local<f32>, time: Res<Time>) {
    // This runs every frame in Bevy's Update schedule
    *timer += time.delta_secs();
    if *timer > 1.0 {
        *timer = 0.0;
        godot_print!("Hello from Bevy ECS!");
    }
}
