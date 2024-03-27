use balls::*;
use bevy::prelude::*;
use bevy_ggrs::GgrsApp;
use bevy_ggrs::ReadInputs;
use bevy_ggrs::GgrsSchedule;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .insert_resource(GlobalCharge { charge: 0. })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            bevy_ggrs::GgrsPlugin::<multiplayer::Config>::default(),
        ))
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_clone::<Velocity>()
        .rollback_component_with_clone::<GlobalTransform>()
        .add_systems(Startup, (setup_world, multiplayer::start_matchbox_socket))
        .add_systems(Update, (
            multiplayer::wait_for_players.run_if(in_state(GameState::Matchmaking)),
            (
                window::cursor, 
                camera::pan_cam, 
                camera::zoom, 
                collisions::collision,
                ui::update_meters,
            ).run_if(in_state(GameState::InGame)),
        ))
        .add_systems(ReadInputs, controls::read_local_inputs)
        .add_systems(GgrsSchedule, multiplayer::read_inputs)
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    arena::init(&mut commands, &mut meshes, &mut images, &mut materials);
    ball::init(&mut commands, &mut meshes, &mut images, &mut materials);
    ui::init(&mut commands);
    camera::init(&mut commands);
}