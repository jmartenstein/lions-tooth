use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CorePlugin,
            WorldPlugin,
            SimulationPlugin,
            CameraPlugin,
        ))
        .run();
}

/// Core functionality for state management and basic engine setup.
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Manages the grid, tile data, and procedural generation.
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}

/// The "Sleeping Heart": Handles non-deterministic systems like heat diffusion.
pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Camera controls and visualization.
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {}
}
