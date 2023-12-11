use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet{
    MOVEMENT,
    CONFINEMENT,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .configure_sets(Startup, PlayerSystemSet::MOVEMENT.before(PlayerSystemSet::CONFINEMENT))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            player_movement.in_set(PlayerSystemSet::MOVEMENT), 
            confine_player_movement.in_set(PlayerSystemSet::CONFINEMENT),
            enemy_hit_player, 
            player_hit_star,
        ));
    }
}