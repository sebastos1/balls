use bevy::{
    prelude::*,
    input::mouse::{MouseMotion, MouseWheel},
    window::{CursorGrabMode, PrimaryWindow},
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_rapier3d::prelude::*;

#[allow(dead_code)]
#[derive(Component)]
struct Ball {
    player: u8,
}

#[derive(Component)]
struct View {
    yaw: f32,
    pitch: f32,
    zoom: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, cursor)
        .add_systems(Update, move_ball)
        .add_systems(Update, (pan_cam, zoom))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100., 100.)),
            material: materials.add(Color::WHITE),
            ..default()
        })
        .insert(Collider::cuboid(100., 0.1, 100.))
        .insert(TransformBundle::from(Transform::from_xyz(0., -2., 0.)));

    spawn_ball(
        &mut commands,
        &mut meshes,
        debug_material.clone(),
        Vec3::new(0., 4., 0.),
        1,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED),
        Vec3::new(15., 4., -20.),
        0,
    );

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10., 3., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection { ..default() }.into(),
        ..Default::default()
    }).insert(View { zoom: -10., pitch: 0., yaw: 0. });
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    player_number: u8,
) {
    let mut ball = commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::default().mesh().uv(32, 18)),
            material,
            transform: Transform::from_translation(position),
            ..default()
        });
    
    ball.insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Friction::coefficient(1.))
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(Restitution::coefficient(0.7))
        .insert(Damping { linear_damping: 0.9, angular_damping: 0.9, });

    if player_number != 0 {
        ball.insert(Ball { player: player_number });
    }
}

fn move_ball(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Ball>>,
    camera: Query<&Transform, (With<Camera>, Without<Ball>)>,
) {
    let camera_transform = camera.single();

    let camera_forward = Vec3::new(camera_transform.forward().x, 0., camera_transform.forward().z).normalize();
    let camera_right = Vec3::new(camera_transform.right().x, 0., camera_transform.right().z).normalize();

    let mut velocity = query.single_mut();

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

fn pan_cam(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cameras: Query<(&mut Transform, &mut View), With<Camera>>,
    ball: Query<&Transform, (With<Ball>, Without<Camera>)>,
) {
    let ball_transform = ball.single();
    for (mut camera_transform, mut view) in cameras.iter_mut() {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            delta += event.delta;
        }

        let sensitivity = Vec2::new(0.3, 0.3);

        view.yaw += delta.x * sensitivity.x * time.delta_seconds();
        view.pitch += -delta.y * sensitivity.y * time.delta_seconds();
        view.pitch = view.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);

        let radius = view.zoom;
        let x = radius * view.yaw.cos() * view.pitch.cos();
        let y = radius * view.pitch.sin();
        let z = radius * view.yaw.sin() * view.pitch.cos();

        camera_transform.translation = ball_transform.translation + Vec3::new(x, y, z);
        camera_transform.look_at(ball_transform.translation, Vec3::Y);
    }
}

fn zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cameras: Query<&mut View, With<Camera>>,
) {
    let mut view = cameras.single_mut();

    let sensitivity = 0.01;
    let min = 5.;
    let max = 15.;

    for event in mouse_wheel_events.read() {
        view.zoom -= event.y * sensitivity;
        view.zoom = view.zoom.clamp(min, max);
    }
}

fn cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) && primary_window.cursor.grab_mode == CursorGrabMode::None {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}