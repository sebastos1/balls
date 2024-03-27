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

// #[derive(Resource)]
// pub struct SyncResources {
//     rng_seed: u64,
// }

#[derive(Resource)]
pub struct GlobalCharge {
    pub charge: f32,
}

pub enum BallType {
    Striped,
    Solid,
    Cue,
    Eight,
}

#[derive(Component)]
pub struct Ball {
    pub number: u8,
    pub color: Color,
    pub ball_type: BallType,
}

impl Ball {
    pub fn new(number: u8) -> Self {
        Ball {
            number,

            ball_type: match number {
                0 => BallType::Cue,
                8 => BallType::Eight,
                1..=7 => BallType::Solid,
                9..=15 => BallType::Striped,
                _ => unreachable!(),
            },

            color: match number {
                0 => Color::WHITE,
                1 | 9 => Color::YELLOW,
                2 | 10 => Color::BLUE,
                3 | 11 => Color::RED,
                4 | 12 => Color::PURPLE,
                5 | 13 => Color::ORANGE,
                6 | 14 => Color::GREEN,
                7 | 15 => Color::MAROON,
                8 => Color::BLACK,
                _ => panic!("Invalid ball number: {}", number),
            },
        }
    }
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