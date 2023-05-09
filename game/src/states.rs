use bevy::prelude::*;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum GameState {
    #[default]
    InGame,
    Pause,
    Respawn,
}
