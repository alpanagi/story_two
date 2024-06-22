use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Instructions,
    Playing,
    Won,
}
