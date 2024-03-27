use crate::*;
use bevy_ggrs::*;
use bevy::core::Pod;
use bytemuck::Zeroable;
use matchbox_socket::PeerId;
use bevy_matchbox::MatchboxSocket;
use bevy_matchbox::prelude::SingleChannel;

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct InputData {
    pub x: f32,
    pub y: f32,
    pub power: f32,
}

pub type Config = bevy_ggrs::GgrsConfig<InputData, PeerId>;

pub fn read_inputs(
    inputs: Res<PlayerInputs<Config>>,
    mut global_charge: ResMut<GlobalCharge>,
    mut players: Query<(&mut Velocity, &mut Player)>,
) {
    let (mut velocity, mut player) = players.single_mut();
    let (input, _) = inputs.get(player.turn).unwrap();
    if input.x != 0. || input.y != 0. {
        info!("this is input {:?}", input);
        let speed = input.power * 3.;
        let dir = Vec3::new(input.x, 0., input.y);

        if dir.length() != 0. {
            velocity.linvel += dir * speed;

            if player.turn == 0 {
                player.turn = 1;
            } else {
                player.turn = 0;
            }
        }
    }
    global_charge.charge = input.power;
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