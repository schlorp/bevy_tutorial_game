use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>
){
    if keyboard_input.just_pressed(KeyCode::Space){
        match simulation_state.get() {
            SimulationState::PAUSED => {
                println!("Was paused");
                next_simulation_state.set(SimulationState::RUNNING);
            }
            SimulationState::RUNNING => {
                println!("Was Running");
                next_simulation_state.set(SimulationState::PAUSED);
            }
        }
    }
}