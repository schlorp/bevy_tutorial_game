pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;
//only chain what is needed to safe performance, dont chain everything

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin))
    .add_event::<GameOver>()
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, (
        exit_game,
        handle_game_over
        )).run();
}