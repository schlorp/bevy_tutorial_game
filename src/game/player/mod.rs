use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet{
    MOVEMENT,
    CONFINEMENT,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .configure_sets(Update, PlayerSystemSet::MOVEMENT.before(PlayerSystemSet::CONFINEMENT).run_if(in_state(AppState::GAME)).run_if(in_state(SimulationState::RUNNING)))
        .add_systems(OnEnter(AppState::GAME), spawn_player)
        .add_systems(Update, (
            player_movement.in_set(PlayerSystemSet::MOVEMENT), 
            confine_player_movement.in_set(PlayerSystemSet::CONFINEMENT),
            enemy_hit_player, 
            player_hit_star,
        ).run_if(in_state(AppState::GAME)).run_if(in_state(SimulationState::RUNNING)))
        .add_systems(OnExit(AppState::GAME), despawn_player);
    }
}