use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
){
    if keyboard_input.just_pressed(KeyCode::Space){
        match simulation_state.get() {
            SimulationState::PAUSED => {
                println!("Was paused");
                commands.insert_resource(NextState(Some(SimulationState::RUNNING)));
            }
            SimulationState::RUNNING => {
                println!("Was Running");
                commands.insert_resource(NextState(Some(SimulationState::PAUSED)));
            }
        }
    }
}