use crate::systems::{
    check_ball_out, confine_ball_movement, confine_player_movement, player_movement, setup,
    toggle_ball, update_ball_position,
};
use bevy::prelude::*;

mod bundles;
mod components;
mod constants;
mod systems;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                confine_player_movement,
                player_movement,
                toggle_ball,
                confine_ball_movement,
                update_ball_position,
                check_ball_out,
            ),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
