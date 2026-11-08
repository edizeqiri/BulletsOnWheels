use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

// Slop: Generated with Mistral 4 Large,

// TODO(Slop): Add env to for this
const CONFIG_PATH: &str = "physics_config.json";

#[derive(Resource, Serialize, Deserialize)]
#[serde(default)]
pub struct PhysicsTuning {
    // --- Character ---
    pub movement_acceleration: f32,
    pub movement_damping: f32,
    pub jump_impulse: f32,
    pub max_slope_angle: f32,
    pub max_acceleration: f32,
    // --- World ---
    pub gravity: f32,
    pub ground_cast_distance: f32,
    pub collision_margin: f32,
}

impl Default for PhysicsTuning {
    fn default() -> Self {
        let acc = 300.0;
        Self {
            movement_acceleration: acc,
            movement_damping: 2.5,
            jump_impulse: 80.0,
            max_slope_angle: core::f32::consts::PI * 0.45,
            max_acceleration: acc * 1.5,
            gravity: 250.0,
            ground_cast_distance: 10.0,
            collision_margin: 0.3,
        }
    }
}

pub struct PhysicsConfigUI;

impl Plugin for PhysicsConfigUI {
    fn build(&self, app: &mut App) {
        // Load saved config if a file exists, otherwise fall back to defaults.
        let tuning = load_config().unwrap_or_default();
        app.insert_resource(tuning);

        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, display)
            .add_systems(Update, apply_world_params);
    }
}

fn save_config(tuning: &PhysicsTuning) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tuning)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(CONFIG_PATH, json)
}

fn load_config() -> std::io::Result<PhysicsTuning> {
    let json = std::fs::read_to_string(CONFIG_PATH)?;
    serde_json::from_str(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn param_slider(
    ui: &mut egui::Ui,
    label: &str,
    description: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) {
    ui.label(label);
    ui.add(egui::Slider::new(value, range).step_by(0.1));
    ui.label(egui::RichText::new(description).small().weak());
    ui.add_space(4.0);
}

fn display(mut contexts: EguiContexts, mut tuning: ResMut<PhysicsTuning>) -> Result {
    egui::Window::new("Physics Configs").show(contexts.ctx_mut()?, |ui| {
        egui::CollapsingHeader::new("Character")
            .default_open(true)
            .show(ui, |ui| {
                param_slider(
                    ui,
                    "Movement Acceleration",
                    "How quickly the character reaches full horizontal speed.",
                    &mut tuning.movement_acceleration,
                    0.0..=2000.0,
                );
                param_slider(
                    ui,
                    "Movement Damping",
                    "How quickly horizontal movement decays when no input is given.",
                    &mut tuning.movement_damping,
                    0.0..=20.0,
                );
                param_slider(
                    ui,
                    "Jump Impulse",
                    "Vertical velocity applied on jump — controls jump height.",
                    &mut tuning.jump_impulse,
                    0.0..=500.0,
                );
                param_slider(
                    ui,
                    "Max Slope Angle",
                    "Steepest slope (in radians) the character can walk up without sliding.",
                    &mut tuning.max_slope_angle,
                    0.0..=core::f32::consts::FRAC_PI_2,
                );
                param_slider(
                    ui,
                    "Max Acceleration",
                    "Hard cap on acceleration applied per frame.",
                    &mut tuning.max_acceleration,
                    0.0..=5000.0,
                );
            });

        egui::CollapsingHeader::new("World")
            .default_open(true)
            .show(ui, |ui| {
                param_slider(
                    ui,
                    "Gravity",
                    "Downward acceleration applied to all dynamic bodies.",
                    &mut tuning.gravity,
                    0.0..=2000.0,
                );
                param_slider(
                    ui,
                    "Ground Cast Distance",
                    "How far below the character the ground probe reaches.",
                    &mut tuning.ground_cast_distance,
                    0.0..=100.0,
                );
                param_slider(
                    ui,
                    "Collision Margin",
                    "Skin thickness around colliders to prevent tunneling.",
                    &mut tuning.collision_margin,
                    0.0..=5.0,
                );
            });

        ui.separator();

        let mut status = String::new();
        ui.horizontal(|ui| {
            if ui.button("Save to file").clicked() {
                status = match save_config(&tuning) {
                    Ok(()) => format!("Saved to {CONFIG_PATH}"),
                    Err(e) => format!("Save failed: {e}"),
                };
            }
            if ui.button("Reload from file").clicked() {
                status = match load_config() {
                    Ok(loaded) => {
                        *tuning = loaded;
                        format!("Loaded from {CONFIG_PATH}")
                    },
                    Err(e) => format!("Reload failed: {e}"),
                };
            }
        });
        if !status.is_empty() {
            ui.label(egui::RichText::new(status).small().weak());
        }
    });
    Ok(())
}

/// TODO(Slop): Check correctness
/// Propagates world-level tuning values to the actual avian2d components
/// and resources every frame so changes take effect immediately.
fn apply_world_params(
    tuning: Res<PhysicsTuning>,
    mut gravity: ResMut<Gravity>,
    mut casters: Query<&mut ShapeCaster>,
    mut margins: Query<&mut CollisionMargin>,
) {
    if tuning.is_changed() {
        gravity.0 = Vec2::NEG_Y * tuning.gravity;
        for mut caster in &mut casters {
            caster.max_distance = tuning.ground_cast_distance;
        }
        for mut margin in &mut margins {
            margin.0 = tuning.collision_margin;
        }
    }
}
