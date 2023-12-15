use bevy::prelude::*;

pub mod hud;
pub mod game_over_menu;

use game_over_menu::GameOverMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GameOverMenuPlugin);
    }
}