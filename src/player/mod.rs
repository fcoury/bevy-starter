use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_hit_star);
    }
}
