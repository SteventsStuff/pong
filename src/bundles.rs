use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::{Ball, BallState, BallStateComp, Player, PlayerSide, PlayerType};
use crate::constants::{BALL_SIZE, PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_X_OFFSET};

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    player_side: PlayerSide,

    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new_left(name: &str) -> Self {
        Self::new(name, PlayerType::Left)
    }

    pub fn new_right(name: &str) -> Self {
        Self::new(name, PlayerType::Right)
    }

    fn new(name: &str, player_side: PlayerType) -> Self {
        let x_offset = match player_side {
            PlayerType::Left => -PLAYER_X_OFFSET,
            PlayerType::Right => PLAYER_X_OFFSET,
        };
        PlayerBundle {
            player: Player::new(name),
            player_side: PlayerSide(player_side),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x_offset, 0.0, 0.0)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    ball_state: BallStateComp,

    material_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl BallBundle {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        BallBundle {
            ball: Ball,
            ball_state: BallStateComp {
                actions: BallState::Idle,
                direction: Vec3::new(0.0, 0.0, 0.0),
                speed: 0.0,
            },

            material_2d_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            },
        }
    }
}
