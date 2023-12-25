use bevy::ecs::schedule::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    InGame,
    Paused,
}
