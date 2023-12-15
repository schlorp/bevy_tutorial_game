use bevy::prelude::*;

pub mod hud;

use hud::systems::layout::*;

use crate::AppState;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::GAME), spawn_hud)
        .add_systems(OnExit(AppState::GAME), despawn_hud);
    }
}