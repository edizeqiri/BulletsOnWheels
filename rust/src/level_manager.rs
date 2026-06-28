use bevy::prelude::*;
use godot::classes::Node;
use godot::prelude::*;
use godot_bevy::interop::signal_names::SceneTreeSignals;
use godot_bevy::plugins::scene_tree::{SceneTreeMessage, SceneTreeMessageType};
use godot_bevy::prelude::*;

/// FYI: This code comes from godot_bevy.

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .init_resource::<PendingLevel>()
            .init_resource::<LevelLoadingState>()
            .init_resource::<SceneTreeSignalConnected>()
            // Enable signal routing for SceneTree.scene_changed
            .add_plugins(GodotSignalsPlugin::<SceneChanged>::default())
            .add_observer(on_load_level_request)
            .add_observer(level_state_change_system)
            .add_observer(on_scene_changed)
            .add_observer(change_state_system_on_loaded_level)
            .add_systems(Startup, connect_scene_tree_signal)
            .add_systems(
                Update,
                (
                    (handle_level_scene_change, ApplyDeferred).chain(),
                    emit_level_loaded_event_when_scene_ready
                )
            );
    }
}

/// Event fired when the Godot scene changes.
/// This demonstrates using `connect_object` to listen to singleton signals.
#[derive(Event, Debug, Clone)]
struct SceneChanged;

/// Simple level identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, GodotConvert, Var, Export, States)]
#[godot(via = GString)]
pub enum LevelId {
    #[default]
    MainMenu,
    Level0,
    Level1
}

impl LevelId {
    /// Get the Godot scene path for this level
    pub fn scene_path(&self) -> &'static str {
        match self {
            LevelId::Level0 => "scenes/levels/level0.tscn",
            LevelId::Level1 => "scenes/levels/level1.tscn",
            LevelId::MainMenu => "scenes/main_menu/main_menu.tscn"
        }
    }

    /// Get the path of the level's root node once spawned in the scene tree
    pub fn root_node_path(&self) -> &'static str {
        match self {
            LevelId::Level0 => "/root/Level0",
            LevelId::Level1 => "/root/Level1",
            LevelId::MainMenu => "/root/MainMenu"
        }
    }
}

/// Resource that tracks the current active level (read-mostly for game state)
#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub level_id: LevelId,
    /// Entity holding the spawned `GodotScene`/`GodotNodeHandle` for the
    /// current level
    pub entity: Option<Entity>
}

impl CurrentLevel {
    /// Set the current level
    pub fn set(&mut self, level_id: LevelId) {
        self.level_id = level_id;
    }
}

/// Resource for tracking level loading state (internal to level manager)
#[derive(Resource, Default)]
struct LevelLoadingState {
    pub loading_handle: Option<Handle<GodotResource>>,
    /// Whether the initial menu scene (Godot's main scene) has been torn down.
    /// It isn't tracked as a Bevy entity, so we free it once on the first load.
    pub menu_cleared: bool
}

/// Resource that tracks the pending level
#[derive(Resource, Default)]
pub struct PendingLevel {
    pub level_id: Option<LevelId>
}

/// Event fired when a level load is requested
#[derive(Event, Debug, Clone)]
pub struct LoadLevelMessage {
    pub level_id: LevelId
}

/// Event fired when level loading is complete
#[derive(Event, Debug, Clone)]
pub struct LevelLoadedEvent {
    pub level_id: LevelId
}

pub struct LevelManagerPlugin;

/// Tracks whether we've connected to the SceneTree signal
#[derive(Resource, Default)]
struct SceneTreeSignalConnected(bool);

/// Connect to the SceneTree's scene_changed signal.
/// This demonstrates using `connect_object` for non-entity signals.
fn connect_scene_tree_signal(
    mut connected: ResMut<SceneTreeSignalConnected>,
    signals: GodotSignals<SceneChanged>,
    mut scene_tree: SceneTreeRef
) {
    if connected.0 {
        return;
    }

    let tree = scene_tree.get().clone();
    signals.connect_object(tree, SceneTreeSignals::SCENE_CHANGED, |_args| {
        Some(SceneChanged)
    });
    connected.0 = true;

    // info!("Connected to SceneTree.scene_changed signal");
}

/// Observer that logs when a scene change occurs
fn on_scene_changed(_trigger: On<SceneChanged>) {
    // info!("Scene changed!");
}

