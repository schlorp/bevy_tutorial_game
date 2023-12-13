use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app
        //startup
        .init_resource::<EnemySpawnTimer>()
        //on enter game state
        .add_systems(OnEnter(AppState::GAME), spawn_enemies)
        //update
        .add_systems(Update, (
            enemy_movement,
            update_enemy_direction,
            confine_enemy_movement, 
        ).run_if(in_state(AppState::GAME)).run_if(in_state(SimulationState::RUNNING)).chain())
        .add_systems(Update, (
            tick_enemy_spawn_timer,
            spawn_enemies_over_time,
        ).run_if(in_state(AppState::GAME)).run_if(in_state(SimulationState::RUNNING)))
        //on exit game state
        .add_systems(OnExit(AppState::GAME), despawn_enemies);
    }
}