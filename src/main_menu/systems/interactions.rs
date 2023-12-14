use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AppState;

use crate::main_menu::components::*;
use crate::main_menu::styles::{NORMAL_BUTTON_COLOR, HOVER_BUTTON_COLOR, PRESSED_BUTTON_COLOR};


pub fn interact_with_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>
){
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::GAME);
            }
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>
){
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }   
}