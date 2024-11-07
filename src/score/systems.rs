use crate::events::*;
use bevy::prelude::*;

use super::resources::{HighScores, Score};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut high_scores: ResMut<HighScores>,
    mut event_reader: EventReader<GameOver>,
) {
    for event in event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores);
    }
}
