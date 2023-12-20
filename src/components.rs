use bevy::prelude::*;

pub enum PlayerType {
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerSide(pub PlayerType);

#[derive(Component)]
pub struct Player {
    name: String,
    score: i8,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            score: 0,
        }
    }
}
