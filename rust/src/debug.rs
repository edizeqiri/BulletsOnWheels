use bevy::prelude::*;
use godot::classes::{Input, Node};
use godot::global::Key;
use godot::prelude::*;
use godot_bevy::prelude::{GodotAccess, SceneTreeRef};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, log_scene_tree_on_keypress);
}

fn log_scene_tree_on_keypress(
    mut scene_tree: SceneTreeRef,
    mut godot: GodotAccess,
    mut was_pressed: Local<bool>
) {
    let gd_input = godot.singleton::<Input>();
    let pressed = gd_input.is_key_pressed(Key::H);

    if pressed && !*was_pressed {
        if let Some(root) = scene_tree.get().get_root() {
            info!("=== Scene Tree ===");
            log_node(&root.upcast::<Node>());
            info!("==================");
        }
    }

    *was_pressed = pressed;
}

fn log_node(node: &Gd<Node>) {
    info!("{} [{}]", node.get_path(), node.get_class());
    for child in node.get_children().iter_shared() {
        log_node(&child);
    }
}
