use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;

use crate::events::*;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
){
    if keyboard_input.just_pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>){
    for event in game_over_event_reader.read(){
        println!("Game Over!!! Score: {}", event.score);
        commands.insert_resource(NextState(Some(AppState::GAMEOVER)));
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
){
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.get() != &AppState::GAME{
            next_app_state.set(AppState::GAME);
            println!("GAME STATE");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
){
    if keyboard_input.just_pressed(KeyCode::M){
        if app_state.get() != &AppState::MAINMENU{
            next_app_state.set(AppState::MAINMENU);
            println!("MAIN MENU STATE");
        }
    }
}