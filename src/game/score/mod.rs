use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::GAME), insert_score)
        .init_resource::<HighScores>()
        .add_systems(Update, (
            update_score,
        ).run_if(in_state(AppState::GAME)).run_if(in_state(SimulationState::RUNNING)))
        .add_systems(Update, (
            update_high_scores,
            high_scores_updated
        ))
        .add_systems(OnExit(AppState::GAME), remove_score);
    }
}