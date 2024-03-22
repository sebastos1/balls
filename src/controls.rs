use crate::*;
use bevy::utils::HashMap;
use bevy_ggrs::{LocalInputs, LocalPlayers};
use crate::multiplayer::{Config, InputData};

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    camera: Query<&Transform, (With<Camera>, Without<Player>)>,
) {
    let mut local_inputs = HashMap::new();
    let mut input = InputData { x: 0.0, y: 0.0, };
    let mut direction = Vec3::ZERO;

    for handle in &local_players.0 {
        for player in players.iter() {
            let camera_transform = camera.single();
            let camera_forward = Vec3::new(camera_transform.forward().x, 0., camera_transform.forward().z).normalize();
            let camera_right = Vec3::new(camera_transform.right().x, 0., camera_transform.right().z).normalize();
        
            if player.grounded  {
                if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
                    direction += camera_forward;
                }
                if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
                    direction -= camera_forward;
                }
                if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
                    direction -= camera_right;
                }
                if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
                    direction += camera_right;
                }
            }

            if direction.length() > 0. {
                direction = direction.normalize();

                input.x = direction.x;
                input.y = direction.z;
            }
        }
        local_inputs.insert(*handle, input);
    }
    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

pub fn ability(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity)>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
) {
    let camera_transform = camera_query.single();
    let camera_forward_flat = Vec3::new(camera_transform.forward().x, 0.0, camera_transform.forward().z).normalize();

    for (mut ball, mut velocity) in query.iter_mut() {
        if ball.cooldown > 0.0 {
            ball.cooldown -= time.delta_seconds();
        }

        if keys.pressed(KeyCode::Space) && ball.cooldown <= 0.0 {
            let current_direction = if velocity.linvel.length() > 0.0 {
                Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z).normalize()
            } else {
                camera_forward_flat
            };
            
            let charge_magnitude = 200.0; 
            velocity.linvel = current_direction * charge_magnitude;

            ball.cooldown = ball.max_cooldown;
        }
    }
}