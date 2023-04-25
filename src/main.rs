use bevy::prelude::*;

mod events;
mod systems;

mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
