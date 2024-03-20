use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;
pub use components::*;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Ball {
    pub player: u8,
    pub cooldown: f32,
    pub grounded: bool,
}

#[derive(Component)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
    pub zoom: f32,
}

#[derive(Component)]
pub struct Meter {
    pub max: f32,
}