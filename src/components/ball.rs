use crate::*;
use bevy::{
    render::{
        texture::ImageSampler,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub fn init(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let striped = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(striped_texture(Color::RED))),
        ..default()
    });
    
    spawn_ball(
        &mut commands,
        &mut meshes,
        materials.add(Color::WHITE),
        Vec3::new(0., 4., 0.),
        1,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        striped.clone(),
        Vec3::new(5., 4., -8.),
        0,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        materials.add(Color::GREEN),
        Vec3::new(5., 4., 7.),
        0,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        materials.add(Color::YELLOW),
        Vec3::new(-6., 4., -9.),
        0,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        materials.add(Color::BLUE),
        Vec3::new(-6., 4., 8.),
        0,
    );
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    player_number: u8,
) {
    let radius = 2.25 / 10. * 5. / 2.;

    let mut ball = commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(radius).mesh().uv(16, 16)),
            material,
            transform: Transform::from_translation(position),
            ..default()
        });
    
    ball.insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Friction::coefficient(1.))
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(Restitution::coefficient(0.7))
        .insert(Damping { linear_damping: 0.9, angular_damping: 0.9, });

    if player_number != 0 {
        ball.insert(Ball { player: player_number, cooldown: 0., grounded: false });
        ball.insert(ActiveEvents::COLLISION_EVENTS);
    }
}



fn striped_texture(stripe_color: Color) -> Image {
    const TEXTURE_SIZE: usize = 16;
    const STRIPE_THICKNESS: usize = 8;

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let index = (y * TEXTURE_SIZE + x) * 4;

            let is_stripe = y >= TEXTURE_SIZE / 2 - STRIPE_THICKNESS / 2
                && y < TEXTURE_SIZE / 2 + STRIPE_THICKNESS / 2;

            let color = if is_stripe {
                stripe_color
            } else {
                Color::WHITE
            };

            texture_data[index] = (color.r() * 255.0) as u8;
            texture_data[index + 1] = (color.g() * 255.0) as u8;
            texture_data[index + 2] = (color.b() * 255.0) as u8;
            texture_data[index + 3] = 255;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}
