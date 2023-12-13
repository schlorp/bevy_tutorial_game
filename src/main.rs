pub mod events;
mod systems;
mod main_menu;
mod game;

use main_menu::*;
use game::*;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugins((GamePlugin, MainMenuPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, (
        exit_game,
        handle_game_over,
        transition_to_game_state,
        transition_to_main_menu_state
    )).run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState{
    #[default]
    MAINMENU,
    GAME,
    GAMEOVER,
}