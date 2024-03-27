use crate::*;
use crate::components::textures::grainy_texture;
use crate::components::textures::noisy_texture;

pub const ARENA_LENGTH: f32 = 9. * 5.; // 9 ft
pub const ARENA_WIDTH: f32 = ARENA_LENGTH / 2.; // 4.5 ft

pub fn init(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // let rail_height = 1.5 / 12. * 5.; // 1.5 inches
    // let rail_thickness = rail_height * 2.;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(ARENA_LENGTH, ARENA_WIDTH)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(images.add(grainy_texture(Color::GREEN, 256, 512))),
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    commands.spawn((
        Collider::cuboid(ARENA_LENGTH / 2., 0.1, ARENA_WIDTH / 2.),
    )).insert(Ground).insert(
        TransformBundle::from(
            Transform::from_xyz(0.0, -0.1, 0.0)
        )
    );

    let wall_thickness = 0.5;
    let wall_height = 2.5;
    let base_of_wall = wall_height / 2.0;

    let wall_positions = [
        (Vec3::new(0., base_of_wall, ARENA_WIDTH / 2. + wall_thickness / 2.), Quat::IDENTITY), // North wall
        (Vec3::new(0., base_of_wall, -(ARENA_WIDTH / 2. + wall_thickness / 2.)), Quat::IDENTITY), // South wall
        (Vec3::new(ARENA_LENGTH / 2. + wall_thickness / 2., base_of_wall, 0.), Quat::from_rotation_y(90.0_f32.to_radians())), // East wall
        (Vec3::new(-(ARENA_LENGTH / 2. + wall_thickness / 2.), base_of_wall, 0.), Quat::from_rotation_y(90.0_f32.to_radians())), // West wall
    ];

    let wall_colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
    ];

    for (i, &(position, rotation)) in wall_positions.iter().enumerate() {
        let wall_length = if i < 2 { ARENA_LENGTH } else { ARENA_WIDTH };
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_length, wall_height, wall_thickness))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(images.add(noisy_texture(wall_colors[i].into()))),
                ..default()
            }),
            transform: Transform {
                translation: position,
                rotation,
                scale: Vec3::ONE,
            },
            ..default()
        })
        .insert(Collider::cuboid(wall_length / 2., wall_height / 2., wall_thickness / 2.));
    }


    let skybox_distance = 200.0;
    let directions = [Vec3::X,-Vec3::X,Vec3::Y,-Vec3::Y,Vec3::Z,-Vec3::Z,];
    for direction in directions.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Plane3d::new(*direction).mesh().size(skybox_distance * 2., skybox_distance * 2.)),
            material: materials.add(StandardMaterial {
                unlit: true,
                base_color_texture: Some(images.add(grainy_texture(Color::rgba(61./255., 59./255., 41./255., 0.8).into(), 512, 512))),
                ..default()
            }),
            transform: Transform::from_translation(*direction * -skybox_distance),
            ..default()
        });
    }

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
}