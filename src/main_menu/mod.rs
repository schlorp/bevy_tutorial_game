mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::layout::*;
use systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MAINMENU), spawn_main_menu)
        .add_systems(Update, (interact_with_play_button, interact_with_quit_button).run_if(in_state(AppState::MAINMENU)))
        .add_systems(OnExit(AppState::MAINMENU), despawn_main_menu);
    }
}