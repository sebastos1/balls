use crate::*;
use bevy::utils::HashMap;
use bevy_ggrs::{LocalInputs, LocalPlayers};
use crate::multiplayer::{Config, InputData};

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Player>,
    local_players: Res<LocalPlayers>,
    camera: Query<&Transform, (With<Camera>, Without<Player>)>,
) {
    let mut local_inputs = HashMap::new();
    let camera_transform = camera.single();
    let camera_forward = Vec3::new(
        camera_transform.forward().x, 0., camera_transform.forward().z
    ).normalize();

    for handle in &local_players.0 {
        let mut input = InputData { x: 0., y: 0., power: 0. };
        let mut direction = Vec3::ZERO;

        let mut player = players.single_mut();
        if *handle == player.turn {
            if keys.pressed(KeyCode::Space) {
                if player.charge < 20. {
                    player.charge += 0.5;
                }
                input.power = player.charge;
                info!("charging: {}", player.charge);
            } else {
                if player.charge > 0. {
                    direction += camera_forward;
                    if direction.length() != 0. {
                        direction = direction.normalize();
                        input.x = direction.x;
                        input.y = direction.z;
                        input.power = player.charge;
                    }
                    player.charge = 0.;
                }
            }
        }
        local_inputs.insert(*handle, input);
    }
    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}
