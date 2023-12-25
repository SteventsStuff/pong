use crate::states::AppState;
use crate::systems::{
    ball_collide_with_player, check_ball_out, confine_ball_movement, confine_player_movement,
    player_movement, setup, toggle_ball, update_ball_position,
};
use bevy::prelude::*;
use systems::handle_game_control_input;

mod bundles;
mod components;
mod constants;
mod states;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
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
                ball_collide_with_player,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, handle_game_control_input)
        .run();
}
