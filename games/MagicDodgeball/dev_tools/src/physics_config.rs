use bevy::prelude::*;

use bevy_egui::{EguiContext, EguiContexts, egui};
#[cfg(debug_assertions)]
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

pub struct PhysicsConfigUI;

impl Plugin for PhysicsConfigUI {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin::default())
             .add_systems(EguiPrimaryContextPass, display);
    } 
}

fn display(mut commands: Commands, mut contexts: EguiContexts) -> Result {
    egui::Window::new("Physics Configs").show(contexts.ctx_mut()?, 
        | ui | {
            ui.label("world")
        });
    Ok(())
}