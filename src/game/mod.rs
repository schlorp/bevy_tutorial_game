use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugins((PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin))
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::GAME)));
    }
}

#[derive(Default, Debug, Clone, States, Hash, PartialEq, Eq)]
pub enum SimulationState{
    #[default]
    PAUSED,
    RUNNING,
}