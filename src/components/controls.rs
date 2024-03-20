use crate::*;

pub fn move_ball(
    keys: Res<ButtonInput<KeyCode>>,
    mut ball: Query<(&mut Velocity, &Ball)>,
    camera: Query<&Transform, (With<Camera>, Without<Ball>)>,
) {
    let (mut velocity, ball) = ball.single_mut();

    if keys.pressed(KeyCode::KeyQ) {
        info!("taking flight");
        velocity.linvel += Vec3::Y * 0.1;
    }


    if ball.grounded == false {
        let camera_transform = camera.single();

        let camera_forward = Vec3::new(camera_transform.forward().x, 0., camera_transform.forward().z).normalize();
        let camera_right = Vec3::new(camera_transform.right().x, 0., camera_transform.right().z).normalize();

        let mut direction = Vec3::ZERO;
        let speed = 0.1;

        if keys.pressed(KeyCode::KeyW) {
            direction += camera_forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= camera_forward;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction -= camera_right;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += camera_right;
        }

        if direction.length() > 0. {
            direction = direction.normalize() * speed;
        }

        velocity.linvel += direction;
    }
}

pub fn ability(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Ball, &mut Velocity)>,
    camera_query: Query<&Transform, (With<Camera>, Without<Ball>)>,
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

            ball.cooldown = 1.0;
        }
    }
}