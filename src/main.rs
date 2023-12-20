use bevy::{prelude::*, window::PrimaryWindow};
use components::{Player, PlayerSide, PlayerType};
use constants::PLAYER_HEIGHT;

mod bundles;
mod components;
mod constants;

fn setup(mut commands: Commands) {
    // camers
    commands.spawn(Camera2dBundle::default());

    // players
    commands.spawn(bundles::PlayerBundle::new_left("Vova"));
    commands.spawn(bundles::PlayerBundle::new_right("Vova"));
}

fn confine_player_movement(
    mut players_q: Query<&mut Transform, With<Player>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window_height = window_q.get_single().unwrap().height();
    let window_y_min: f32 = window_height / -2.0 + PLAYER_HEIGHT / 2.0;
    let window_y_max: f32 = window_height / 2.0 - PLAYER_HEIGHT / 2.0;

    for mut player_transform in players_q.iter_mut() {
        if player_transform.translation.y >= window_y_max {
            player_transform.translation.y = window_y_max;
        }
        if player_transform.translation.y <= window_y_min {
            player_transform.translation.y = window_y_min;
        }
    }
}

fn player_movement(
    mut query: Query<(&PlayerSide, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // NOTE: I do not quite understand not booriwing works here yet
    let mut left_player_transform = None;
    let mut right_player_transform = None;
    for (player_side, player_transform) in query.iter_mut() {
        match player_side.0 {
            PlayerType::Left => {
                left_player_transform = Some(player_transform);
            }
            PlayerType::Right => {
                right_player_transform = Some(player_transform);
            }
        }
    }

    if keyboard_input.pressed(KeyCode::Up) {
        right_player_transform.unwrap().translation += Vec3::new(0.0, 1.0, 0.0) * 10.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        right_player_transform.unwrap().translation -= Vec3::new(0.0, 1.0, 0.0) * 10.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        left_player_transform.unwrap().translation += Vec3::new(0.0, 1.0, 0.0) * 10.0;
    } else if keyboard_input.pressed(KeyCode::S) {
        left_player_transform.unwrap().translation -= Vec3::new(0.0, 1.0, 0.0) * 10.0;
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (confine_player_movement, player_movement))
        .add_plugins(DefaultPlugins)
        .run();
}
