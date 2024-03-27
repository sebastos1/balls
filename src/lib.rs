use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod controls;
pub mod multiplayer;
pub mod components;
pub use components::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Matchmaking,
    InGame,
}

#[derive(Resource)]
pub struct GlobalCharge {
    pub charge: f32,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Player {
    pub turn: usize,
    pub charge: f32,
}

#[derive(Component)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
    pub zoom: f32,
}

#[derive(Component)]
pub struct Meter;