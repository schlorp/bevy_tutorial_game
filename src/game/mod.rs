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

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugins((PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum SimulationState{
    #[default]
    PAUSED,
    RUNNING,
}