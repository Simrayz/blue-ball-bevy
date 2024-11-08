use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct FaceInDirection {
    pub angle: f32,
}

#[derive(Component)]
pub struct MoveInFacingDirection {
    pub speed: f32,
}
