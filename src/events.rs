use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

impl Default for GameOver {
    fn default() -> Self {
        Self { score: 0 }
    }
}
