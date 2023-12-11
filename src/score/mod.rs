use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<EnemyAmount>()
        .add_systems(Update, (
            update_score,
            update_enemy_amount,
            update_high_scores,
            high_scores_updated
        ).chain());
    }
}