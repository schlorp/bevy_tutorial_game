use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn insert_score(mut commands: Commands){
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands){
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highscores: ResMut<HighScores>
){
    for event in game_over_event_reader.read(){
        highscores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(
    highscores: Res<HighScores>
){
    if highscores.is_changed(){
        println!("HighScores: {:?}", highscores.scores);
    }
}