fn level_state_change_system(trigger: On<LoadLevelMessage>, mut commands: Commands) {
    commands.set_state(trigger.event().level_id);
}

/// Observer that handles level loading requests - loads the asset
fn on_load_level_request(
    trigger: On<LoadLevelMessage>,
    mut loading_state: ResMut<LevelLoadingState>,
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>
) {
    let event = trigger.event();
    info!("Loading level asset: {:?}", event.level_id);

    // Load the level scene through Bevy's asset system
    let level_handle: Handle<GodotResource> = asset_server.load(event.level_id.scene_path());

    // Track loading state separately from current level
    loading_state.loading_handle = Some(level_handle);

    // Update current level
    current_level.set(event.level_id);

    info!("Level asset loading started for: {:?}", event.level_id);
}

/// System that handles actual scene changing once assets are loaded
fn handle_level_scene_change(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    mut loading_state: ResMut<LevelLoadingState>,
    mut pending_level: ResMut<PendingLevel>,
    mut assets: ResMut<Assets<GodotResource>>,
    mut scene_tree: SceneTreeRef,
    mut godot: GodotAccess,
    godot_nodes: Query<(Entity, &GodotNodeHandle)>
) {
    let level_id = current_level.level_id;
    let Some(handle) = loading_state.loading_handle.clone() else {
        return;
    };

    // Check if the asset is loaded
    if assets.get_mut(&handle).is_none() {
        // If asset isn't loaded yet, we'll try again next frame
        return;
    }

    info!("Spawning level scene: {:?}", level_id);

    // Despawn the previously active level's scene root, if any
    if let Some(old_entity) = current_level.entity.take() {
        commands
            .entity(old_entity)
            .queue_silenced(|e: EntityWorldMut| {
                e.despawn();
            });
    }

    // Tear down the initial menu scene the first time we load a level.
    // It's Godot's main scene (not a Bevy entity), so nothing else frees it;
    // leaving it in the tree overlaps the level and keeps its shootable
    // buttons live, re-triggering LoadLevelMessage.

    if !loading_state.menu_cleared {
        if let Some(mut menu) = scene_tree.get().get_current_scene() {
            let menu_path = menu.get_path().to_string();

            for (entity, handle) in &godot_nodes {
                let Some(node) = godot.try_get::<Node>(*handle) else {
                    continue;
                };

                if node.get_path().to_string().starts_with(&menu_path) {
                    commands.entity(entity).queue_silenced(|e: EntityWorldMut| {
                        e.despawn();
                    });
                }
            }

            menu.queue_free();
        }
        loading_state.menu_cleared = true;
    }

    // Spawn the new level scene at the scene tree root
    let entity = commands.spawn(GodotScene::from_handle(handle)).id();
    current_level.entity = Some(entity);

    // Do NOT emit LevelLoadedMessage here!
    pending_level.level_id = Some(level_id);

    // info!("Level scene spawn requested for: {:?}", level_id);

    // Clear the loading handle since we've used it
    loading_state.loading_handle = None;
}

fn emit_level_loaded_event_when_scene_ready(
    mut pending_level: ResMut<PendingLevel>,
    mut scene_tree_events: MessageReader<SceneTreeMessage>,
    mut commands: Commands,
    mut godot: GodotAccess
) {
    let Some(level_id) = pending_level.level_id else {
        return;
    };

    let expected_path = level_id.root_node_path();
    for event in scene_tree_events.read() {
        if let SceneTreeMessageType::NodeAdded = event.message_type
            && let Some(node) = godot.try_get::<Node>(event.node_id)
            && node.is_inside_tree()
        {
            let node_path = node.get_path().to_string();
            if node_path == expected_path {
                commands.trigger(LevelLoadedEvent { level_id });
                pending_level.level_id = None;
                break;
            }
        }
    }
}

fn change_state_system_on_loaded_level(event: On<LevelLoadedEvent>, mut commands: Commands) {
    let trigger = event.event();
    commands.set_state(trigger.level_id);
    info!("current level state: {:?}", trigger.level_id);
}

#[derive(Component, GodotNode)]
#[godot_node(base(Node2D), class_name(RScore))]
pub struct Score {
    pub count: i32,
    pub highscore: i32,
    pub is_new_highscore: bool
}

impl Default for Score {
    fn default() -> Self {
        Self {
            count: 0,
            highscore: -1,
            is_new_highscore: false
        }
    }
}
