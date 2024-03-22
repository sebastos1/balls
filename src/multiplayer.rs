use crate::*;
use bevy_ggrs::*;
use bevy::core::Pod;
use bytemuck::Zeroable;
use matchbox_socket::PeerId;
use bevy_matchbox::MatchboxSocket;
use bevy_matchbox::prelude::SingleChannel;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct InputData {
    pub x: f32,
    pub y: f32,
}

pub type Config = bevy_ggrs::GgrsConfig<InputData, PeerId>;

pub fn move_players(
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in players.iter_mut() {
        if let Some((input, _)) = inputs.get(player.handle) {
            let speed = 0.1;
            let dir = Vec3::new(input.x, 0., input.y);

            if dir.length() != 0. {
                velocity.linvel += dir * speed;
            }
        }
    }
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/pool?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut commands: Commands, 
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    
    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    next_state.set(GameState::InGame);
}