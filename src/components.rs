use bevy::prelude::*;

pub enum PlayerType {
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
pub enum BallState {
    Idle,
    Moving,
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

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallStateComp {
    pub actions: BallState,
    pub direction: Vec3,
    pub speed: f32,
}
