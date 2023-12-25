use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    bundles,
    components::{Ball, BallState, BallStateComp, Player, PlayerSide, PlayerType},
    constants::{BALL_SIZE, BALL_SPEED, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_VECTOR_OFFSET},
};

pub fn setup(
    mut commands: Commands,
    mashes: ResMut<Assets<Mesh>>,
    matarials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // players
    commands.spawn(bundles::PlayerBundle::new_left("Vova"));
    commands.spawn(bundles::PlayerBundle::new_right("Vova"));

    // ball
    commands.spawn(bundles::BallBundle::new(mashes, matarials));
}

pub fn confine_player_movement(
    mut players_q: Query<&mut Transform, With<Player>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window_height: f32 = window_q.get_single().unwrap().height();
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

pub fn confine_ball_movement(
    mut ball_q: Query<(&Transform, &mut BallStateComp), With<Ball>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window_height: f32 = window_q.get_single().unwrap().height();

    let window_y_min: f32 = window_height / -2.0 + BALL_SIZE / 2.0;
    let window_y_max: f32 = window_height / 2.0 - BALL_SIZE;

    for (ball_transform, mut balls_state) in ball_q.iter_mut() {
        // println!(
        //     "ball Y: {}, ball X: {}, win_x_min: {}, win_x_max: {}",
        //     ball_transform.translation.y, ball_transform.translation.x, window_y_min, window_y_max
        // );
        if ball_transform.translation.y >= window_y_max
            || ball_transform.translation.y <= window_y_min
        {
            balls_state.direction.y = -1.0 * balls_state.direction.y;
        }
    }
}

pub fn check_ball_out(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut ball_q: Query<(&mut Transform, &mut BallStateComp), With<Ball>>,
) {
    let window = window_q.get_single().unwrap();
    let window_min_left = window.width() / -2.0;
    let window_max_right = window.width() / 2.0;

    let (mut ball_transform, mut ball_state) = ball_q.get_single_mut().unwrap();
    if ball_transform.translation.x > window_max_right {
        reset_ball(&mut ball_transform, &mut ball_state);
    } else if ball_transform.translation.x < window_min_left {
        reset_ball(&mut ball_transform, &mut ball_state);
    }
}

pub fn reset_ball(ball_transform: &mut Transform, ball_state: &mut BallStateComp) {
    ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
    ball_state.actions = BallState::Idle;
    ball_state.direction = Vec3::new(0.0, 0.0, 0.0);
    ball_state.speed = 0.0;
}

pub fn toggle_ball(mut ball_q: Query<&mut BallStateComp>, keyboard_input: Res<Input<KeyCode>>) {
    let mut ball_state = ball_q.single_mut();
    // println!(
    //     "{:?}, {}, {}",
    //     ball_state.actions, ball_state.direction, ball_state.speed
    // );
    if ball_state.actions == BallState::Idle && keyboard_input.pressed(KeyCode::Space) {
        ball_state.actions = BallState::Moving;
        ball_state.direction = get_init_ball_direction();
        ball_state.speed = BALL_SPEED;
    }
}

pub fn get_init_ball_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-0.5..=1.0);
    let y: f32 = rng.gen_range(-0.5..=1.0);
    println!("{}, {}", x, y);
    Vec3::new(x, y, 0.0).normalize()
}

pub fn update_ball_position(
    mut ball_q: Query<(&mut Transform, &BallStateComp), With<Ball>>,
    time: Res<Time>,
) {
    let (mut ball_transform, ball_state) = ball_q.single_mut();
    ball_transform.translation += ball_state.direction * BALL_SPEED * time.delta_seconds();
}

pub fn player_movement(
    time: Res<Time>,
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

    let player_offset: Vec3 = PLAYER_VECTOR_OFFSET * PLAYER_SPEED * time.delta_seconds();
    if keyboard_input.pressed(KeyCode::Up) {
        right_player_transform.unwrap().translation += player_offset;
    } else if keyboard_input.pressed(KeyCode::Down) {
        right_player_transform.unwrap().translation -= player_offset;
    }

    if keyboard_input.pressed(KeyCode::W) {
        left_player_transform.unwrap().translation += player_offset;
    } else if keyboard_input.pressed(KeyCode::S) {
        left_player_transform.unwrap().translation -= player_offset;
    }
}

fn ball_collide_with_player(
    mut ball_q: Query<(&Transform, &mut BallStateComp), With<Ball>>,
    mut players_q: Query<&mut Transform, With<Player>>,
) {
    
}
