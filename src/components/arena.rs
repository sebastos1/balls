use crate::*;

pub fn init(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let arena_length = 9. * 5.;
    let arena_width = arena_length / 2.;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(arena_length, arena_width)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    commands.spawn((
        Collider::cuboid(arena_length / 2., 0.1, arena_width / 2.),
    )).insert(Ground).insert(
        TransformBundle::from(
            Transform::from_xyz(0.0, -0.1, 0.0)
        )
    );

    let wall_thickness = 0.5;
    let wall_height = 2.5;
    let base_of_wall = wall_height / 2.0;

    let wall_positions = [
        (Vec3::new(0., base_of_wall, arena_width / 2. + wall_thickness / 2.), Quat::IDENTITY), // North wall
        (Vec3::new(0., base_of_wall, -(arena_width / 2. + wall_thickness / 2.)), Quat::IDENTITY), // South wall
        (Vec3::new(arena_length / 2. + wall_thickness / 2., base_of_wall, 0.), Quat::from_rotation_y(90.0_f32.to_radians())), // East wall
        (Vec3::new(-(arena_length / 2. + wall_thickness / 2.), base_of_wall, 0.), Quat::from_rotation_y(90.0_f32.to_radians())), // West wall
    ];

    let wall_colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
    ];

    for (i, &(position, rotation)) in wall_positions.iter().enumerate() {
        let wall_length = if i < 2 { arena_length } else { arena_width };
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_length, wall_height, wall_thickness))),
            material: materials.add(StandardMaterial {
                base_color: wall_colors[i].into(),
                ..Default::default()
            }),
            transform: Transform {
                translation: position,
                rotation,
                scale: Vec3::ONE,
            },
            ..Default::default()
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
                base_color: Color::rgba(121./255., 118./255., 82./255., 0.8).into(),
                ..Default::default()
            }),
            transform: Transform::from_translation(*direction * -skybox_distance),
            ..Default::default()
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