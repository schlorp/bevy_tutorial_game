use bevy::prelude::*;

use crate::AppState;

pub mod systems;
pub mod components;
pub mod styles;

use systems::layout::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::GAMEOVER), spawn_game_over_menu)
        .add_systems(OnExit(AppState::GAMEOVER), despawn_game_over_menu);
    }
}