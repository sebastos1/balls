use balls::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, window::cursor)
        .add_systems(Update, (controls::move_ball, controls::ability))
        .add_systems(Update, (camera::pan_cam, camera::zoom))
        .add_systems(Update, collisions::collision)
        .add_systems(Update, ui::update_meters)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    arena::init(&mut commands, &mut meshes, &mut materials);
    ball::init(&mut commands, &mut meshes, &mut images, &mut materials);
    ui::init(&mut commands);
    camera::init(&mut commands);  
}