use crate::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

pub fn init(
    commands: &mut Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10., 3., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection { ..default() }.into(),
        ..Default::default()
    }).insert(View { zoom: 10., pitch: 0., yaw: 0. });
}

pub fn pan_cam(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cameras: Query<(&mut Transform, &mut View), With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let ball_transform = player.single();
    for (mut camera_transform, mut view) in cameras.iter_mut() {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        let sensitivity = Vec2::new(0.3, 0.3);

        view.yaw += delta.x * sensitivity.x * time.delta_seconds();
        view.pitch += delta.y * sensitivity.y * time.delta_seconds();
        view.pitch = view.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);

        let radius = view.zoom;
        let x = radius * view.yaw.cos() * view.pitch.cos();
        let y = radius * view.pitch.sin();
        let z = radius * view.yaw.sin() * view.pitch.cos();

        camera_transform.translation = ball_transform.translation + Vec3::new(x, y, z);
        let focus_point = ball_transform.translation + Vec3::new(0., 2., 0.);
        camera_transform.look_at(focus_point, Vec3::Y);
    }
}

pub fn zoom(
    mut cameras: Query<&mut View, With<Camera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for mut view in cameras.iter_mut() {
        let sensitivity = 0.01;
        let min = 5.;
        let max = 15.;

        for event in mouse_wheel_events.read() {
            view.zoom -= event.y * sensitivity;
            view.zoom = view.zoom.clamp(min, max);
        }
    }
}